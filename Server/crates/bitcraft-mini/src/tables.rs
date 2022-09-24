use spacetimedb::spacetimedb;

#[derive(Copy, Clone)]
#[spacetimedb(table)]
pub struct Config {
    #[unique]
    // always 0 for now
    pub version: u32,

    // Maximum amount of pockets the player can hold
    pub max_player_inventory_slots: u32,
    // Terrain points in each direction per chunk
    pub chunk_terrain_resolution: u32,
    // Image resolution of the splats
    pub chunk_splat_resolution: u32,
    // In game size of the terrain chunk
    pub chunk_size: f64,
    pub terrain_seed: u32,
    // The amount of entities that can be placed on the terrain in each direction
    pub entity_density: u32,
    // The minimum range from a player or npc a npc can spawn
    pub min_spawn_range: f32,
    // The maximum range from a player a npc can spawn or remain spawned
    pub max_spawn_range: f32,
    // Range at which the npc will start reacting to the player
    pub npc_detection_range: f32,
}

#[spacetimedb(table)]
pub struct PlayerChatMessage {
    pub player_id: u32,
    pub msg_time: u64,
    pub message: String,
}
