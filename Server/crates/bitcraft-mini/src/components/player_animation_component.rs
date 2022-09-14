use spacetimedb_bindgen::spacetimedb;

#[spacetimedb(table)]
pub struct PlayerAnimationComponent {
    #[unique]
    pub entity_id: u32,
    pub moving: bool,
}