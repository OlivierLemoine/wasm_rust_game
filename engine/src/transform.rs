use math::Vec2;
use specs::prelude::*;

#[derive(Default, Debug)]
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

    pub fn translate(&mut self, other: Vec2<f64>) {
        self.position += other;
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
