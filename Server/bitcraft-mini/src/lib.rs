use spacetimedb_bindgen::spacetimedb;

#[spacetimedb(table)]
pub struct Entity {
    id: u32,
    player_num: i32,
}

#[spacetimedb(table)]
pub struct Position {
    entity_id: u32,
    // x: f32,
    // y: f32,
    // z: f32,
}

#[spacetimedb(table)]
pub struct Player {
    entity_id: u32,
    // username: String,
}

#[spacetimedb(reducer)]
pub fn create_player(username: String, position: Position) {
    // let mut rng = rand::thread_rng();
    // let id: u32 = rng.gen::<u32>();
    // Entity::insert(id);
}
