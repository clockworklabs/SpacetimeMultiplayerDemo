use crate::math::remap;
use crate::math::{clamp, map_to_u8};
use crate::{random, Config};
use conv::{ConvUtil, RoundToNegInf};
use fast_poisson::Poisson2D;
use noise::NoiseFn;
use noise::Perlin;
use noise::Seedable;
use rand::Rng;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha8Rng;
use spacetimedb_bindgen::spacetimedb;
use spacetimedb_bindings::hash::hash_bytes;
use spacetimedb_bindings::Hash;

#[spacetimedb(tuple)]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

#[spacetimedb(tuple)]
pub struct Grass {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}

#[spacetimedb(tuple)]
pub struct Tree {
    pub chunk: ChunkPosition,
    pub tree_idx: u16,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}

#[spacetimedb(tuple)]
pub struct Deposit {
    pub chunk: ChunkPosition,
    pub deposit_idx: u16,
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}

#[spacetimedb(table)]
pub struct ChunkData {
    #[unique]
    pub chunk_id: Hash,
    pub data: Vec<u8>,
    pub grass: Vec<Grass>,
    pub trees: Vec<Tree>,
    pub deposits: Vec<Deposit>,
}

#[spacetimedb(table)]
pub struct Chunk {
    #[unique]
    pub chunk_id: Hash,
    pub position: ChunkPosition,
}

