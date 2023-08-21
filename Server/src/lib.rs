use log;
use rand::Rng;
use spacetimedb::{spacetimedb, ReducerContext, Timestamp};

pub mod tables;
use tables::*;

impl InventoryComponent {
    pub fn get_pocket(&self, pocket_idx: u32) -> Option<Pocket> {
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket_idx && self.pockets[x].item_count > 0 {
                return Some(self.pockets[x]);
            }
        }

        None
    }

    pub fn set_pocket(&mut self, pocket: Pocket) {
        // Try to find the pocket in the inventory
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket.pocket_idx {
                self.pockets[x] = pocket;
                return;
            }
        }

        // We did not find this pocket, create a new pocket
        self.pockets.push(pocket);
    }

    pub fn delete_pocket(&mut self, pocket_idx: u32) {
        // Try to find the pocket in the inventory
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket_idx {
                self.pockets.remove(x);
                return;
            }
        }
    }

    pub fn add(&mut self, item_id: u32, item_count: i32, index: Option<u32>) -> bool {
        // Check to see if this pocket index is bad
        let config = Config::filter_by_version(&0).unwrap();

        // Change empty pocket index for the first EMPTY pocket index
        let pocket_idx = if let Some(idx) = index {
            idx
        } else {
            let mut idx = u32::MAX;
            for i in 0..config.max_player_inventory_slots {
                if self.get_pocket(i).is_none() {
                    idx = i;
                    break;
                }
            }
            if idx >= config.max_player_inventory_slots {
                return false;
            }
            idx
        };

        if pocket_idx >= config.max_player_inventory_slots {
            return false;
        }

        let pocket = match self.get_pocket(pocket_idx) {
            Some(mut pocket) => {
                assert_eq!(pocket.item_id, item_id, "Item ID mismatch");
                if pocket.item_count + item_count < 0 {
                    // removed more than what's available
                    return false;
                }
                pocket.item_count += item_count;
                pocket
            }
            None => Pocket {
                pocket_idx,
                item_id,
                item_count,
            },
        };

        if pocket.item_count == 0 {
            self.delete_pocket(pocket.pocket_idx);
        } else {
            self.set_pocket(pocket);
        }
        true
    }

    pub fn can_hold(&self, items: &Vec<(u32, i32)>) -> bool {
        let mut copy = self.clone();
        let mut success = true;
        for &(item_id, item_count) in items {
            success &= copy.add(item_id, item_count, None);
        }
        success
    }

    pub fn combine(&mut self, other: &InventoryComponent) -> bool {
        let other_items: Vec<(u32, i32)> = other
            .pockets
            .iter()
            .map(|p| (p.item_id, p.item_count))
            .collect();
        if !self.can_hold(&other_items) {
            return false;
        }
        for (item_id, item_count) in other_items {
            self.add(item_id, item_count, None);
        }
        true
    }
}

#[spacetimedb(init)]
pub fn init() {
    // Called when the module is initially published

    // Create our global config table.
    Config::insert(Config {
        version: 0,
        message_of_the_day: "Hello, World!".to_string(),

        // new variables for resource node spawner
        map_extents: 25,
        num_resource_nodes: 10,

        max_player_inventory_slots: 30,
    })
    .expect("Failed to insert config.");

    // Start our resource spawner repeating reducer
    spacetimedb::schedule!("1000ms", resource_spawner_agent(_, Timestamp::now()));
}

#[spacetimedb(connect)]
pub fn identity_connected(ctx: ReducerContext) {
    // called when the client connects, we update the logged_in state to true
    update_player_login_state(ctx, true);
}

#[spacetimedb(disconnect)]
pub fn identity_disconnected(ctx: ReducerContext) {
    // Called when the client disconnects, we update the logged_in state to false
    update_player_login_state(ctx, false);
}

pub fn update_player_login_state(ctx: ReducerContext, logged_in: bool) {
    // This helper function gets the PlayerComponent, sets the logged
    // in variable and updates the SpacetimeDB table row.
    if let Some(player) = PlayerComponent::filter_by_owner_id(&ctx.sender) {
        let entity_id = player.entity_id;
        // We clone the PlayerComponent so we can edit it and pass it back.
        let mut player = player.clone();
        player.logged_in = logged_in;
        PlayerComponent::update_by_entity_id(&entity_id, player);
    }
}

