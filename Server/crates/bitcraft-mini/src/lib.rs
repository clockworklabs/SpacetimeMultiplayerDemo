use spacetimedb_bindgen::spacetimedb;
use spacetimedb_bindings::hash::Hash;

#[spacetimedb(table(1))]
pub struct Player {
    #[unique]
    pub entity_id: u32,
    #[unique]
    pub owner_id: Hash,
    pub creation_time: u64,
}

#[spacetimedb(table(2))]
pub struct EntityTransform {
    #[unique]
    pub entity_id: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
}

#[spacetimedb(table(3))]
pub struct PlayerAnimation {
    #[unique]
    pub entity_id: u32,
    pub moving: bool,
}

#[spacetimedb(table(4))]
pub struct EntityInventory {
    #[unique]
    pub entity_id: u32,
    pub pockets: Vec<Pocket>,
}

impl EntityInventory {
    fn get_pocket(&self, pocket_idx: u32) -> Option<Pocket> {
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket_idx && self.pockets[x].item_count > 0 {
                return Some(self.pockets[x]);
            }
        }

        return None;
    }

    fn set_pocket(&mut self, pocket: Pocket) {
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

    fn delete_pocket(&mut self, pocket_idx: u32) {
        // Try to find the pocket in the inventory
        for x in 0..self.pockets.len() {
            if self.pockets[x].pocket_idx == pocket_idx {
                self.pockets.remove(x);
                return;
            }
        }
    }
}

#[derive(Copy, Clone)]
#[spacetimedb(tuple)]
pub struct Pocket {
    pub item_id: u32,
    pub pocket_idx: u32,
    pub item_count: i32,
}

#[derive(Copy, Clone)]
#[spacetimedb(table(5))]
pub struct Config {
    #[unique]
    pub version: u32,
    // always 0 for now
    pub max_player_inventory_slots: u32,
}

#[spacetimedb(table(6))]
pub struct PlayerChatMessage {
    pub player_id: u32,
    pub msg_time: u64,
    pub message: String,
}

// This is in charge of initializing any static global data
#[spacetimedb(reducer)]
pub fn initialize(_identity: Hash, _timestamp: u64) {
    match Config::filter_version_eq(0) {
        Some(_) => {
            spacetimedb_bindings::println!("Config already exists, skipping config.");
        }
        None => {
            spacetimedb_bindings::println!("Creating new config!");
            Config::insert(Config {
                version: 0,
                max_player_inventory_slots: 30,
            });
        }
    }
}

#[spacetimedb(reducer)]
pub fn move_or_swap_inventory_slot(
    identity: Hash,
    _timestamp: u64,
    entity_id: u32,
    source_pocket_idx: u32,
    dest_pocket_idx: u32,
) {
    let config = Config::filter_version_eq(0).unwrap();

    // Check to see if the source pocket index is bad
    if source_pocket_idx >= config.max_player_inventory_slots {
        panic!(
            "move_or_swap_inventory_slot: The source pocket index is invalid: {}",
            source_pocket_idx
        );
    }

    // Check to see if the dest pocket index is bad
    if dest_pocket_idx >= config.max_player_inventory_slots {
        panic!(
            "move_or_swap_inventory_slot: The dest pocket index is invalid: {}",
            dest_pocket_idx
        );
    }

    // Make sure this identity owns this player
    let player = Player::filter_entity_id_eq(entity_id)
        .expect("move_or_swap_inventory_slot: This player doesn't exist!");
    if player.owner_id != identity {
        // TODO: We are doing this for now so that its easier to test reducers from the command line
        spacetimedb_bindings::println!(
            "move_or_swap_inventory_slot: This identity doesn't own this player! (allowed for now)"
        );
        // return;
    }

    let mut inventory = EntityInventory::filter_entity_id_eq(entity_id)
        .expect("move_or_swap_inventory_slot: This player doesn't have an inventory!");
    let mut source_pocket = inventory
        .get_pocket(source_pocket_idx)
        .expect("move_or_swap_inventory_slot: Nothing in source pocket, nothing to do.");
    let dest_pocket = inventory.get_pocket(dest_pocket_idx);

    // If we don't have a dest pocket, then just do a direct move
    if let None = dest_pocket {
        inventory.delete_pocket(source_pocket_idx);
        source_pocket.pocket_idx = dest_pocket_idx;
        inventory.set_pocket(source_pocket);
        EntityInventory::update_entity_id_eq(entity_id, inventory);
        spacetimedb_bindings::println!(
            "move_or_swap_inventory_slot: Source pocket moved to dest pocket."
        );
        return;
    }

    // If we have a dest and source pocket then we have to see if we can stack onto the dest
    let mut dest_pocket = dest_pocket.unwrap();
    if source_pocket.item_id == dest_pocket.item_id {
        // Move source items to dest
        dest_pocket.item_count += source_pocket.item_count;
        inventory.delete_pocket(source_pocket_idx);
        inventory.set_pocket(dest_pocket);
        EntityInventory::update_entity_id_eq(entity_id, inventory);
        spacetimedb_bindings::println!(
            "move_or_swap_inventory_slot: Source pocket moved into dest pocket (same item)"
        );
        return;
    }

    inventory.delete_pocket(source_pocket_idx);
    inventory.delete_pocket(dest_pocket_idx);
    dest_pocket.pocket_idx = source_pocket_idx;
    source_pocket.pocket_idx = dest_pocket_idx;
    inventory.set_pocket(source_pocket);
    inventory.set_pocket(dest_pocket);
    EntityInventory::update_entity_id_eq(entity_id, inventory);
    spacetimedb_bindings::println!(
        "move_or_swap_inventory_slot: Pockets swapped (different items)"
    );
}

