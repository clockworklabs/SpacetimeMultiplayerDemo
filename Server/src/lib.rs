mod components;
mod helpers;
mod math;
mod npcs;
mod tables;
mod trade_session;
mod tuples;

use crate::components::chunk_component::generate_terrain_stub;
use crate::npcs::{despawn_npcs, move_npcs, spawn_npcs};
use crate::tables::{Config, PlayerChatMessage, ServerGlobals};
use crate::terrain_generation::check_chunks_for_all_players;
use crate::trade_session::cancel_trade_session_with_participant;
use crate::tuples::Pocket;
use components::{
    ActiveTradeComponent, AnimationComponent, InventoryComponent, PlayerComponent, PlayerLoginComponent,
    ResourceComponent, TradeSessionComponent, TransformComponent,
};
use math::{StdbQuaternion, StdbVector3};

use spacetimedb::{println, ReducerContext};
use spacetimedb::{spacetimedb, Timestamp};

mod random;
mod terrain_generation;

// This is in charge of initializing any static global data
#[spacetimedb(init)]
pub fn init() {
    // TODO(cloutiertyler): Validate that the identity is the authorized
    // identity. (i.e. the one who initialized this database)

    let config = Config::filter_by_version(0);
    if config.is_some() {
        println!("Config already exists, skipping config.");
        return;
    }
    println!("Creating new config!");
    Config::insert(Config {
        version: 0,
        max_player_inventory_slots: 30,
        trading_slots: 18,
        chunk_terrain_resolution: 16,
        chunk_splat_resolution: 128,
        chunk_size: 10.0,
        entity_density: 16,
        terrain_seed: 78648326,
        min_spawn_range: 32.0,
        max_spawn_range: 48.0,
        npc_detection_range: 20.0,
    });

    ServerGlobals::insert(ServerGlobals {
        version: 0,
        entity_id_counter: 0,
    });

    // This one terrain chunk is inserted so the client can identify the world state message. The issue we have right now
    // is that other players or agents can update tables, therefore there is now way to be sure that the first subscription
    // received by the client is the world state.
    // TODO: the client should be able to subscribe on demand and the server should make sure no subscription is received until then.
    generate_terrain_stub();

    spacetimedb::schedule!("1000ms", check_chunks_for_all_players(_, Timestamp::now()));
    spacetimedb::schedule!("5000ms", spawn_npcs(_, Timestamp::now()));
    spacetimedb::schedule!("15000ms", despawn_npcs(_, Timestamp::now()));
    spacetimedb::schedule!("100ms", move_npcs(_, Timestamp::now()));
}

#[spacetimedb(reducer)]
pub fn move_or_swap_inventory_slot(
    ctx: ReducerContext,
    player_entity_id: u64,
    inventory_entity_id: u64,
    source_pocket_idx: u32,
    dest_pocket_idx: u32,
) -> Result<(), String> {
    let config = Config::filter_by_version(0).expect("Config exists.");

    // Check to see if the source pocket index is bad
    if source_pocket_idx >= config.max_player_inventory_slots {
        return Err(format!("The source pocket index is invalid: {}", source_pocket_idx));
    }

    // Check to see if the dest pocket index is bad
    if dest_pocket_idx >= config.max_player_inventory_slots {
        return Err(format!("The dest pocket index is invalid: {}", dest_pocket_idx));
    }

    if source_pocket_idx == dest_pocket_idx {
        // Cannot drag and drop on itself
        return Ok(());
    }

    // Make sure this identity owns this player
    let player = PlayerComponent::filter_by_entity_id(player_entity_id).expect("This player doesn't exist!");
    if player.owner_id != ctx.sender {
        // TODO: We are doing this for now so that its easier to test reducers from the command line
        return Err(format!("This identity doesn't own this player! (allowed for now)"));
    }

    if inventory_entity_id != player_entity_id {
        // Make sure the player is allowed to modify this inventory
        let mut valid = false;

        // Is it part of a trade involving the player?
        if let Some(active_trade) = ActiveTradeComponent::filter_by_entity_id(player_entity_id) {
            if let Some(session) = TradeSessionComponent::filter_by_entity_id(active_trade.trade_session_entity_id) {
                valid |= session.initiator_entity_id == player_entity_id;
                valid |= session.acceptor_entity_id == player_entity_id;
            }
        }

        // ToDo: external storages, etc.

        // We did all the checks for external inventory update.
        if !valid {
            return Err(format!("This player is not allowed to modify that inventory"));
        }
    }

    let mut inventory =
        InventoryComponent::filter_by_entity_id(inventory_entity_id).expect("This inventory doesn't exist!");

    let mut source_pocket = inventory
        .get_pocket(source_pocket_idx)
        .expect("Nothing in source pocket, nothing to do.");

    let dest_pocket = inventory.get_pocket(dest_pocket_idx);

    // If we don't have a dest pocket, then just do a direct move
    if dest_pocket.is_none() {
        inventory.delete_pocket(source_pocket_idx);
        source_pocket.pocket_idx = dest_pocket_idx;
        inventory.set_pocket(source_pocket);
        InventoryComponent::update_by_entity_id(inventory_entity_id, inventory);
        println!("Source pocket moved to dest pocket.");

        return Ok(());
    }

    // If we have a dest and source pocket then we have to see if we can stack onto the dest
    let mut dest_pocket = dest_pocket.unwrap();
    if source_pocket.item_id == dest_pocket.item_id {
        // Move source items to dest
        dest_pocket.item_count += source_pocket.item_count;
        inventory.delete_pocket(source_pocket_idx);
        inventory.set_pocket(dest_pocket);
        InventoryComponent::update_by_entity_id(inventory_entity_id, inventory);
        println!("Source pocket moved into dest pocket (same item)");

        return Ok(());
    }

    inventory.delete_pocket(source_pocket_idx);
    inventory.delete_pocket(dest_pocket_idx);
    dest_pocket.pocket_idx = source_pocket_idx;
    source_pocket.pocket_idx = dest_pocket_idx;
    inventory.set_pocket(source_pocket);
    inventory.set_pocket(dest_pocket);
    InventoryComponent::update_by_entity_id(inventory_entity_id, inventory);
    println!("Pockets swapped (different items)");

    Ok(())
}

