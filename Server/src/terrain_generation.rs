use crate::components::chunk_component::world_pos_to_chunk_pos;
use crate::components::chunk_component::Chunk;
use crate::components::chunk_component::{generate_chunk, ChunkPosition};
use crate::{Config, PlayerComponent, TransformComponent};
use spacetimedb::spacetimedb;
use spacetimedb::ReducerContext;
use spacetimedb::Timestamp;
use std::collections::BTreeSet;

#[spacetimedb(reducer, repeat = 1000ms)]
pub(crate) fn check_chunks_for_all_players(_ctx: ReducerContext, _prev_time: Timestamp) {
    let max_chunks_per_call = 20;
    let config = Config::filter_by_version(&0);
    if config.is_none() {
        return;
    }
    let config = config.unwrap();
    let mut chunk_positions = BTreeSet::<ChunkPosition>::new();
    let spawn_radius = 8;

    for player in PlayerComponent::iter() {
        let transform = TransformComponent::filter_by_entity_id(&player.entity_id).unwrap();
        let player_chunk_position =
            world_pos_to_chunk_pos(transform.pos.x as f64, transform.pos.z as f64, config.chunk_size);

        for x in -spawn_radius..spawn_radius {
            for y in -spawn_radius..spawn_radius {
                let pos = ChunkPosition {
                    x: player_chunk_position.x + x,
                    y: player_chunk_position.y + y,
                };

                if chunk_positions.get(&pos).is_none() {
                    chunk_positions.insert(pos);
                }
            }
        }
    }

    for chunk in Chunk::iter() {
        chunk_positions.remove(&chunk.position);
    }

    for (idx, chunk_pos) in chunk_positions.into_iter().enumerate() {
        if idx >= max_chunks_per_call {
            return;
        }
        generate_chunk(chunk_pos);
    }
}
