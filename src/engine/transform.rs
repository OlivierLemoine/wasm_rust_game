use crate::math::vector::Vec2;
use specs::prelude::*;

#[derive(Default)]
pub struct Transform {
    position: Vec2<f64>,
    rotation: f64,
    scale: Vec2<f64>,
}

impl Transform {
    #[inline]
    pub fn position(&self) -> &Vec2<f64> {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut Vec2<f64> {
        &mut self.position
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
