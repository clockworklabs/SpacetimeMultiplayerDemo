mod components;
mod math;
mod npcs;
mod tables;
mod tuples;

use crate::tables::{Config, PlayerChatMessage};
use crate::tuples::Pocket;
use components::{
    AnimationComponent, InventoryComponent, PlayerComponent, PlayerLoginComponent, ResourceComponent,
    TransformComponent,
};
use math::{StdbQuaternion, StdbVector3};
use spacetimedb::spacetimedb;
use spacetimedb::Hash;
use spacetimedb::println;

mod random;
mod terrain_generation;

// This is in charge of initializing any static global data
#[spacetimedb(reducer)]
pub fn initialize(_identity: Hash, _timestamp: u64) {
    // TODO(cloutiertyler): Validate that the identity is the authorized
    // identity. (i.e. the one who initialized this database)

    let config = Config::filter_version_eq(0);
    if config.is_some() {
        println!("Config already exists, skipping config.");
        return;
    }
    println!("Creating new config!");
    Config::insert(Config {
        version: 0,
        max_player_inventory_slots: 30,
        chunk_terrain_resolution: 16,
        chunk_splat_resolution: 128,
        chunk_size: 10.0,
        entity_density: 16,
        terrain_seed: 78648326,
        min_spawn_range: 32.0,
        max_spawn_range: 48.0,
        npc_detection_range: 20.0,
    });
}

#[spacetimedb(reducer)]
pub fn move_or_swap_inventory_slot(
    identity: Hash,
    _timestamp: u64,
    entity_id: u32,
    source_pocket_idx: u32,
    dest_pocket_idx: u32,
) {
    let config = Config::filter_version_eq(0).expect("Config exists.");

    // Check to see if the source pocket index is bad
    if source_pocket_idx >= config.max_player_inventory_slots {
        panic!("The source pocket index is invalid: {}", source_pocket_idx);
    }

    // Check to see if the dest pocket index is bad
    if dest_pocket_idx >= config.max_player_inventory_slots {
        panic!("The dest pocket index is invalid: {}", dest_pocket_idx);
    }

    // Make sure this identity owns this player
    let player = PlayerComponent::filter_entity_id_eq(entity_id).expect("This player doesn't exist!");
    if player.owner_id != identity {
        // TODO: We are doing this for now so that its easier to test reducers from the command line
        panic!("This identity doesn't own this player! (allowed for now)");
    }

    let mut inventory =
        InventoryComponent::filter_entity_id_eq(entity_id).expect("This player doesn't have an inventory!");

    let mut source_pocket = inventory
        .get_pocket(source_pocket_idx)
        .expect("Nothing in source pocket, nothing to do.");

    let dest_pocket = inventory.get_pocket(dest_pocket_idx);

    // If we don't have a dest pocket, then just do a direct move
    if let None = dest_pocket {
        inventory.delete_pocket(source_pocket_idx);
        source_pocket.pocket_idx = dest_pocket_idx;
        inventory.set_pocket(source_pocket);
        InventoryComponent::update_entity_id_eq(entity_id, inventory);
        println!("Source pocket moved to dest pocket.");
        return;
    }

    // If we have a dest and source pocket then we have to see if we can stack onto the dest
    let mut dest_pocket = dest_pocket.unwrap();
    if source_pocket.item_id == dest_pocket.item_id {
        // Move source items to dest
        dest_pocket.item_count += source_pocket.item_count;
        inventory.delete_pocket(source_pocket_idx);
        inventory.set_pocket(dest_pocket);
        InventoryComponent::update_entity_id_eq(entity_id, inventory);
        println!("Source pocket moved into dest pocket (same item)");
        return;
    }

    inventory.delete_pocket(source_pocket_idx);
    inventory.delete_pocket(dest_pocket_idx);
    dest_pocket.pocket_idx = source_pocket_idx;
    source_pocket.pocket_idx = dest_pocket_idx;
    inventory.set_pocket(source_pocket);
    inventory.set_pocket(dest_pocket);
    InventoryComponent::update_entity_id_eq(entity_id, inventory);
    println!("Pockets swapped (different items)");
}

/// This adds or removes items from an inventory slot. you can pass a negative item count in order
/// to remove items.
#[spacetimedb(reducer)]
pub fn add_item_to_inventory(
    identity: Hash,
    _timestamp: u64,
    entity_id: u32,
    item_id: u32,
    pocket_idx: i32, // < 0 to auto assign the first valid index
    item_count: i32,
) {
    // Make sure this identity owns this player
    let player =
        PlayerComponent::filter_entity_id_eq(entity_id).expect("add_item_to_inventory: This player doesn't exist!");
    if player.owner_id != identity {
        // TODO: We are doing this for now so that its easier to test reducers from the command line
        println!("This identity doesn't own this player! (allowed for now)");
        // return;
    }

    let mut inventory =
        InventoryComponent::filter_entity_id_eq(entity_id).expect("This player doesn't have an inventory!");

    // Check to see if this pocket index is bad
    let config = Config::filter_version_eq(0).unwrap();

    // Change negative pocket index for the first valid pocket index
    let pocket_idx = if pocket_idx < 0 {
        let mut idx = u32::MAX;
        for i in 0..config.max_player_inventory_slots {
            if let Some(pocket) = inventory.get_pocket(i) {
                if pocket.item_id == item_id {
                    idx = i;
                    break;
                }
            } else {
                idx = i;
                break;
            }
        }
        if idx >= config.max_player_inventory_slots {
            panic!("No free slot");
        }
        idx
    } else {
        pocket_idx as u32
    };

    assert!(
        pocket_idx < config.max_player_inventory_slots,
        "This pocket index is invalid: {}",
        pocket_idx
    );

    match inventory.get_pocket(pocket_idx) {
        Some(mut pocket) => {
            assert_eq!(pocket.item_id, item_id, "Item ID mismatch");
            pocket.item_count += item_count;
        }
        None => {
            inventory.set_pocket(Pocket {
                pocket_idx,
                item_id,
                item_count,
            });
        }
    }

    InventoryComponent::update_entity_id_eq(entity_id, inventory);
    println!("Item {} inserted into inventory {}", item_id, entity_id);
}

