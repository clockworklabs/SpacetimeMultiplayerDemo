use spacetimedb::{spacetimedb, Identity, SpacetimeType, Timestamp};

#[spacetimedb(table)]
#[derive(Clone)]
pub struct Config {
    // Config is a global table with a single row. This table will be used to
    // store configuration or global variables
    #[primarykey]
    // always 0
    // having a table with a primarykey field which is always zero is a way to store singleton global state
    pub version: u32,

    pub message_of_the_day: String,

    // new variables for resource node spawner
    // X and Z range of the map (-map_extents to map_extents)
    pub map_extents: u32,
    // maximum number of resource nodes to spawn on the map
    pub num_resource_nodes: u32,

    // Maximum amount of pockets the player can hold
    pub max_player_inventory_slots: u32,
}

#[spacetimedb(table)]
pub struct SpawnableEntityComponent {
    // All entities that can be spawned in the world will have this component.
    // This allows us to find all objects in the world by iterating through
    // this table. It also ensures that all world objects have a unique
    // entity_id.
    #[primarykey]
    #[autoinc]
    pub entity_id: u64,
}

#[derive(Clone)]
#[spacetimedb(table)]
pub struct PlayerComponent {
    // All players have this component and it associates the spawnable entity
    // with the user's identity. It also stores their username.
    #[primarykey]
    pub entity_id: u64,
    #[unique]
    pub owner_id: Identity,

    // username is provided to the create_player reducer
    pub username: String,
    // this value is updated when the user logs in and out
    pub logged_in: bool,
}

#[derive(SpacetimeType, Clone)]
pub enum ResourceNodeType {
    Iron,
}

#[spacetimedb(table)]
pub struct ResourceNodeComponent {
    #[primarykey]
    pub entity_id: u64,
    pub health: i32,
    pub resource_type: ResourceNodeType,
    pub max_health: u8,
    pub item_yield_id: u8,
    pub item_yield_quantity: u8,
}

#[derive(SpacetimeType, Clone)]
pub struct StdbVector2 {
    // A spacetime type which can be used in tables and reducers to represent
    // a 2d position.
    pub x: f32,
    pub z: f32,
}

impl StdbVector2 {
    // this allows us to use StdbVector2::ZERO in reducers
    pub const ZERO: StdbVector2 = StdbVector2 { x: 0.0, z: 0.0 };
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct MobileEntityComponent {
    // This component will be created for all world objects that can move
    // smoothly throughout the world. It keeps track of the position the last
    // time the component was updated and the direction the mobile object is
    // currently moving.
    #[primarykey]
    pub entity_id: u64,

    // The last known location of this entity
    pub location: StdbVector2,
    // Movement direction, {0,0} if not moving at all.
    pub direction: StdbVector2,
    // Timestamp when movement started. Timestamp::UNIX_EPOCH if not moving.
    pub move_start_timestamp: Timestamp,
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct StaticLocationComponent {
    #[primarykey]
    pub entity_id: u64,

    pub location: StdbVector2,
    pub rotation: f32,
}

#[spacetimedb(table)]
pub struct ChatMessage {
    // The primary key for this table will be auto-incremented
    #[primarykey]
    #[autoinc]
    pub chat_entity_id: u64,

    // The entity id of the player (or NPC) that sent the message
    pub source_entity_id: u64,
    // Message contents
    pub chat_text: String,
    // Timestamp of when the message was sent
    pub timestamp: Timestamp,
}

#[derive(Copy, Clone, Debug, SpacetimeType)]
pub struct Pocket {
    pub item_id: u32,
    pub pocket_idx: u32,
    pub item_count: i32,
}

#[spacetimedb(table)]
pub struct AnimationComponent {
    #[primarykey]
    pub entity_id: u64,
    pub moving: bool,
    pub jump_start_timestamp: Timestamp,
    pub action_target_entity_id: u64,
}

#[spacetimedb(table)]
#[derive(Debug, Clone)]
pub struct InventoryComponent {
    #[primarykey]
    pub entity_id: u64,
    pub pockets: Vec<Pocket>,
}
