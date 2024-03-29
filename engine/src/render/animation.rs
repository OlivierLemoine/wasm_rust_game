use super::image::Image;

pub struct AnimationBuilder {
    images: Option<Vec<usize>>,
    wait_time_between_2_img: Option<u32>,
    repeat: Option<bool>,
    next_animation: Option<String>,
}
impl AnimationBuilder {
    pub fn new() -> Self {
        AnimationBuilder {
            images: None,
            wait_time_between_2_img: None,
            repeat: None,
            next_animation: None,
        }
    }

    pub fn register_images_index(mut self, indexes: Vec<usize>) -> Self {
        self.images = Some(indexes);
        self
    }

    pub fn change_wait_time(mut self, time: u32) -> Self {
        self.wait_time_between_2_img = Some(time);
        self
    }

    pub fn no_repeat(mut self) -> Self {
        self.repeat = Some(false);
        self
    }

    pub fn next_animation(mut self, next: String) -> Self {
        self.next_animation = Some(next);
        self
    }

    pub fn build(self, images: &Vec<Image>) -> Animation {
        let AnimationBuilder {
            images: indexes,
            wait_time_between_2_img,
            repeat,
            next_animation,
        } = self;

        let indexes = indexes.unwrap_or(vec![0]);

        let mut res = Vec::new();

        for i in indexes {
            res.push(images[i].clone());
        }

        Animation {
            images: res,
            index: 0,
            length: wait_time_between_2_img.unwrap_or(1),
            curr_timer: 0,
            next: next_animation,
            repeat: repeat.unwrap_or(true),
        }
    }
}

#[derive(Debug)]
pub struct Animation {
    images: Vec<Image>,
    index: usize,
    length: u32,
    curr_timer: u32,
    next: Option<String>,
    repeat: bool,
}
impl Animation {
    pub fn change_length(&mut self, length: u32) {
        self.length = length;
    }

    pub fn get(&self) -> &Image {
        &self.images[self.index]
    }

    pub fn update(&mut self) -> Option<String> {
        self.curr_timer += 1;
        if self.curr_timer >= self.length {
            if self.repeat {
                self.index = (self.index + 1) % self.images.len();
                self.curr_timer = 0;
            }
            return self.next.clone();
        }
        None
    }

    pub fn reset(&mut self) {
        self.curr_timer = 0;
        self.index = 0;
    }
}
