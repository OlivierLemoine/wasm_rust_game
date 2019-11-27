use super::animation::{Animation, AnimationBuilder};
use super::color::Color;
use super::image::Image;
// use log::*;
use specs::prelude::*;
use std::collections::BTreeMap;

pub struct SpriteBuilder {
    raw_image: Option<Image>,
    image_size: Option<(usize, usize)>,
    animations: Vec<(String, AnimationBuilder)>,
}
impl SpriteBuilder {
    pub fn new() -> Self {
        SpriteBuilder {
            raw_image: None,
            image_size: None,
            animations: Vec::new(),
        }
    }
    pub fn register_animation(mut self, name: String, animation: AnimationBuilder) -> Self {
        self.animations.push((name, animation));
        self
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
    pub fn apply_transparancy_on(mut self, c: Color) -> Self {
        if let Some(img) = &mut self.raw_image {
            let Color(r, g, b, _) = c;
            for i in (0..img.data().len()).step_by(4) {
                if img.data()[i] == r && img.data()[i + 1] == g && img.data()[i + 2] == b {
                    img.data_mut()[i + 3] = 0;
                }
            }
        }
        self
    }
    pub fn build(self) -> Sprite {
        let SpriteBuilder {
            raw_image,
            image_size,
            animations,
        } = self;

        let sprites: Vec<_> = match (raw_image, image_size) {
            (Some(img), Some((w, h))) => {
                let nb_col = img.width() as usize / w;
                let nb_row = img.height() as usize / h;
                let mut res = vec![vec![0u8; w * h * 4]; nb_row * nb_col];

                for i in (0..img.width() as usize * img.height() as usize * 4).step_by(4) {
                    let color_r = img.data()[i];
                    let color_g = img.data()[i + 1];
                    let color_b = img.data()[i + 2];
                    let color_a = img.data()[i + 3];

                    let pixel_index = i / 4;

                    let global_pixel_x = pixel_index % img.width() as usize;
                    let global_pixel_y = pixel_index / img.width() as usize;

                    let sub_image_x = global_pixel_x / w;
                    let sub_image_y = global_pixel_y / h;

                    let index_sub_image = sub_image_x + sub_image_y * nb_col;

                    let local_pixel_x = global_pixel_x % w;
                    let local_pixel_y = global_pixel_y % h;
                    let local_index_pixel = local_pixel_x + local_pixel_y * w;

                    res[index_sub_image][local_index_pixel * 4] = color_r;
                    res[index_sub_image][local_index_pixel * 4 + 1] = color_g;
                    res[index_sub_image][local_index_pixel * 4 + 2] = color_b;
                    res[index_sub_image][local_index_pixel * 4 + 3] = color_a;
                }

                res.iter()
                    .map(|v| Image::from_raw(v.clone(), w, h))
                    .collect()
            }
            (Some(img), None) => vec![img].into(),
            (_, _) => vec![Image::rec(Color::red(), 10, 10)].into(),
        };

        let mut animations_tree = BTreeMap::new();

        let curr_animation = if animations.len() > 0 {
            animations[0].0.clone()
        } else {
            "".into()
        };

        for (name, anim_builder) in animations {
            animations_tree.insert(name, anim_builder.build(&sprites));
        }

        Sprite {
            animations: animations_tree,
            curr_animation,
        }
    }
}

#[derive(Debug)]
pub struct Sprite {
    animations: BTreeMap<String, Animation>,
    curr_animation: String,
}
impl Sprite {
    pub fn image(&self) -> Option<&Image> {
        self.animations.get(&self.curr_animation).map(|v| v.get())
    }
    pub fn animation(&mut self, anim: String) {
        self.curr_animation = anim;
        self.animations
            .get_mut(&self.curr_animation)
            .map(|v| v.reset());
    }
    pub fn update(&mut self) {
        self.animations
            .get_mut(&self.curr_animation)
            .map(|v| v.update());
    }
}
impl From<Vec<Image>> for Sprite {
    fn from(images: Vec<Image>) -> Self {
        let mut animations = BTreeMap::new();
        animations.insert(String::from(""), AnimationBuilder::new().build(&images));
        Sprite {
            animations,
            curr_animation: "".into(),
        }
    }
}
impl Component for Sprite {
    type Storage = VecStorage<Sprite>;
}

pub struct SpriteUpdaterSystem;

impl<'a> System<'a> for SpriteUpdaterSystem {
    type SystemData = WriteStorage<'a, Sprite>;

    fn run(&mut self, mut sprites: Self::SystemData) {
        for s in (&mut sprites).join() {
            s.update();
        }
    }
}

#[test]
fn sprite_splitting() {
    println!(
        "{:?}",
        SpriteBuilder::new()
            .add_image_from_raw(
                vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
                4,
                1
            )
            .register_sprite_size(1, 1)
            .build()
    );
    unimplemented!();
}
