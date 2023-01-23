use spacetimedb::spacetimedb;

#[spacetimedb(table)]
pub struct NpcComponent {
    #[unique]
    pub entity_id: u64,
    pub model: String,
    pub next_action: u64,
}
