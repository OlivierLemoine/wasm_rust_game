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
                    let nb_row = img.width() as usize / w;
                    let nb_col = img.height() as usize/ h;
                    let mut res = vec![vec![0u8; w * h * 4]; nb_row * nb_col];

//                    println!("{}", img.data().len());

                    for i in 0..img.width() as usize * img.height() as usize{
                        let elem_r = img.data()[i*4];
                        let elem_g = img.data()[i*4+1];
                        let elem_b = img.data()[i*4+2];
                        let elem_a = img.data()[i*4+3];

                        let col = i / w;
                        let line = col / nb_col;
                        let row = line / h;
                        let index = col % nb_col + row * nb_col;
                        println!("{} {}", col, index);
                        res[index][i % w] = elem_r;
                        res[index][i % w + 1] = elem_g;
                        res[index][i % w+ 2] = elem_b;
                        res[index][i % w +3] = elem_a;
                    }

                    res.iter()
                        .map(|v| Image::from_raw(v.clone(), w, h))
                        .collect()
                }
                (Some(img), None) => vec![img],
                (_, _) => vec![Image::rec(super::color::Color::red(), 10, 10)],
            },
            index: 0,
        }
    }
}

#[derive(Debug)]
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

#[test]
fn sprite_splitting() {
    println!("{:?}", SpriteBuilder::new().add_image_from_raw(vec![0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15], 2, 2).register_sprite_size(1, 1).build());
    unimplemented!();
}