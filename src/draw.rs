use engine::components::{Collider, Sprite, Transform};
use engine::specs::prelude::*;
use engine::{Camera, Image};
use js_sys::*;
use lazy_static::*;
// use log::*;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

lazy_static! {
    static ref CTX: Context = { Context::from_id("game").unwrap() };
}

static mut canvas_width: f64 = 1.0;
static mut canvas_height: f64 = 1.0;

#[wasm_bindgen]
pub fn resize() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("game").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    unsafe {
        canvas_width = width;
        canvas_height = height;
    }
}

pub struct DebugCollider;
impl<'a> System<'a> for DebugCollider {
    type SystemData = (
        Read<'a, Camera>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Collider>,
    );

    fn run(&mut self, (_camera, transforms, colliders): Self::SystemData) {
        let ctx: &Context = &CTX;

        for (t, c) in (&transforms, &colliders).join() {
            let canvas_center_x = unsafe { canvas_width } as f64 / 2.0;
            let canvas_center_y = unsafe { canvas_height } as f64 / 2.0;

            let obj_center_x = *t.position().x();
            let obj_center_y = *t.position().y();

            let pos_x = canvas_center_x + obj_center_x;
            let pos_y = canvas_center_y - obj_center_y;

            match **c {
                engine::types::ColliderType::Circle(r) => {
                    ctx.draw_circle(pos_x, pos_y, r).unwrap();
                }
                engine::types::ColliderType::Rect(w, h) => {
                    ctx.draw_rect(pos_x - w / 2.0, pos_y - h / 2.0, w, h)
                        .unwrap();
                }
                _ => {}
            }
        }
    }
}

pub struct SysRender;
impl<'a> System<'a> for SysRender {
    type SystemData = (
        Read<'a, Camera>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Sprite>,
    );

    fn run(&mut self, (_camera, transforms, sprites): Self::SystemData) {
        let ctx: &Context = &CTX;
        ctx.clear();

        for (t, s) in (&transforms, &sprites).join() {
            let image_center_x = s.image().width() as i32 / 2;
            let image_center_y = s.image().height() as i32 / 2;

            let canvas_center_x = unsafe { canvas_width } as i32 / 2;
            let canvas_center_y = unsafe { canvas_height } as i32 / 2;

            let obj_center_x = *t.position().x() as i32;
            let obj_center_y = *t.position().y() as i32;

            let pos_x = canvas_center_x - image_center_x + obj_center_x;
            let pos_y = canvas_center_y - image_center_y - obj_center_y;

            ctx.draw(s.image(), pos_x as u32, pos_y as u32).unwrap();
        }
    }
}

pub struct Context {
    ctx: web_sys::CanvasRenderingContext2d,
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

        Ok(Context { ctx })
    }

    pub fn draw(&self, img: &Image, pos_x: u32, pos_y: u32) -> Result<(), JsValue> {
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut img.data().clone()),
            img.width(),
            img.height(),
        )?;
        self.ctx.put_image_data(&data, pos_x as f64, pos_y as f64)?;
        Ok(())
    }

    pub fn draw_circle(&self, x: f64, y: f64, r: f64) -> Result<(), JsValue> {
        self.ctx.set_stroke_style(&JsValue::from("green"));
        self.ctx.begin_path();
        self.ctx.arc(x, y, r, 0.0, 6.28318530718)?;
        self.ctx.stroke();
        Ok(())
    }

    pub fn draw_rect(&self, x: f64, y: f64, w: f64, h: f64) -> Result<(), JsValue> {
        self.ctx.set_stroke_style(&JsValue::from("green"));
        self.ctx.begin_path();
        self.ctx.rect(x, y, w, h);
        self.ctx.stroke();
        Ok(())
    }

    pub fn clear(&self) {
        self.ctx.clear_rect(
            0.0,
            0.0,
            unsafe { canvas_width } as f64,
            unsafe { canvas_height } as f64,
        );
    }
}
