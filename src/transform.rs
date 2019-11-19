use crate::vector::Vec2;
use specs::prelude::*;

#[derive(Default)]
pub struct Position(Vec2<f32>);

#[derive(Default)]
pub struct Speed(f32);

impl Position {
    pub fn get(&self) -> &Vec2<f32> {
        &self.0
    }

    pub fn translate(&mut self, depl: Vec2<f32>) {
        self.0 += depl;
    }
}

impl Speed {
    pub fn new(base: f32) -> Self {
        Speed(base)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Component for Speed {
    type Storage = DenseVecStorage<Self>;
}