#[spacetimedb(reducer)]
pub fn create_player(ctx: ReducerContext, username: String) -> Result<(), String> {
    // This reducer is called when the user logs in for the first time and
    // enters a username

    let owner_id = ctx.sender;
    // We check to see if there is already a PlayerComponent with this identity.
    // this should never happen because the client only calls it if no player
    // is found.
    if PlayerComponent::filter_by_owner_id(&owner_id).is_some() {
        log::info!("Player already exists");
        return Err("Player already exists".to_string());
    }

    // Next we create the SpawnableEntityComponent. The entity_id for this
    // component automatically increments and we get it back from the result
    // of the insert call and use it for all components.

    let entity_id = SpawnableEntityComponent::insert(SpawnableEntityComponent { entity_id: 0 })
        .expect("Failed to create player spawnable entity component.")
        .entity_id;
    // The PlayerComponent uses the same entity_id and stores the identity of
    // the owner, username, and whether or not they are logged in.
    PlayerComponent::insert(PlayerComponent {
        entity_id,
        owner_id,
        username: username.clone(),
        logged_in: true,
    })
    .expect("Failed to insert player component.");
    // The MobileEntityComponent is used to calculate the current position
    // of an entity that can move smoothly in the world. We are using 2d
    // positions and the client will use the terrain height for the y value.
    MobileEntityComponent::insert(MobileEntityComponent {
        entity_id,
        location: StdbVector2::ZERO,
        direction: StdbVector2::ZERO,
        move_start_timestamp: Timestamp::UNIX_EPOCH,
    })
    .expect("Failed to insert player mobile entity component.");

    AnimationComponent::insert(AnimationComponent {
        entity_id,
        moving: false,
        action_target_entity_id: 0,
    })
    .expect("Failed to insert player animation component.");

    // The InventoryComponent is used to store the player's inventory. We
    // initialize it with an empty vector of pockets.
    InventoryComponent::insert(InventoryComponent {
        entity_id,
        pockets: Vec::<Pocket>::new(),
    })
    .expect("Failed to insert player inventory component.");

    log::info!("Player created: {}({})", username, entity_id);

    Ok(())
}

#[spacetimedb(reducer)]
pub fn move_player(
    ctx: ReducerContext,
    start: StdbVector2,
    direction: StdbVector2,
) -> Result<(), String> {
    // Update the MobileEntityState component with the current movement
    // values. The client will call this regularly as the direction of movement
    // changes. A fully developed game should validate these moves on the server
    // before committing them, but that is beyond the scope of this tutorial.

    let owner_id = ctx.sender;
    // First, look up the player using the sender identity, then use that
    // entity_id to retrieve and update the MobileEntityComponent
    if let Some(player) = PlayerComponent::filter_by_owner_id(&owner_id) {
        if let Some(mut mobile) = MobileEntityComponent::filter_by_entity_id(&player.entity_id) {
            mobile.location = start;
            mobile.direction = direction;
            mobile.move_start_timestamp = ctx.timestamp;
            MobileEntityComponent::update_by_entity_id(&player.entity_id, mobile);

            return Ok(());
        }
    }

    // If we can not find the PlayerComponent for this user something went wrong.
    // This should never happen.
    return Err("Player not found".to_string());
}

#[spacetimedb(reducer)]
pub fn stop_player(ctx: ReducerContext, location: StdbVector2) -> Result<(), String> {
    // Update the MobileEntityComponent when a player comes to a stop. We set
    // the location to the current location and the direction to {0,0}
    let owner_id = ctx.sender;
    if let Some(player) = PlayerComponent::filter_by_owner_id(&owner_id) {
        if let Some(mut mobile) = MobileEntityComponent::filter_by_entity_id(&player.entity_id) {
            mobile.location = location;
            mobile.direction = StdbVector2::ZERO;
            mobile.move_start_timestamp = Timestamp::UNIX_EPOCH;
            MobileEntityComponent::update_by_entity_id(&player.entity_id, mobile);

            return Ok(());
        }
    }

    return Err("Player not found".to_string());
}

#[spacetimedb(reducer)]
pub fn chat_message(ctx: ReducerContext, message: String) -> Result<(), String> {
    // Add a chat entry to the ChatMessage table

    // Get the player component based on the sender identity
    let owner_id = ctx.sender;
    if let Some(player) = PlayerComponent::filter_by_owner_id(&owner_id) {
        // Now that we have the player we can insert the chat message using the player entity id.
        ChatMessage::insert(ChatMessage {
            // this column auto-increments so we can set it to 0
            chat_entity_id: 0,
            source_entity_id: player.entity_id,
            chat_text: message,
            timestamp: ctx.timestamp,
        })
        .unwrap();

        return Ok(());
    }

    Err("Player not found".into())
}

