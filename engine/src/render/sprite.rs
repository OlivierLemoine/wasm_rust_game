use super::image::Image;
use log::*;
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
    pub fn register_sprite_size(mut self, width: usize, height: usize) -> Self {
        self.image_size = Some((width, height));
        self
    }
    pub fn build(self) -> Sprite {
        let SpriteBuilder {
            raw_image,
            image_size,
        } = self;

        Sprite {
            images: match (raw_image, image_size) {
                (Some(img), Some((w, h))) => {
                    let nb_row = img.width() / w as u32;
                    let nb_col = img.height() / h as u32;
                    let mut res = vec![vec![0u8; w * h * 4]; nb_row as usize * nb_col as usize];

                    let datas = img.data();

                    for i in 0..nb_col {
                        for j in 0..nb_row {
                            let offset_w = i as usize * w;
                            let offset_h = j as usize * h;
                            for k in 0..w * h * 4 {
                                let base_index = k % w + img.width() as usize * (k / w);
                                let final_index = base_index + offset_w + offset_h;
                                res[i as usize + j as usize * nb_col as usize][k] =
                                    datas[final_index];
                            }
                        }
                    }
                    console_log!("test");
                    res.iter()
                        .map(|v| Image::from_raw(v.to_vec(), w, h))
                        .collect()
                }
                (Some(img), None) => vec![img],
                (_, _) => vec![Image::rec(super::color::Color::red(), 10, 10)],
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
        if self.images.len() > 0 {
            &self.images[self.index]
        } else {
            panic!("Not enougth to display")
        }
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
