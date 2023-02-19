use super::move_npc;
use super::update_npc_animation;
use crate::components::AnimationComponent;
use crate::components::NpcComponent;
use crate::components::PlayerLoginComponent;
use crate::components::TransformComponent;
use crate::helpers;
use crate::math::StdbQuaternion;
use crate::math::StdbVector3;
use crate::{random, Config};
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use spacetimedb::Timestamp;
use spacetimedb::{hash::hash_bytes, spacetimedb, ReducerContext};
use std::f32::consts::PI;
use std::ops::Add;
use spacetimedb::println;

#[spacetimedb(reducer, repeat = 5000ms)]
pub(crate) fn spawn_npcs(ctx: ReducerContext, _prev_time: Timestamp) {
    println!("spawn_npcs");

    let config = Config::filter_by_version(0);
    if config.is_none() {
        return;
    }
    let config = config.unwrap();
    let timestamp = ctx
        .timestamp
        .duration_since(Timestamp::UNIX_EPOCH)
        .ok()
        .unwrap()
        .as_millis() as u64;
    random::register();
    let mut rng = ChaCha8Rng::seed_from_u64(timestamp);

    let min_sq_radius = config.min_spawn_range.powi(2);

    // pick a random logged-in player around which the npc will be spawned
    let logged_in_players: Vec<u64> = PlayerLoginComponent::iter()
        .filter(|p| p.logged_in)
        .map(|p| p.entity_id)
        .collect();
    let count = logged_in_players.len();
    if count == 0 {
        // nobody is logged in, no need for npcs
        return;
    }
    let index = rng.gen_range(0..logged_in_players.len()) as usize;
    let player_entity_id = logged_in_players[index];

    let mut spawn_pos = TransformComponent::filter_by_entity_id(player_entity_id).unwrap().pos;

    let range = rng.gen_range(config.min_spawn_range..config.max_spawn_range);
    let rad = rng.gen_range(-PI..PI);

    spawn_pos.x += range * rad.cos();
    spawn_pos.z += range * rad.sin();

    // Make sure the position is not within (min_radius) distance of another player
    for player in PlayerLoginComponent::iter() {
        if player.logged_in {
            let transform = TransformComponent::filter_by_entity_id(player.entity_id).unwrap();
            let dist = transform.pos.sq_distance(&spawn_pos);
            if dist < min_sq_radius {
                return;
            }
        }
    }

    // Make sure the position is not within (min_radius) distance of another npc
    for npc in NpcComponent::iter() {
        let transform = TransformComponent::filter_by_entity_id(npc.entity_id).unwrap();
        let dist = transform.pos.sq_distance(&spawn_pos);
        if dist < min_sq_radius {
            return;
        }
    }

    // Spawn the npc.
    let rot = StdbQuaternion::new(0.0, rng.gen_range(-PI..PI), 0.0);

    let entity_id = helpers::next_entity_id();

    // Make sure this npc doesn't already exist
    if NpcComponent::filter_by_entity_id(entity_id).is_some() {
        panic!("A npc with this entity_id already exists: {}", entity_id);
    }

    NpcComponent::insert(NpcComponent {
        entity_id,
        model: "Rabbit".to_string(),
        next_action: timestamp,
    });
    TransformComponent::insert(TransformComponent {
        entity_id,
        pos: spawn_pos,
        rot,
    });
    AnimationComponent::insert(AnimationComponent {
        entity_id,
        moving: false,
        action_target_entity_id: 0,
    });
}

