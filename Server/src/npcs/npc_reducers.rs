use crate::{
    components::{AnimationComponent, NpcComponent, TransformComponent},
    math::{StdbQuaternion, StdbVector3},
};
use spacetimedb::{spacetimedb, Identity};

#[spacetimedb(reducer)]
pub fn move_npc(_identity: Identity, timestamp: u64, entity_id: u64, pos: StdbVector3, rot: StdbQuaternion, duration: u64) {
    /*
    TODO: Uncomment when supported.
    if identity != 0 {
        println!("Only the server should move NPCs (allowed for now)");
    }
    */

    // Next action timestamp
    let mut npc = NpcComponent::filter_by_entity_id(entity_id).expect("This npc doesn't exist.");
    npc.next_action = timestamp + duration;
    NpcComponent::update_by_entity_id(entity_id, npc);

    TransformComponent::update_by_entity_id(entity_id, TransformComponent { entity_id, pos, rot });
}

#[spacetimedb(reducer)]
pub fn update_npc_animation(
    _identity: Identity,
    _timestamp: u64,
    entity_id: u64,
    moving: bool,
    action_target_entity_id: u64,
) {
    let _npc = NpcComponent::filter_by_entity_id(entity_id).expect("This npc doesn't exist.");

    /*
    TODO: Uncomment when supported.
    // Make sure this identity owns this player
    if identity != 0 {
        println!("Only the server should animate NPCs (allowed for now)");
    }
    */

    AnimationComponent::update_by_entity_id(
        entity_id,
        AnimationComponent {
            entity_id,
            moving,
            action_target_entity_id,
        },
    );
}