#[spacetimedb(reducer)]
pub fn dump_inventory(_identity: Hash, _timestamp: u64, entity_id: u32) {
    let inventory = InventoryComponent::filter_entity_id_eq(entity_id)
        .expect(&format!("Inventory NOT found for entity {}", entity_id));

    println!("Inventory found for entity: {}", entity_id);
    for pocket in inventory.pockets {
        println!(
            "PocketIdx: {} Item: {} Count: {}",
            pocket.pocket_idx, pocket.item_id, pocket.item_count
        );
    }
}

#[spacetimedb(reducer)]
pub fn move_player(identity: Hash, _timestamp: u64, entity_id: u32, pos: StdbVector3, rot: StdbQuaternion) {
    let player = PlayerComponent::filter_entity_id_eq(entity_id).expect("This player doesn't exist.");

    // Make sure this identity owns this player
    if player.owner_id != identity {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    TransformComponent::update_entity_id_eq(entity_id, TransformComponent { entity_id, pos, rot });
}

#[spacetimedb(reducer)]
pub fn update_animation(identity: Hash, _timestamp: u64, entity_id: u32, moving: bool, action: u32) {
    let player = PlayerComponent::filter_entity_id_eq(entity_id).expect("This player doesn't exist!");

    // Make sure this identity owns this player
    if player.owner_id != identity {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    AnimationComponent::update_entity_id_eq(
        entity_id,
        AnimationComponent {
            entity_id,
            moving,
            action,
        },
    );
}

#[spacetimedb(reducer)]
pub fn create_new_player(
    identity: Hash,
    timestamp: u64,
    entity_id: u32,
    start_pos: StdbVector3,
    start_rot: StdbQuaternion,
    username: String,
) {
    // Make sure this player doesn't already exist
    if PlayerComponent::filter_entity_id_eq(entity_id).is_some() {
        panic!("A player with this entity_id already exists: {}", entity_id);
    }
    println!("Creating player with this ID: {}", entity_id);
    PlayerComponent::insert(PlayerComponent {
        entity_id,
        owner_id: identity,
        username,
        creation_time: timestamp,
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
}

#[spacetimedb(reducer)]
pub fn player_chat(_identity: Hash, timestamp: u64, player_id: u32, message: String) {
    let chat = PlayerChatMessage {
        player_id,
        msg_time: timestamp,
        message,
    };

    PlayerChatMessage::insert(chat);
}

#[spacetimedb(reducer)]
pub fn player_update_login_state(identity: Hash, _timestamp: u64, logged_in: bool) {
    let player = PlayerComponent::filter_owner_id_eq(identity).expect("You cannot sign in without a player!");

    if let Some(login_state) = PlayerLoginComponent::filter_entity_id_eq(player.entity_id) {
        assert!(
            login_state.logged_in != logged_in,
            "Player is already set to this login state: {}",
            logged_in
        );
        PlayerLoginComponent::update_entity_id_eq(
            player.entity_id,
            PlayerLoginComponent {
                entity_id: player.entity_id,
                logged_in,
            },
        );
        return;
    }

    println!("Player set login state to: {}", logged_in);
    PlayerLoginComponent::insert(PlayerLoginComponent {
        entity_id: player.entity_id,
        logged_in,
    });
}

#[spacetimedb(connect)]
pub fn identity_connected(identity: Hash, _timestamp: u64) {
    let player = PlayerComponent::filter_owner_id_eq(identity);
    if let Some(player) = player {
        println!("Player {} has returned.", player.entity_id);
    } else {
        println!("A new identity has connected.");
    }
}

#[spacetimedb(disconnect)]
pub fn identity_disconnected(identity: Hash, _timestamp: u64) {
    if let Some(player) = PlayerComponent::filter_owner_id_eq(identity) {
        if let Some(login_state) = PlayerLoginComponent::filter_entity_id_eq(player.entity_id) {
            if login_state.logged_in {
                println!("User has disconnected without signing out.");
                PlayerLoginComponent::update_entity_id_eq(
                    player.entity_id,
                    PlayerLoginComponent {
                        entity_id: player.entity_id,
                        logged_in: false,
                    },
                );
            }
        }
    }
}

#[spacetimedb(reducer)]
pub fn extract(identity: Hash, timestamp: u64, entity_id: u32, resource_entity_id: u32) {
    let player = PlayerComponent::filter_entity_id_eq(entity_id).expect("This player doesn't exist.");

    // Make sure this identity owns this player
    if player.owner_id != identity {
        println!("This identity doesn't own this player! (allowed for now)");
    }

    // ToDo: validate resource distance from player. For now resource position is determined by the chunk so we can't.

    let mut resource = ResourceComponent::filter_entity_id_eq(resource_entity_id).expect("This resource doesn't exist");

    // Attempt to add resources to the player's inventory

    add_item_to_inventory(
        identity,
        timestamp,
        entity_id,
        resource.item_yield_id.into(),
        -1,
        resource.item_yield_quantity.into(),
    );

    resource.health -= 1;

    if resource.health <= 0 {
        ResourceComponent::delete_entity_id_eq(resource_entity_id);
    } else {
        ResourceComponent::update_entity_id_eq(resource_entity_id, resource);
    }
}
