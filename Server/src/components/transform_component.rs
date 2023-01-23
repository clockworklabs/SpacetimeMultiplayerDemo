use crate::math::StdbQuaternion;
use crate::math::StdbVector3;
use spacetimedb::spacetimedb;

#[spacetimedb(table)]
pub struct TransformComponent {
    #[unique]
    pub entity_id: u64,
    pub pos: StdbVector3,
    pub rot: StdbQuaternion,
}
