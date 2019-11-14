use crate::vector::Vec2;
use specs::prelude::*;

#[derive(Default)]
pub struct Transform {
    position: Vec2<f32>,
}

impl Transform {
    pub fn position(&self) -> &Vec2<f32> {
        &self.position
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
