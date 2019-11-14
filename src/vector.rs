#[derive(Default, Clone)]
pub struct Vec2<T>(T, T);

impl<T> Vec2<T> {
    pub fn x(&self) -> &T {
        &self.0
    }

    pub fn y(&self) -> &T {
        &self.1
    }
}
