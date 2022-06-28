use spacetimedb_bindgen::spacetimedb;

#[spacetimedb(tuple)]
pub struct Pocket {
    pub item_id: u32,
    pub item_count: u32,
}

#[spacetimedb(tuple)]
pub struct Inventory {
    pub pocket_count: u32,
    pub pocket: Pocket,
}

#[spacetimedb(table(1))]
pub struct Player {
    #[primary_key]
    pub player_id: u32,
    pub inventory: Inventory,
    pub inventory2: Inventory,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
}

#[spacetimedb(reducer)]
#[allow(unused)]
pub fn move_player(player_id: u32, x: f32, y: f32, z: f32) {
    let player = Player::filter_player_id_eq(player_id);
    match player {
        Some(mut player) => {
            player.pos_x = x;
            player.pos_y = y;
            player.pos_z = z;

            // Update player position
            // Player::update_player_id_eq(player_id, player);
        },
        None => {
            // Player not found!
        }
    }
}