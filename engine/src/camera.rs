use super::transform::Transform;

#[derive(Default)]
pub struct Camera {
    transform: Transform,
}

impl Camera {
    pub fn transform(&self) -> &Transform {
        &self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Transform {
        &mut self.transform
    }
}
