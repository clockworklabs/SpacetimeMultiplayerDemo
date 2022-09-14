use spacetimedb_bindgen::spacetimedb;

#[spacetimedb(table)]
pub struct PlayerLoginComponent {
    #[unique]
    pub entity_id: u32,
    pub logged_in: bool,
}