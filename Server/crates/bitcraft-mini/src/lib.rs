use spacetimedb_bindgen::spacetimedb;
use spacetimedb_bindings::hash::Hash;

#[spacetimedb(tuple)]
pub struct Position {
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
}

#[spacetimedb(table(1))]
pub struct Player {
    #[primary_key]
    pub player_id: u32,
    // pub sender: Hash,
    pub position: Position,
}

#[spacetimedb(reducer)]
pub fn move_player(_sender: Hash, _timestamp: u64, player_id: u32, x: f32, y: f32, z: f32) {
    let player = Player::filter_player_id_eq(player_id);
    match player {
        Some(mut player) => {
            player.position.pos_x = x;
            player.position.pos_y = y;
            player.position.pos_z = z;

            // Update player position
            Player::update_player_id_eq(player_id, player);
        },
        None => {
            // Player not found!
        }
    }
}

#[spacetimedb(reducer)]
pub fn create_new_player(_sender: Hash, _timestamp: u64, player_id: u32, position: Position) {
    let player = Player {
        player_id,
        position,
    };

    Player::insert(player);
}