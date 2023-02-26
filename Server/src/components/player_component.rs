use spacetimedb::spacetimedb;
use spacetimedb::Identity;

#[spacetimedb(table)]
pub struct PlayerComponent {
    #[unique]
    pub entity_id: u64,
    #[unique]
    pub owner_id: Identity,
    #[unique]
    pub username: String,
    pub creation_time: u64,
}
