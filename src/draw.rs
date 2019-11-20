use engine::components::{Sprite, Transform};
use engine::specs::prelude::*;
use engine::Image;
use js_sys::*;
use lazy_static::*;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

lazy_static! {
    static ref CTX: Context = { Context::from_id("game").unwrap() };
}

pub struct SysRender;

impl<'a> System<'a> for SysRender {
    type SystemData = (ReadStorage<'a, Transform>, ReadStorage<'a, Sprite>);

    fn run(&mut self, (transforms, sprites): Self::SystemData) {
        for (t, s) in (&transforms, &sprites).join() {
            let t: &Transform = t;
            let s: &Sprite = s;
            let ctx: &Context = &CTX;

            ctx.draw(
                s.image(),
                *t.position().x() as u32,
                *t.position().y() as u32,
            )
            .unwrap();
        }
    }
}

pub struct Context {
    ctx: web_sys::CanvasRenderingContext2d,
    width: usize,
    height: usize,
}

unsafe impl Sync for Context {}

impl Context {
    pub fn from_id(id: &str) -> Result<Context, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(id).unwrap();
        let canvas: web_sys::HtmlCanvasElement =
            canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let width = canvas.width() as usize;
        let height = canvas.height() as usize;

        let ctx = canvas
            .get_context("2d")?
            .ok_or(JsValue::from(Error::new("No context")))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        Ok(Context { ctx, width, height })
    }

    pub fn draw(&self, img: &Image, pos_x: u32, pos_y: u32) -> Result<(), JsValue> {
        self.ctx
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut img.data().clone()),
            img.width(),
            img.height(),
        )?;
        self.ctx.put_image_data(&data, pos_x as f64, pos_y as f64)?;
        Ok(())
    }
}