pub(crate) fn generate_chunk(chunk_pos: ChunkPosition) {
    spacetimedb_bindings::println!("Generating chunk: {:?}", chunk_pos);
    let config = Config::filter_version_eq(0).unwrap();
    random::register();

    let mut rng = ChaCha8Rng::seed_from_u64(config.terrain_seed as u64 + chunk_pos.x as u64 + chunk_pos.y as u64);

    let grass_poisson_dist = 0.3;
    let grass_model_scale = [0.8, 1.75];
    let grass_perlin_scale = 1.0;
    let forest_noise_scale = 30.0;
    let forest_noise_strength = 1.0;
    let tree_poisson_dist = 2.5;
    let tree_model_scale = [0.5, 1.5];
    let deposit_model_scale = [0.75, 1.25];
    let deposit_poisson_dist = 5.0;
    let global_noise_offset_x = 1000.0;
    let global_noise_offset_y = 1000.0;

    let mut heightmap = Vec::<u8>::new();
    let chunk_pos_x = chunk_pos.x as f64 * config.chunk_size;
    let chunk_pos_y = chunk_pos.y as f64 * config.chunk_size;
    let splat_point_size = config.chunk_size as f64 / config.chunk_splat_resolution as f64;

    // For now heightmap is just flat
    for _ in 0..config.chunk_terrain_resolution {
        for _ in 0..config.chunk_terrain_resolution {
            heightmap.push(0);
        }
    }

    let dirt_perlin = Perlin::new();
    dirt_perlin.set_seed(config.terrain_seed + 1);
    let sand_perlin = Perlin::new();
    sand_perlin.set_seed(config.terrain_seed + 2);

    fn get_dirt_value(x: f64, y: f64, dirt_perlin: Perlin) -> f32 {
        let dirt_strength: f64 = 3.0;
        let dirt_scale: f64 = 15.0;
        let dirt_value = remap(
            dirt_perlin.get([x / dirt_scale, y / dirt_scale]) as f32,
            -1.0,
            1.0,
            0.0,
            1.0,
        );
        return clamp(remap(dirt_value, 0.7, 1.0, 0.0, 1.0) * dirt_strength as f32, 0.0, 1.0);
    }

    fn get_sand_value(x: f64, y: f64, sand_perlin: Perlin) -> f32 {
        let sand_strength: f32 = 8.0;
        let sand_scale: f64 = 20.0;
        let sand_min: f32 = 0.9;

        let sand_value = sand_perlin.get([x / sand_scale, y / sand_scale]) as f32;
        let sand_reinterpolate = remap(sand_value, -1.0, 1.0, 0.0, 1.0);
        return clamp(
            remap(sand_reinterpolate, sand_min, 1.0, 0.0, 1.0) * sand_strength,
            0.0,
            1.0,
        );
    }

    let mut dirt_splat = Vec::<u8>::new();
    let mut sand_splat = Vec::<u8>::new();
    // The splat maps are laid out in the X direction first
    for y in 0..config.chunk_splat_resolution {
        for x in 0..config.chunk_splat_resolution {
            let mut splat_world_x =
                x as f64 * splat_point_size + chunk_pos_x + (splat_point_size / 2.0) + global_noise_offset_x;
            let mut splat_world_y =
                y as f64 * splat_point_size + chunk_pos_y + (splat_point_size / 2.0) + global_noise_offset_y;

            // Seamless splat probing
            if x == 0 {
                splat_world_x = chunk_pos_x + global_noise_offset_x;
            }
            if y == 0 {
                splat_world_y = chunk_pos_y + global_noise_offset_y;
            }
            if x == config.chunk_splat_resolution - 1 {
                splat_world_x = chunk_pos_x + config.chunk_size + global_noise_offset_x;
            }
            if y == config.chunk_splat_resolution - 1 {
                splat_world_y = chunk_pos_y + config.chunk_size + global_noise_offset_y;
            }

            dirt_splat.push(map_to_u8(
                get_dirt_value(splat_world_x, splat_world_y, dirt_perlin),
                0.0,
                1.0,
            ));
            sand_splat.push(map_to_u8(0.0, 0.0, 1.0));
        }
    }

    let forest_noise = Perlin::new();
    forest_noise.set_seed(config.terrain_seed + 3);
    let mut forest_poisson = Poisson2D::new();
    let mut trees = Vec::<Tree>::new();
    let mut tree_idx: u16 = 0;
    let tree_points = forest_poisson.with_dimensions([config.chunk_size, config.chunk_size], tree_poisson_dist);
    for point in tree_points.generate() {
        let splat_world_x = (point[0] as f64 + chunk_pos_x) / forest_noise_scale;
        let splat_world_y = (point[1] as f64 + chunk_pos_y) / forest_noise_scale;

        let dirt_value = get_dirt_value(splat_world_x, splat_world_y, dirt_perlin);
        let forest_value = clamp(
            remap(
                forest_noise.get([splat_world_x, splat_world_y]) as f32,
                0.6,
                1.0,
                0.0,
                1.0,
            ) * forest_noise_strength,
            0.0,
            1.0,
        );
        if forest_value - dirt_value > rng.gen_range(0.0..1.0) {
            trees.push(Tree {
                chunk: chunk_pos,
                tree_idx,
                x: point[0] as f32,
                y: point[1] as f32,
                scale: remap(forest_value as f32, 0.0, 1.0, tree_model_scale[0], tree_model_scale[1]),
            });
            tree_idx += 1;
        }
    }

    let grass_perlin = Perlin::new();
    grass_perlin.set_seed(config.terrain_seed + 4);
    let mut grass_poisson = Poisson2D::new();
    let mut grass = Vec::<Grass>::new();
    let grass_points = grass_poisson.with_dimensions([config.chunk_size, config.chunk_size], grass_poisson_dist);
    for point in grass_points.generate() {
        let splat_world_x = (point[0] as f64 + chunk_pos_x) + global_noise_offset_x;
        let splat_world_y = (point[1] as f64 + chunk_pos_y) + global_noise_offset_y;
        let grass_world_x = (point[0] as f64 + chunk_pos_x) / grass_perlin_scale;
        let grass_world_y = (point[1] as f64 + chunk_pos_y) / grass_perlin_scale;

        let dirt_value = get_dirt_value(splat_world_x, splat_world_y, dirt_perlin);
        let grass_noise = grass_perlin.get([grass_world_x, grass_world_y]);
        if grass_noise - dirt_value as f64 > rng.gen_range(0.0..1.0) {
            grass.push(Grass {
                x: point[0] as f32,
                y: point[1] as f32,
                scale: rng.gen_range(grass_model_scale[0]..grass_model_scale[1]),
            })
        }
    }

    let mut deposit_poisson = Poisson2D::new();
    let mut deposits = Vec::<Deposit>::new();
    let deposit_points = deposit_poisson.with_dimensions([config.chunk_size, config.chunk_size], deposit_poisson_dist);
    let mut deposit_idx = 0;
    for point in deposit_points.generate() {
        let splat_world_x = (point[0] as f64 + chunk_pos_x) + global_noise_offset_x;
        let splat_world_y = (point[1] as f64 + chunk_pos_y) + global_noise_offset_y;

        let dirt_value = get_dirt_value(splat_world_x, splat_world_y, dirt_perlin);
        if dirt_value >= 0.95 {
            deposits.push(Deposit {
                chunk: chunk_pos,
                deposit_idx,
                x: point[0] as f32,
                y: point[1] as f32,
                scale: rng.gen_range(deposit_model_scale[0]..deposit_model_scale[1]),
            });

            deposit_idx += 1;
        }
    }

    ChunkData::insert(ChunkData {
        chunk_id: hash_chunk(chunk_pos),
        data: encode_chunk_data(vec![heightmap, dirt_splat, sand_splat]),
        grass,
        trees,
        deposits,
    });

    Chunk::insert(Chunk {
        chunk_id: hash_chunk(chunk_pos),
        position: chunk_pos,
    });
}

fn encode_chunk_data(splats: Vec<Vec<u8>>) -> Vec<u8> {
    let mut result = Vec::<u8>::new();
    for splat in splats {
        result.extend_from_slice(splat.as_slice());
    }

    return result;
}

fn hash_chunk(pos: ChunkPosition) -> Hash {
    let mut buff = Vec::<u8>::new();
    let value = spacetimedb_bindings::TypeValue::I32(pos.x);
    value.encode(&mut buff);
    let value = spacetimedb_bindings::TypeValue::I32(pos.y);
    value.encode(&mut buff);
    hash_bytes(buff)
}

pub(crate) fn world_pos_to_chunk_pos(x: f64, y: f64, chunk_size: f64) -> ChunkPosition {
    let x = (x / chunk_size).approx_as_by::<i32, RoundToNegInf>().unwrap();
    let y = (y / chunk_size).approx_as_by::<i32, RoundToNegInf>().unwrap();
    ChunkPosition { x, y }
}
