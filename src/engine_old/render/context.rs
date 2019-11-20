use super::sprite::Image;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

// use std::sync::

pub struct Context {
    ctx: web_sys::CanvasRenderingContext2d,
    width: usize,
    height: usize,
}

unsafe impl Sync for Context {}

impl Context {
    pub fn from_id(id: &str) -> Context {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(id).unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let width = canvas.width() as usize;
        let height = canvas.height() as usize;

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        Ok(Context { ctx, width, height })
    }

    pub fn draw(&self, img: &Image, pos_x: u32, pos_y: u32) {
        self.ctx
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut img.data().clone()),
            img.width(),
            img.height(),
        )?;
        self.ctx.put_image_data(&data, pos_x as f64, pos_y as f64)
    }
}
