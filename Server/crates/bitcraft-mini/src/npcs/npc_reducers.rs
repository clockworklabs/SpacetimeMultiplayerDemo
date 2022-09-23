use crate::{
    components::{AnimationComponent, NpcComponent, TransformComponent},
    math::{StdbQuaternion, StdbVector3},
};
use spacetimedb_bindgen::spacetimedb;
use spacetimedb_lib::hash::Hash;

#[spacetimedb(reducer)]
pub fn move_npc(_identity: Hash, timestamp: u64, entity_id: u32, pos: StdbVector3, rot: StdbQuaternion, duration: u64) {
    /*
    TODO: Uncomment when supported.
    if identity != 0 {
        println!("Only the server should move NPCs (allowed for now)");
    }
    */

    // Next action timestamp
    let mut npc = NpcComponent::filter_entity_id_eq(entity_id).expect("This npc doesn't exist.");
    npc.next_action = timestamp + duration;
    NpcComponent::update_entity_id_eq(entity_id, npc);

    TransformComponent::update_entity_id_eq(entity_id, TransformComponent { entity_id, pos, rot });
}

#[spacetimedb(reducer)]
pub fn update_npc_animation(_identity: Hash, _timestamp: u64, entity_id: u32, moving: bool, action: u32) {
    let _npc = NpcComponent::filter_entity_id_eq(entity_id).expect("This npc doesn't exist.");

    /*
    TODO: Uncomment when supported.
    // Make sure this identity owns this player
    if identity != 0 {
        println!("Only the server should animate NPCs (allowed for now)");
    }
    */

    AnimationComponent::update_entity_id_eq(
        entity_id,
        AnimationComponent {
            entity_id,
            moving,
            action,
        },
    );
}
