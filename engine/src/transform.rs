use math::Vec2;
use specs::prelude::*;

pub struct TransformBuilder {
    position: Option<Vec2<f64>>,
    rotation: Option<f64>,
    scale: Option<Vec2<f64>>,
}

impl TransformBuilder {
    pub fn new() -> Self {
        TransformBuilder {
            position: None,
            rotation: None,
            scale: None,
        }
    }

    pub fn position(mut self, pos: Vec2<f64>) -> Self {
        self.position = Some(pos);
        self
    }

    pub fn rotation(mut self, rot: f64) -> Self {
        self.rotation = Some(rot);
        self
    }

    pub fn scale(mut self, scal: Vec2<f64>) -> Self {
        self.scale = Some(scal);
        self
    }

    pub fn build(self) -> Transform {
        let TransformBuilder {
            position,
            rotation,
            scale,
        } = self;

        Transform {
            position: match position {
                Some(v) => v,
                None => Vec2::default(),
            },
            rotation: match rotation {
                Some(v) => v,
                None => 0.0,
            },
            scale: match scale {
                Some(v) => v,
                None => Vec2::from((1.0, 1.0)),
            },
        }
    }
}

#[derive(Debug)]
pub struct Transform {
    pub position: Vec2<f64>,
    pub rotation: f64,
    pub scale: Vec2<f64>,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vec2::default(),
            rotation: 0.0,
            scale: Vec2::from((1.0, 1.0)),
        }
    }
}

impl Transform {
    #[inline]
    pub fn position(&self) -> &Vec2<f64> {
        &self.position
    }

    pub fn position_mut(&mut self) -> &mut Vec2<f64> {
        &mut self.position
    }

    pub fn scale(&self) -> &Vec2<f64> {
        &self.scale
    }

    pub fn scale_mut(&mut self) -> &mut Vec2<f64> {
        &mut self.scale
    }

    pub fn translate(&mut self, other: Vec2<f64>) {
        self.position += other;
    }

    pub fn flip_side(&mut self) {
        self.scale.x = -self.scale.x;
    }

    pub fn face_left(&mut self) {
        if self.scale.x > 0.0 {
            self.flip_side();
        }
    }

    pub fn face_right(&mut self) {
        if self.scale.x < 0.0 {
            self.flip_side();
        }
    }
}

impl Component for Transform {
    type Storage = VecStorage<Self>;
}
