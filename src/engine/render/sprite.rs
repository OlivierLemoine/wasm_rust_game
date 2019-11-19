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

pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn red() -> Self {
        Color(255, 0, 0, 255)
    }

    pub fn _green() -> Self {
        Color(0, 255, 0, 255)
    }

    pub fn _blue() -> Self {
        Color(0, 0, 255, 255)
    }
}

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn rec(c: Color, width: usize, height: usize) -> Self {
        let image_size = width * height * 4;

        let data = (0..image_size)
            .map(|i| match i % 4 {
                0 => c.0,
                1 => c.1,
                2 => c.2,
                _ => c.3,
            })
            .collect();

        Image {
            data,
            width,
            height,
        }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        self.height as u32
    }
}
