use spacetimedb::spacetimedb;

#[spacetimedb(table)]
pub struct PlayerLoginComponent {
    #[unique]
    pub entity_id: u64,
    pub logged_in: bool,
}
