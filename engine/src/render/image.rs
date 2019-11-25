use super::color::Color;

pub struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn from_raw(data: Vec<u8>, width: usize, height: usize) -> Self {
        Image {
            data,
            width,
            height,
        }
    }
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