#[spacetimedb(reducer, repeat = 1000ms)]
pub fn resource_spawner_agent(_ctx: ReducerContext, _prev_time: Timestamp) -> Result<(), String> {
    let config = Config::filter_by_version(&0).unwrap();

    // Retrieve the maximum number of nodes we want to spawn from the Config table
    let num_resource_nodes = config.num_resource_nodes as usize;

    // Count the number of nodes currently spawned and exit if we have reached num_resource_nodes
    let num_resource_nodes_spawned = ResourceNodeComponent::iter().count();
    if num_resource_nodes_spawned >= num_resource_nodes {
        //log::info!("All resource nodes spawned. Skipping.");
        return Ok(());
    }

    // Pick a random X and Z based off the map_extents
    let mut rng = rand::thread_rng();
    let map_extents = config.map_extents as f32;
    let location = StdbVector2 {
        x: rng.gen_range(-map_extents..map_extents),
        z: rng.gen_range(-map_extents..map_extents),
    };
    // Pick a random Y rotation in degrees
    let rotation = rng.gen_range(0.0..360.0);

    // Insert our SpawnableEntityComponent which assigns us our entity_id
    let entity_id = SpawnableEntityComponent::insert(SpawnableEntityComponent { entity_id: 0 })
        .expect("Failed to create resource spawnable entity component.")
        .entity_id;

    // Insert our static location with the random position and rotation we selected
    StaticLocationComponent::insert(StaticLocationComponent {
        entity_id,
        location: location.clone(),
        rotation,
    })
    .expect("Failed to insert resource static location component.");

    // Insert our resource node component, so far we only have iron
    ResourceNodeComponent::insert(ResourceNodeComponent {
        entity_id,
        health: 5,
        max_health: 5,
        item_yield_id: 0,
        item_yield_quantity: 1,
        resource_type: ResourceNodeType::Iron,
    })
    .expect("Failed to insert resource node component.");

    // Log that we spawned a node with the entity_id and location
    log::info!(
        "Resource node spawned: {} at ({}, {})",
        entity_id,
        location.x,
        location.z,
    );

    Ok(())
}

