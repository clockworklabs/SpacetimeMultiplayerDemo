use spacetimedb_bindgen::spacetimedb;
use spacetimedb_bindings::Hash;

#[spacetimedb(table)]
pub struct PlayerComponent {
    #[unique]
    pub entity_id: u32,
    #[unique]
    pub owner_id: Hash,
    #[unique]
    pub username: String,
    pub creation_time: u64,
}