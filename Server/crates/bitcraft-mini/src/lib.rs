use spacetimedb_bindgen::spacetimedb;
use spacetimedb_bindings::hash::Hash;

#[spacetimedb(tuple)]
pub struct Position {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
}

#[spacetimedb(tuple)]
pub struct Rotation {
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub rot_w: f32,
}

#[spacetimedb(table(1))]
pub struct Player {
    pub owner_id: Hash,
    #[primary_key]
    pub player_id: u32,
    pub creation_time: u64,
    pub position: Position,
    pub rotation: Rotation,
    pub moving: bool,
}

#[spacetimedb(reducer)]
pub fn move_player(_identity_id: Hash, _timestamp: u64, player_id: u32, position: Position, rotation: Rotation, moving: bool) {
    let player = Player::filter_player_id_eq(player_id);
    match player {
        Some(mut player) => {
            player.position = position;
            player.rotation = rotation;
            player.moving = moving;

            // Update player position
            Player::update_player_id_eq(player_id, player);
        },
        None => {
            // Player not found!
        }
    }
}

#[spacetimedb(reducer)]
pub fn create_new_player(identity: Hash, timestamp: u64, player_id: u32, position: Position, rotation: Rotation) {
    let player = Player {
        owner_id: identity,
        player_id,
        creation_time: timestamp,
        position,
        rotation,
        moving: false
    };

    Player::insert(player);
}