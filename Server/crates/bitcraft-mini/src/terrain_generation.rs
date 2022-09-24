use crate::{TransformComponent, PlayerComponent, Config};
use spacetimedb::spacetimedb;
use std::collections::BTreeSet;
use crate::components::chunk_component::{ChunkPosition, generate_chunk};
use crate::components::chunk_component::Chunk;
use crate::components::chunk_component::world_pos_to_chunk_pos;

#[spacetimedb(reducer, repeat = 1000ms)]
pub(crate) fn check_chunks_for_all_players(_timestamp: u64, _delta_time: u64) {
    let max_chunks_per_call = 20;
    let config = Config::filter_version_eq(0);
    if let None = config {
        return;
    }
    let config = config.unwrap();
    let mut chunk_positions = BTreeSet::<ChunkPosition>::new();
    let spawn_radius = 8;

    for player in PlayerComponent::iter() {
        let transform = TransformComponent::filter_entity_id_eq(player.entity_id).unwrap();
        let player_chunk_position =
            world_pos_to_chunk_pos(transform.pos.x as f64, transform.pos.z as f64, config.chunk_size);

        for x in -spawn_radius..spawn_radius {
            for y in -spawn_radius..spawn_radius {
                let pos = ChunkPosition {
                    x: player_chunk_position.x + x,
                    y: player_chunk_position.y + y,
                };

                if let None = chunk_positions.get(&pos) {
                    chunk_positions.insert(pos);
                }
            }
        }
    }

    for chunk in Chunk::iter() {
        chunk_positions.remove(&chunk.position);
    }

    let mut idx = 0;
    for chunk_pos in chunk_positions {
        if idx >= max_chunks_per_call {
            return;
        }
        generate_chunk(chunk_pos);
        idx += 1;
    }
}
