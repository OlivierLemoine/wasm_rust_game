use crate::render::{Color, Image, Renderable};

pub struct Player {}

impl Renderable for Player {
    fn render(&self) -> (Image, u32, u32) {
        (Image::rec(Color::red(), 10, 10), 0, 0)
    }
}
