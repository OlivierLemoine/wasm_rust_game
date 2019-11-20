use super::image::Image;
use specs::prelude::*;

pub struct Sprite {
    images: Vec<Image>,
    index: usize,
}

impl Sprite {
    pub fn image(&self) -> &Image {
        &self.images[self.index]
    }
}

impl From<Vec<Image>> for Sprite {
    fn from(images: Vec<Image>) -> Self {
        Sprite { images, index: 0 }
    }
}

impl Component for Sprite {
    type Storage = VecStorage<Sprite>;
}
