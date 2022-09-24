use spacetimedb_bindgen::spacetimedb;

#[spacetimedb(table)]
pub struct ResourceComponent {
    #[unique]
    pub entity_id: u32,
    pub health: u8,
    pub resource_id: u8,
    pub max_health: u8, // todo: ideally we would find that static data from a table using resource_id
    pub item_yield_id: u8,
    pub item_yield_quantity: u8,
}
