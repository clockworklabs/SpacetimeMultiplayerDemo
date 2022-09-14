use spacetimedb_bindgen::spacetimedb;
use crate::math::StdbVector3;
use crate::math::StdbQuarternion;

#[spacetimedb(table)]
pub struct TransformComponent {
    #[unique]
    pub entity_id: u32,
    pub pos: StdbVector3,
    pub rot: StdbQuarternion,
}