#[spacetimedb(reducer)]
pub fn move_or_swap_inventory_slot(
    ctx: ReducerContext,
    player_entity_id: u64,
    inventory_entity_id: u64,
    source_pocket_idx: u32,
    dest_pocket_idx: u32,
) -> Result<(), String> {
    let config = Config::filter_by_version(&0).expect("Config exists.");

    // Check to see if the source pocket index is bad
    if source_pocket_idx >= config.max_player_inventory_slots {
        return Err(format!(
            "The source pocket index is invalid: {}",
            source_pocket_idx
        ));
    }

    // Check to see if the dest pocket index is bad
    if dest_pocket_idx >= config.max_player_inventory_slots {
        return Err(format!(
            "The dest pocket index is invalid: {}",
            dest_pocket_idx
        ));
    }

    if source_pocket_idx == dest_pocket_idx {
        // Cannot drag and drop on itself
        return Ok(());
    }

    // Make sure this identity owns this player
    let player = PlayerComponent::filter_by_entity_id(&player_entity_id)
        .expect("This player doesn't exist!");
    if player.owner_id != ctx.sender {
        // TODO: We are doing this for now so that its easier to test reducers from the command line
        return Err(format!(
            "This identity doesn't own this player! (allowed for now)"
        ));
    }

    let mut inventory = InventoryComponent::filter_by_entity_id(&inventory_entity_id)
        .expect("This inventory doesn't exist!");

    let mut source_pocket = inventory
        .get_pocket(source_pocket_idx)
        .expect("Nothing in source pocket, nothing to do.");

    let dest_pocket = inventory.get_pocket(dest_pocket_idx);

    // If we don't have a dest pocket, then just do a direct move
    if dest_pocket.is_none() {
        inventory.delete_pocket(source_pocket_idx);
        source_pocket.pocket_idx = dest_pocket_idx;
        inventory.set_pocket(source_pocket);
        InventoryComponent::update_by_entity_id(&inventory_entity_id, inventory);
        log::info!("Source pocket moved to dest pocket.");

        return Ok(());
    }

    // If we have a dest and source pocket then we have to see if we can stack onto the dest
    let mut dest_pocket = dest_pocket.unwrap();
    if source_pocket.item_id == dest_pocket.item_id {
        // Move source items to dest
        dest_pocket.item_count += source_pocket.item_count;
        inventory.delete_pocket(source_pocket_idx);
        inventory.set_pocket(dest_pocket);
        InventoryComponent::update_by_entity_id(&inventory_entity_id, inventory);
        log::info!("Source pocket moved into dest pocket (same item)");

        return Ok(());
    }

    inventory.delete_pocket(source_pocket_idx);
    inventory.delete_pocket(dest_pocket_idx);
    dest_pocket.pocket_idx = source_pocket_idx;
    source_pocket.pocket_idx = dest_pocket_idx;
    inventory.set_pocket(source_pocket);
    inventory.set_pocket(dest_pocket);
    InventoryComponent::update_by_entity_id(&inventory_entity_id, inventory);
    log::info!("Pockets swapped (different items)");

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
    let player = PlayerComponent::filter_by_entity_id(&entity_id)
        .expect("add_item_to_inventory: This player doesn't exist!");

    if player.owner_id != ctx.sender {
        // TODO: We are doing this for now so that its easier to test reducers from the command line
        log::info!("This identity doesn't own this player! (allowed for now)");
        // return;
    }

    let mut inventory = InventoryComponent::filter_by_entity_id(&entity_id)
        .expect("This player doesn't have an inventory!");

    if !inventory.add(
        item_id,
        item_count,
        if pocket_idx < 0 {
            None
        } else {
            Some(pocket_idx as u32)
        },
    ) {
        panic!("Failed to add items to inventory");
    }

    InventoryComponent::update_by_entity_id(&entity_id, inventory);
    log::info!("Item {} inserted into inventory {}", item_id, entity_id);

    Ok(())
}

#[spacetimedb(reducer)]
pub fn dump_inventory(_ctx: ReducerContext, entity_id: u64) -> Result<(), String> {
    let inventory = InventoryComponent::filter_by_entity_id(&entity_id)
        .unwrap_or_else(|| panic!("Inventory NOT found for entity {}", entity_id));

    for pocket in inventory.pockets {
        log::info!(
            "PocketIdx: {} Item: {} Count: {}",
            pocket.pocket_idx,
            pocket.item_id,
            pocket.item_count
        );
    }

    Ok(())
}

#[spacetimedb(reducer)]
pub fn extract(ctx: ReducerContext, entity_id: u64, resource_entity_id: u64) -> Result<(), String> {
    let player =
        PlayerComponent::filter_by_entity_id(&entity_id).expect("This player doesn't exist.");

    // Make sure this identity owns this player
    if player.owner_id != ctx.sender {
        log::info!("This identity doesn't own this player! (allowed for now)");
    }

    // ToDo: validate resource distance from player. For now resource position is determined by the chunk so we can't.

    let mut resource = ResourceNodeComponent::filter_by_entity_id(&resource_entity_id)
        .expect("This resource doesn't exist");

    // Attempt to add resources to the player's inventory
    add_item_to_inventory(
        ctx,
        entity_id,
        resource.item_yield_id.into(),
        -1,
        resource.item_yield_quantity.into(),
    )?;

    resource.health -= 1;

    if resource.health <= 0 {
        ResourceNodeComponent::delete_by_entity_id(&resource_entity_id);
    } else {
        log::info!("Resource health: {}", resource.health);
        ResourceNodeComponent::update_by_entity_id(&resource_entity_id, resource);
    }

    Ok(())
}

#[spacetimedb(reducer)]
pub fn update_animation(
    ctx: ReducerContext,
    entity_id: u64,
    moving: bool,
    action_target_entity_id: u64,
) -> Result<(), String> {
    let player =
        PlayerComponent::filter_by_entity_id(&entity_id).expect("This player doesn't exist!");

    // Make sure this identity owns this player
    if player.owner_id != ctx.sender {
        log::info!("This identity doesn't own this player! (allowed for now)");
    }

    AnimationComponent::update_by_entity_id(
        &entity_id,
        AnimationComponent {
            entity_id,
            moving,
            action_target_entity_id,
        },
    );

    Ok(())
}
