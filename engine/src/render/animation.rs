use super::image::Image;

#[derive(Debug)]
pub struct Animation {
    images: Vec<Image>,
    index: usize,
}
impl Animation {
    pub fn get(&self) -> &Image {
        &self.images[self.index]
    }
}
impl From<Vec<Image>> for Animation {
    fn from(images: Vec<Image>) -> Self {
        Animation { images, index: 0 }
    }
}