#[spacetimedb(reducer, repeat = 15000ms)]
pub(crate) fn despawn_npcs(_ctx: ReducerContext, _prev_time: Timestamp) {
    println!("despawn_npcs");

    let config = Config::filter_by_version(0);
    if config.is_none() {
        return;
    }
    let config = config.unwrap();

    let min_sq_radius = config.max_spawn_range.powi(2);

    let mut despawn_array = Vec::new();
    // Make sure the position is within (min_radius) distance of a player
    for npc in NpcComponent::iter() {
        let mut within_range = false;
        let npc_transform = TransformComponent::filter_by_entity_id(npc.entity_id).unwrap();
        for player in PlayerLoginComponent::iter() {
            // Keep logged out players for this check so the NPC will still be there when you relog.
            let player_transform = TransformComponent::filter_by_entity_id(player.entity_id).unwrap();
            within_range |= npc_transform.pos.sq_distance(&player_transform.pos) <= min_sq_radius;
        }
        if !within_range {
            despawn_array.push(npc.entity_id);
        }
    }

    for entity_id in despawn_array {
        NpcComponent::delete_by_entity_id(entity_id);
        TransformComponent::delete_by_entity_id(entity_id);
        AnimationComponent::delete_by_entity_id(entity_id);
    }
}

#[spacetimedb(reducer, repeat = 100ms)]
pub(crate) fn move_npcs(ctx: ReducerContext, _prev_time: Timestamp) {
    println!("move_npcs");

    let config = Config::filter_by_version(0);
    if config.is_none() {
        return;
    }
    let config = config.unwrap();
    let detection_range = config.npc_detection_range;

    let timestamp = ctx
        .timestamp
        .duration_since(Timestamp::UNIX_EPOCH)
        .ok()
        .unwrap()
        .as_millis() as u64;

    random::register();
    let mut rng = ChaCha8Rng::seed_from_u64(timestamp);

    let npc_entity_ids: Vec<u64> = NpcComponent::iter().map(|npc| npc.entity_id).collect();

    for npc_entity_id in npc_entity_ids {
        let npc = NpcComponent::filter_by_entity_id(npc_entity_id).unwrap();
        if npc.next_action > timestamp {
            continue;
        }

        let npc_transform = TransformComponent::filter_by_entity_id(npc_entity_id).unwrap();
        let mut vector = StdbVector3 { x: 0.0, y: 0.0, z: 0.0 };

        // Calculate threat level under the form of a vector
        for player in PlayerLoginComponent::iter() {
            if player.logged_in {
                // Keep logged out players for this check so the NPC will still be there when you relog.
                let player_transform = TransformComponent::filter_by_entity_id(player.entity_id).unwrap();
                let delta = npc_transform.pos - player_transform.pos;
                let len = (detection_range - delta.length()).max(0.0);
                if len > 0.0 {
                    vector = vector.add(delta.normalized() * len);
                }
            }
        }

        if rng.gen_range(0.0..detection_range) <= vector.length() {
            // React on threat
            move_npc(
                hash_bytes(vec![0]), // todo : server hash
                timestamp,
                npc_entity_id,
                npc_transform.pos + vector.normalized() * 2.0,
                StdbQuaternion::look_rotation(vector, StdbVector3::up()),
                300000,
            );
            update_npc_animation(hash_bytes(vec![0]), timestamp, npc_entity_id, true, 0);
        } else {
            // React randomly
            let rnd = rng.gen_range(0..40);
            if rnd == 0 {
                let distance = rng.gen_range(1.0..2.0);
                let vector = StdbVector3 {
                    x: rng.gen_range(-1.0..1.0),
                    y: 0.0,
                    z: rng.gen_range(-1.0..1.0),
                };
                move_npc(
                    hash_bytes(vec![0]), // todo : server hash
                    timestamp,
                    npc_entity_id,
                    npc_transform.pos + vector.normalized() * distance,
                    StdbQuaternion::look_rotation(vector, StdbVector3::up()),
                    (150000.0 * distance) as u64,
                );
                update_npc_animation(hash_bytes(vec![0]), timestamp, npc_entity_id, true, 0);
            } else {
                let npc_animation = AnimationComponent::filter_by_entity_id(npc_entity_id).unwrap();
                if npc_animation.moving {
                    update_npc_animation(hash_bytes(vec![0]), timestamp, npc_entity_id, false, 0);
                }
                let mut npc = NpcComponent::filter_by_entity_id(npc_entity_id).unwrap();
                npc.next_action = timestamp + rng.gen_range(100000..300000);
                NpcComponent::update_by_entity_id(npc_entity_id, npc);
            }
        }
    }
}