/// This adds or removes items from an inventory slot. you can pass a negative item count in order
/// to remove items.
#[spacetimedb(reducer)]
pub fn add_item_to_inventory(
    identity: Hash,
    _timestamp: u64,
    entity_id: u32,
    item_id: u32,
    pocket_idx: u32,
    item_count: i32,
) {
    // Check to see if this pocket index is bad
    let config = Config::filter_version_eq(0).unwrap();
    assert!(
        pocket_idx < config.max_player_inventory_slots,
        "add_item_to_inventory: This pocket index is invalid: {}",
        pocket_idx
    );

    // Make sure this identity owns this player
    let player = Player::filter_entity_id_eq(entity_id)
        .expect("add_item_to_inventory: This player doesn't exist!");
    if player.owner_id != identity {
        // TODO: We are doing this for now so that its easier to test reducers from the command line
        spacetimedb_bindings::println!(
            "add_item_to_inventory: This identity doesn't own this player! (allowed for now)"
        );
        // return;
    }

    let mut inventory = EntityInventory::filter_entity_id_eq(entity_id)
        .expect("add_item_to_inventory: This player doesn't have an inventory!");
    match inventory.get_pocket(pocket_idx) {
        Some(mut pocket) => {
            assert_eq!(
                pocket.item_id, item_id,
                "add_item_to_inventory: Item ID mismatch"
            );
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

    EntityInventory::update_entity_id_eq(entity_id, inventory);
    spacetimedb_bindings::println!(
        "add_item_to_inventory: Item {} inserted into inventory {}",
        item_id,
        entity_id
    );
}

#[spacetimedb(reducer)]
pub fn dump_inventory(_identity: Hash, _timestamp: u64, entity_id: u32) {
    let inventory = EntityInventory::filter_entity_id_eq(entity_id);
    assert!(
        inventory.is_some(),
        "Inventory NOT found for entity:: {}",
        entity_id
    );
    let inventory = inventory.unwrap();

    spacetimedb_bindings::println!("Inventory found for entity: {}", entity_id);
    for pocket in inventory.pockets {
        spacetimedb_bindings::println!(
            "PocketIdx: {} Item: {} Count: {}",
            pocket.pocket_idx,
            pocket.item_id,
            pocket.item_count
        );
    }
}

#[spacetimedb(reducer)]
pub fn move_player(
    identity: Hash,
    _timestamp: u64,
    entity_id: u32,
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
    rot_w: f32,
) {
    // Make sure this identity owns this player
    match Player::filter_entity_id_eq(entity_id) {
        Some(player) => {
            if player.owner_id != identity {
                spacetimedb_bindings::println!(
                    "move_player: This identity doesn't own this player! (allowed for now)"
                );
                // return;
            }
        }
        None => {
            spacetimedb_bindings::println!("move_player: This player doesn't exist: {}", entity_id);
            return;
        }
    }

    EntityTransform::update_entity_id_eq(
        entity_id,
        EntityTransform {
            entity_id,
            pos_x,
            pos_y,
            pos_z,
            rot_x,
            rot_y,
            rot_z,
            rot_w,
        },
    );
}

#[spacetimedb(reducer)]
pub fn update_player_animation(identity: Hash, _timestamp: u64, entity_id: u32, moving: bool) {
    // Make sure this identity owns this player
    match Player::filter_entity_id_eq(entity_id) {
        Some(player) => {
            if player.owner_id != identity {
                spacetimedb_bindings::println!("update_player_animation: This identity doesn't own this player! (allowed for now)");
                // return;
            }
        }
        None => {
            spacetimedb_bindings::println!("update_player_animation: This player doesn't exist!");
            return;
        }
    }

    PlayerAnimation::update_entity_id_eq(entity_id, PlayerAnimation { entity_id, moving });
}

#[spacetimedb(reducer)]
pub fn create_new_player(identity: Hash, timestamp: u64, entity_id: u32) {
    // Make sure this player doesn't already exist
    if let Some(_) = Player::filter_entity_id_eq(entity_id) {
        spacetimedb_bindings::println!(
            "create_new_player: A player with this entity_id already exists: {}",
            entity_id
        );
        return;
    }

    spacetimedb_bindings::println!("create_new_player: player created: {}", entity_id);
    Player::insert(Player {
        entity_id,
        owner_id: identity,
        creation_time: timestamp,
    });
    EntityInventory::insert(EntityInventory {
        entity_id,
        pockets: Vec::<Pocket>::new(),
    });
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

#[spacetimedb(connect)]
pub fn identity_connected(_identity: Hash, _timestamp: u64) {}

#[spacetimedb(disconnect)]
pub fn identity_disconnected(_identity: Hash, _timestamp: u64) {}
