use crate::math::StdbQuaternion;
use crate::math::StdbVector3;
use spacetimedb_bindgen::spacetimedb;

#[spacetimedb(table)]
pub struct TransformComponent {
    #[unique]
    pub entity_id: u32,
    pub pos: StdbVector3,
    pub rot: StdbQuaternion,
}
