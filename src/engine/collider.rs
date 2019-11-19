use crate::math::vector::Vec2;
use specs::prelude::*;

pub struct Collider {
    is_trigger: bool,
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
