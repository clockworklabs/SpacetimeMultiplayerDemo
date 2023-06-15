use log;
use rand::Rng;
use spacetimedb::{spacetimedb, Identity, ReducerContext, SpacetimeType, Timestamp};

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

#[derive(SpacetimeType, Clone)]
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
    // Called when the module is initially published

    // Create our global config table.
    Config::insert(Config {
        version: 0,
        message_of_the_day: "Hello, World!".to_string(),

        map_extents: 25,
        num_resource_nodes: 10,
    })
    .expect("Failed to insert config.");

    // Start our resource spawner repeating reducer
    spacetimedb::schedule!("1000ms", resource_spawner_agent(_, Timestamp::now()));
}

#[spacetimedb(connect)]
pub fn identity_connected(ctx: ReducerContext) {
    // called when the client connects, we update the logged_in state to true
    update_player_login_state(ctx, true);
}

#[spacetimedb(disconnect)]
pub fn identity_disconnected(ctx: ReducerContext) {
    // Called when the client disconnects, we update the logged_in state to false
    update_player_login_state(ctx, false);
}

pub fn update_player_login_state(ctx: ReducerContext, logged_in: bool) {
    // This helper function gets the PlayerComponent, sets the logged
    // in variable and updates the SpacetimeDB table row.
    if let Some(player) = PlayerComponent::filter_by_owner_id(&ctx.sender) {
        let entity_id = player.entity_id;
        // We clone the PlayerComponent so we can edit it and pass it back.
        let mut player = player.clone();
        player.logged_in = logged_in;
        PlayerComponent::update_by_entity_id(&entity_id, player);
    }
}

#[spacetimedb(reducer)]
pub fn create_player(ctx: ReducerContext, username: String) -> Result<(), String> {
    // This reducer is called when the user logs in for the first time and
    // enters a username

    let owner_id = ctx.sender;
    // We check to see if there is already a PlayerComponent with this identity.
    // this should never happen because the client only calls it if no player
    // is found.
    if PlayerComponent::filter_by_owner_id(&owner_id).is_some() {
        log::info!("Player already exists");
        return Err("Player already exists".to_string());
    }

    // Next we create the SpawnableEntityComponent. The entity_id for this
    // component automatically increments and we get it back from the result
    // of the insert call and use it for all components.

    let entity_id = SpawnableEntityComponent::insert(SpawnableEntityComponent { entity_id: 0 })
        .expect("Failed to create player spawnable entity component.")
        .entity_id;
    // The PlayerComponent uses the same entity_id and stores the identity of
    // the owner, username, and whether or not they are logged in.
    PlayerComponent::insert(PlayerComponent {
        entity_id,
        owner_id,
        username: username.clone(),
        logged_in: true,
    })
    .expect("Failed to insert player component.");
    // The MobileEntityComponent is used to calculate the current position
    // of an entity that can move smoothly in the world. We are using 2d
    // positions and the client will use the terrain height for the y value.
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
    // Update the MobileEntityState component with the current movement
    // values. The client will call this regularly as the direction of movement
    // changes. A fully developed game should validate these moves on the server
    // before committing them, but that is beyond the scope of this tutorial.

    let owner_id = ctx.sender;
    // First, look up the player using the sender identity, then use that
    // entity_id to retrieve and update the MobileEntityComponent
    if let Some(player) = PlayerComponent::filter_by_owner_id(&owner_id) {
        if let Some(mut mobile) = MobileEntityComponent::filter_by_entity_id(&player.entity_id) {
            mobile.location = start;
            mobile.direction = direction;
            mobile.move_start_timestamp = ctx.timestamp;
            MobileEntityComponent::update_by_entity_id(&player.entity_id, mobile);

            return Ok(());
        }
    }

    // If we can not find the PlayerComponent for this user something went wrong.
    // This should never happen.
    return Err("Player not found".to_string());
}

#[spacetimedb(reducer)]
pub fn stop_player(ctx: ReducerContext, location: StdbVector2) -> Result<(), String> {
    // Update the MobileEntityComponent when a player comes to a stop. We set
    // the location to the current location and the direction to {0,0}
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

#[spacetimedb(reducer, repeat = 1000ms)]
pub fn resource_spawner_agent(_ctx: ReducerContext, _prev_time: Timestamp) -> Result<(), String> {
    let config = Config::filter_by_version(&0).unwrap();

    // Retrieve the maximum number of nodes we want to spawn from the Config table
    let num_resource_nodes = config.num_resource_nodes as usize;

    // Count the number of nodes currently spawned and exit if we have reached num_resource_nodes
    let num_resource_nodes_spawned = ResourceNodeComponent::iter().count();
    if num_resource_nodes_spawned >= num_resource_nodes {
        log::info!("All resource nodes spawned. Skipping.");
        return Ok(());
    }

    // Pick a random X and Z based off the map_extents
    let mut rng = rand::thread_rng();
    let map_extents = config.map_extents as f32;
    let location = StdbVector2 {
        x: rng.gen_range(-map_extents..map_extents),
        z: rng.gen_range(-map_extents..map_extents),
    };
    // Pick a random Y rotation in degrees
    let rotation = rng.gen_range(0.0..360.0);

    // Insert our SpawnableEntityComponent which assigns us our entity_id
    let entity_id = SpawnableEntityComponent::insert(SpawnableEntityComponent { entity_id: 0 })
        .expect("Failed to create resource spawnable entity component.")
        .entity_id;

    // Insert our static location with the random position and rotation we selected
    StaticLocationComponent::insert(StaticLocationComponent {
        entity_id,
        location: location.clone(),
        rotation,
    })
    .expect("Failed to insert resource static location component.");

    // Insert our resource node component, so far we only have iron
    ResourceNodeComponent::insert(ResourceNodeComponent {
        entity_id,
        resource_type: ResourceNodeType::Iron,
    })
    .expect("Failed to insert resource node component.");

    // Log that we spawned a node with the entity_id and location
    log::info!(
        "Resource node spawned: {} at ({}, {})",
        entity_id,
        location.x,
        location.z,
    );

    Ok(())
}