/// This adds or removes items from an inventory slot. you can pass a negative item count in order
/// to remove items.
#[spacetimedb(reducer)]
pub fn add_item_to_inventory(
    ctx: ReducerContext,
    entity_id: u64,
    item_id: u32,
    pocket_idx: i32, // < 0 to auto assign the first valid index
    item_count: i32,
) -> Result<(), String> {
    // Make sure this identity owns this player
    let player =
        PlayerComponent::filter_by_entity_id(entity_id).expect("add_item_to_inventory: This player doesn't exist!");

    if player.owner_id != ctx.sender {
        // TODO: We are doing this for now so that its easier to test reducers from the command line
        println!("This identity doesn't own this player! (allowed for now)");
        // return;
    }

    let mut inventory =
        InventoryComponent::filter_by_entity_id(entity_id).expect("This player doesn't have an inventory!");

    if !inventory.add(
        item_id,
        item_count,
        if pocket_idx < 0 { None } else { Some(pocket_idx as u32) },
    ) {
        panic!("Failed to add items to inventory");
    }

    InventoryComponent::update_by_entity_id(entity_id, inventory);
    println!("Item {} inserted into inventory {}", item_id, entity_id);

    Ok(())
}

#[spacetimedb(reducer)]
pub fn dump_inventory(_ctx: ReducerContext, entity_id: u64) -> Result<(), String> {
    let inventory = InventoryComponent::filter_by_entity_id(entity_id)
        .unwrap_or_else(|| panic!("Inventory NOT found for entity {}", entity_id));

    for pocket in inventory.pockets {
        println!(
            "PocketIdx: {} Item: {} Count: {}",
            pocket.pocket_idx, pocket.item_id, pocket.item_count
        );
    }

    Ok(())
}

