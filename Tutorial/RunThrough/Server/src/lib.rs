use log;
use rand::Rng;
use spacetimedb::{spacetimedb, Identity, SpacetimeType};
use spacetimedb::{ReducerContext, Timestamp};

#[spacetimedb(table)]
pub struct Config {
    #[primarykey]
    // always 0 for now
    pub version: u32,

    pub message_of_the_day: String,

    // PART2
    pub map_extents: u32,
    pub num_resource_nodes: u32,
}

#[spacetimedb(table)]
pub struct SpawnableEntityComponent {
    #[primarykey]
    #[autoinc]
    pub entity_id: u64,
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct PlayerComponent {
    #[primarykey]
    pub entity_id: u64,
    #[unique]
    pub owner_id: Identity,

    pub username: String,
    pub logged_in: bool,
}

#[derive(SpacetimeType, Clone)]
pub struct StdbVector2 {
    pub x: f32,
    pub z: f32,
}

impl StdbVector2 {
    pub const ZERO: StdbVector2 = StdbVector2 { x: 0.0, z: 0.0 };
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct MobileEntityComponent {
    #[primarykey]
    pub entity_id: u64,

    pub location: StdbVector2,
    pub direction: StdbVector2,

    pub move_start_timestamp: Timestamp,
}

// PART2
#[derive(spacetimedb::SpacetimeType, Clone)]
pub enum ResourceNodeType {
    Iron,
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct ResourceNodeComponent {
    #[primarykey]
    pub entity_id: u64,

    pub resource_type: ResourceNodeType,
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct StaticLocationComponent {
    #[primarykey]
    pub entity_id: u64,

    pub location: StdbVector2,
    pub rotation: f32,
}

#[spacetimedb(init)]
pub fn init() {
    Config::insert(Config {
        version: 0,
        message_of_the_day: "Hello, World!".to_string(),

        // PART2
        map_extents: 25,
        num_resource_nodes: 10,
    })
    .expect("Failed to insert config.");

    // PART2
    spacetimedb::schedule!("5000ms", resource_spawner_agent(_, Timestamp::now()));
}

#[spacetimedb(connect)]
pub fn identity_connected(ctx: ReducerContext) {
    update_player_login_state(ctx, true);
}

#[spacetimedb(disconnect)]
pub fn identity_disconnected(ctx: ReducerContext) {
    update_player_login_state(ctx, false);
}

pub fn update_player_login_state(ctx: ReducerContext, logged_in: bool) {
    if let Some(player) = PlayerComponent::filter_by_owner_id(&ctx.sender) {
        let entity_id = player.entity_id;
        let mut player = player.clone();
        player.logged_in = logged_in;
        PlayerComponent::update_by_entity_id(&entity_id, player);
    }
}

#[spacetimedb(reducer)]
pub fn create_player(ctx: ReducerContext, username: String) -> Result<(), String> {
    let owner_id = ctx.sender;
    if PlayerComponent::filter_by_owner_id(&owner_id).is_some() {
        log::info!("Player already exists");
        return Err("Player already exists".to_string());
    }

    let entity_id = SpawnableEntityComponent::insert(SpawnableEntityComponent { entity_id: 0 })
        .expect("Failed to create player spawnable entity component.")
        .entity_id;

    PlayerComponent::insert(PlayerComponent {
        entity_id,
        owner_id,
        username: username.clone(),
        logged_in: true,
    })
    .expect("Failed to insert player component.");

    MobileEntityComponent::insert(MobileEntityComponent {
        entity_id,
        location: StdbVector2::ZERO,
        direction: StdbVector2::ZERO,
        move_start_timestamp: Timestamp::UNIX_EPOCH,
    })
    .expect("Failed to insert player mobile entity component.");

    log::info!("Player created: {}({})", username, entity_id);

    Ok(())
}

#[spacetimedb(reducer)]
pub fn move_player(
    ctx: ReducerContext,
    start: StdbVector2,
    direction: StdbVector2,
) -> Result<(), String> {
    let owner_id = ctx.sender;
    if let Some(player) = PlayerComponent::filter_by_owner_id(&owner_id) {
        if let Some(mut mobile) = MobileEntityComponent::filter_by_entity_id(&player.entity_id) {
            mobile.location = start;
            mobile.direction = direction;
            mobile.move_start_timestamp = ctx.timestamp;
            MobileEntityComponent::update_by_entity_id(&player.entity_id, mobile);

            return Ok(());
        }
    }

    return Err("Player not found".to_string());
}

#[spacetimedb(reducer)]
pub fn stop_player(ctx: ReducerContext, location: StdbVector2) -> Result<(), String> {
    let owner_id = ctx.sender;
    if let Some(player) = PlayerComponent::filter_by_owner_id(&owner_id) {
        if let Some(mut mobile) = MobileEntityComponent::filter_by_entity_id(&player.entity_id) {
            mobile.location = location;
            mobile.direction = StdbVector2::ZERO;
            mobile.move_start_timestamp = Timestamp::UNIX_EPOCH;
            MobileEntityComponent::update_by_entity_id(&player.entity_id, mobile);

            return Ok(());
        }
    }

    return Err("Player not found".to_string());
}

// PART2
#[spacetimedb(reducer, repeat = 1000ms)]
pub fn resource_spawner_agent(_ctx: ReducerContext, _prev_time: Timestamp) -> Result<(), String> {
    let config = Config::filter_by_version(&0).unwrap();
    let num_resource_nodes = config.num_resource_nodes as usize;

    let num_resource_nodes_spawned = ResourceNodeComponent::iter().count();
    if num_resource_nodes_spawned >= num_resource_nodes {
        log::info!("All resource nodes spawned. Skipping.");
        return Ok(());
    }

    let mut rng = rand::thread_rng();

    let map_extents = config.map_extents as f32;

    let location = StdbVector2 {
        x: rng.gen_range(-map_extents..map_extents),
        z: rng.gen_range(-map_extents..map_extents),
    };

    let rotation = rng.gen_range(0.0..360.0);

    let entity_id = SpawnableEntityComponent::insert(SpawnableEntityComponent { entity_id: 0 })
        .expect("Failed to create resource spawnable entity component.")
        .entity_id;

    StaticLocationComponent::insert(StaticLocationComponent {
        entity_id,
        location: location.clone(),
        rotation,
    })
    .expect("Failed to insert resource static location component.");

    ResourceNodeComponent::insert(ResourceNodeComponent {
        entity_id,
        resource_type: ResourceNodeType::Iron,
    })
    .expect("Failed to insert resource node component.");

    log::info!(
        "Resource node spawned: {} at ({}, {})",
        entity_id,
        location.x,
        location.z,
    );

    Ok(())
}

#[spacetimedb(reducer)]
pub fn message_of_the_day(_ctx: ReducerContext, new_message: String) {
    let mut config = Config::filter_by_version(&0).unwrap();
    config.message_of_the_day = new_message;
    Config::update_by_version(&0, config);
}
