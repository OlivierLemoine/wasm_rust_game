use super::image::Image;

#[derive(Debug)]
pub struct Animation {
    images: Vec<Image>,
    index: usize,
    length: u32,
    curr_timer: u32,
}
impl Animation {
    pub fn change_length(&mut self, length: u32) {
        self.length = length;
    }

    pub fn get(&self) -> &Image {
        &self.images[self.index]
    }

    pub fn update(&mut self) {
        self.curr_timer += 1;
        if self.curr_timer >= self.length {
            self.index = (self.index + 1) % self.images.len();
            self.curr_timer = 0;
        }
    }

    pub fn reset(&mut self) {
        self.curr_timer = 0;
        self.index = 0;
    }
}
impl From<Vec<Image>> for Animation {
    fn from(images: Vec<Image>) -> Self {
        Animation {
            images,
            index: 0,
            length: 1,
            curr_timer: 0,
        }
    }
}
