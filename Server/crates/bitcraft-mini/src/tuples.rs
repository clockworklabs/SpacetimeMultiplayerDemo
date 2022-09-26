use spacetimedb::spacetimedb;

#[derive(Copy, Clone, Debug)]
#[spacetimedb(tuple)]
pub struct Pocket {
    pub item_id: u32,
    pub pocket_idx: u32,
    pub item_count: i32,
}