#[spacetimedb(reducer)]
pub fn move_player(ctx: ReducerContext, entity_id: u64, pos: StdbVector3, rot: StdbQuaternion) -> Result<(), String> {
    let player = PlayerComponent::filter_by_entity_id(entity_id).expect("This player doesn't exist.");

    // Make sure this identity owns this player
    if player.owner_id != ctx.sender {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    TransformComponent::update_by_entity_id(entity_id, TransformComponent { entity_id, pos, rot });

    Ok(())
}

#[spacetimedb(reducer)]
pub fn update_animation(
    ctx: ReducerContext,
    entity_id: u64,
    moving: bool,
    action_target_entity_id: u64,
) -> Result<(), String> {
    let player = PlayerComponent::filter_by_entity_id(entity_id).expect("This player doesn't exist!");

    // Make sure this identity owns this player
    if player.owner_id != ctx.sender {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    AnimationComponent::update_by_entity_id(
        entity_id,
        AnimationComponent {
            entity_id,
            moving,
            action_target_entity_id,
        },
    );

    Ok(())
}

#[spacetimedb(reducer)]
pub fn create_new_player(
    ctx: ReducerContext,
    start_pos: StdbVector3,
    start_rot: StdbQuaternion,
    username: String,
) -> Result<(), String> {
    let entity_id = helpers::next_entity_id();

    println!("Creating player with this ID: {}", entity_id);
    let creation_time = ctx
        .timestamp
        .duration_since(Timestamp::UNIX_EPOCH)
        .ok()
        .unwrap()
        .as_millis() as u64;

    PlayerComponent::insert(PlayerComponent {
        entity_id,
        owner_id: ctx.sender,
        username,
        creation_time,
    });
    InventoryComponent::insert(InventoryComponent {
        entity_id,
        pockets: Vec::<Pocket>::new(),
    });
    TransformComponent::insert(TransformComponent {
        entity_id,
        pos: start_pos,
        rot: start_rot,
    });
    println!(
        "We have to make sure this entity has a chunk to stand on: {}",
        entity_id
    );
    println!("Player created: {}", entity_id);

    Ok(())
}

#[spacetimedb(reducer)]
pub fn player_chat(ctx: ReducerContext, player_id: u64, message: String) -> Result<(), String> {
    let msg_time = ctx
        .timestamp
        .duration_since(Timestamp::UNIX_EPOCH)
        .ok()
        .unwrap()
        .as_millis() as u64;

    let chat = PlayerChatMessage {
        player_id,
        msg_time,
        message,
    };

    PlayerChatMessage::insert(chat);

    Ok(())
}

#[spacetimedb(reducer)]
pub fn player_update_login_state(ctx: ReducerContext, logged_in: bool) -> Result<(), String> {
    let player = PlayerComponent::filter_by_owner_id(ctx.sender).expect("You cannot sign in without a player!");

    if let Some(login_state) = PlayerLoginComponent::filter_by_entity_id(player.entity_id) {
        assert!(
            login_state.logged_in != logged_in,
            "Player is already set to this login state: {}",
            logged_in
        );
        let player_entity_id = player.entity_id;

        if !logged_in {
            cancel_trade_session_with_participant(player_entity_id);
        }

        PlayerLoginComponent::update_by_entity_id(
            player_entity_id,
            PlayerLoginComponent {
                entity_id: player_entity_id,
                logged_in,
            },
        );

        return Ok(());
    }

    println!("Player set login state to: {}", logged_in);
    PlayerLoginComponent::insert(PlayerLoginComponent {
        entity_id: player.entity_id,
        logged_in,
    });

    Ok(())
}

#[spacetimedb(connect)]
pub fn identity_connected(ctx: ReducerContext) {
    let player = PlayerComponent::filter_by_owner_id(ctx.sender);
    if let Some(player) = player {
        println!("Player {} has returned.", player.entity_id);
    } else {
        println!("A new identity has connected.");
    }
}

#[spacetimedb(disconnect)]
pub fn identity_disconnected(ctx: ReducerContext) {
    if let Some(player) = PlayerComponent::filter_by_owner_id(ctx.sender) {
        if let Some(login_state) = PlayerLoginComponent::filter_by_entity_id(player.entity_id) {
            if login_state.logged_in {
                println!("User has disconnected without signing out.");
                let player_entity_id = player.entity_id;

                cancel_trade_session_with_participant(player_entity_id);

                PlayerLoginComponent::update_by_entity_id(
                    player_entity_id,
                    PlayerLoginComponent {
                        entity_id: player_entity_id,
                        logged_in: false,
                    },
                );
            }
        }
    }
}

#[spacetimedb(reducer)]
pub fn extract(ctx: ReducerContext, entity_id: u64, resource_entity_id: u64) -> Result<(), String> {
    let player = PlayerComponent::filter_by_entity_id(entity_id).expect("This player doesn't exist.");

    // Make sure this identity owns this player
    if player.owner_id != ctx.sender {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    // ToDo: validate resource distance from player. For now resource position is determined by the chunk so we can't.

    let mut resource = ResourceComponent::filter_by_entity_id(resource_entity_id).expect("This resource doesn't exist");

    // Attempt to add resources to the player's inventory
    add_item_to_inventory(
        ctx,
        entity_id,
        resource.item_yield_id.into(),
        -1,
        resource.item_yield_quantity.into(),
    )?;

    resource.health -= 1;

    //TODO: Is this a bug? `health` is unsigned so this cmp is non-sense.
    if resource.health <= 0 {
        ResourceComponent::delete_by_entity_id(resource_entity_id);
    } else {
        ResourceComponent::update_by_entity_id(resource_entity_id, resource);
    }

    Ok(())
}
