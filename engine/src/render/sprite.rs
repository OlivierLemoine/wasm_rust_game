use super::image::Image;
use specs::prelude::*;

pub struct SpriteBuilder {
    raw_image: Option<Image>,
    image_size: Option<(usize, usize)>,
}
impl SpriteBuilder {
    pub fn new() -> Self {
        SpriteBuilder {
            raw_image: None,
            image_size: None,
        }
    }
    pub fn add_image_from_raw(mut self, data: Vec<u8>, width: usize, height: usize) -> Self {
        self.raw_image = Some(Image::from_raw(data, width, height));
        self
    }
    pub fn add_image(mut self, img: Image) -> Self {
        self.raw_image = Some(img);
        self
    }
    pub fn build(self) -> Sprite {
        let SpriteBuilder {
            raw_image,
            image_size,
        } = self;

        Sprite {
            images: match raw_image {
                Some(img) => vec![img],
                None => vec![Image::rec(super::color::Color::red(), 10, 10)],
            },
            index: 0,
        }
    }
}

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
