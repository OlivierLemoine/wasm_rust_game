use engine::components::{Collider, Sprite, Transform};
use engine::specs::prelude::*;
use engine::{Camera, Image};
use js_sys::*;
use lazy_static::*;
use log::*;
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

lazy_static! {
    static ref CTX: Context = { Context::from_id("game").unwrap() };
}

static mut CANVAS_WIDTH: f64 = 1.0;
static mut CANVAS_HEIGHT: f64 = 1.0;

#[wasm_bindgen]
pub fn resize() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("game").unwrap();
    let canvas: web_sys::HtmlCanvasElement =
        canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    unsafe {
        CANVAS_WIDTH = width;
        CANVAS_HEIGHT = height;
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
            let canvas_center_x = unsafe { CANVAS_WIDTH } as f64 / 2.0;
            let canvas_center_y = unsafe { CANVAS_HEIGHT } as f64 / 2.0;

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

        let mut res = ImgToDraw::new(unsafe { CANVAS_WIDTH } as usize, unsafe { CANVAS_HEIGHT }
            as usize);

        for (t, s) in (&transforms, &sprites).join() {
            if let Some(img) = s.image() {
                let image_center_x = img.width() as i32 / 2;
                let image_center_y = img.height() as i32 / 2;

                let canvas_center_x = unsafe { CANVAS_WIDTH } as i32 / 2;
                let canvas_center_y = unsafe { CANVAS_HEIGHT } as i32 / 2;

                let obj_center_x = *t.position().x() as i32;
                let obj_center_y = *t.position().y() as i32;

                let obj_scale_x = *t.scale().x() as i32;

                let pos_x = canvas_center_x - image_center_x + obj_center_x;
                let pos_y = canvas_center_y - image_center_y - obj_center_y;
                let width = img.width();

                if obj_scale_x < 0 {
                    res.put_flipped(img.data(), width as usize, pos_x, pos_y);
                } else {
                    res.put(img.data(), width as usize, pos_x, pos_y);
                }
            }

            ctx.draw(&mut res.0, unsafe { CANVAS_WIDTH } as u32, 0, 0)
                .unwrap();
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

        let ctx = canvas
            .get_context("2d")?
            .ok_or(JsValue::from(Error::new("No context")))?
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        Ok(Context { ctx })
    }

    pub fn draw(
        &self,
        img: &mut Vec<u8>,
        width: u32,
        pos_x: u32,
        pos_y: u32,
    ) -> Result<(), JsValue> {
        let data = ImageData::new_with_u8_clamped_array(Clamped(img), width)?;
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
}

struct ImgToDraw(Vec<u8>, usize, usize);
impl ImgToDraw {
    fn new(w: usize, h: usize) -> Self {
        ImgToDraw(vec![0; 4 * w * h], w, h)
    }
    fn put(&mut self, img: &Vec<u8>, width: usize, x: i32, y: i32) {
        for i in 0..width as i32 * (img.len() / 4 / width) as i32 {
            let i_4 = i as usize * 4;
            let r = i_4;
            let g = i_4 + 1;
            let b = i_4 + 2;
            let a = i_4 + 3;

            let index_x = i % width as i32;
            let index_y = i / width as i32;
            let global_x = x + index_x;
            let global_y = y + index_y;

            if 0 <= global_x
                && global_x < self.1 as i32
                && 0 <= global_y
                && global_y < self.2 as i32
            {
                let global_index = global_x as usize + global_y as usize * self.1;
                if img[a] != 0 {
                    self.0[global_index * 4] = img[r];
                    self.0[global_index * 4 + 1] = img[g];
                    self.0[global_index * 4 + 2] = img[b];
                    self.0[global_index * 4 + 3] = 255;
                }
            }
        }
    }
    fn put_flipped(&mut self, img: &Vec<u8>, width: usize, x: i32, y: i32) {
        for i in 0..width as i32 * (img.len() / 4 / width) as i32 {
            let index_x = i % width as i32;
            let index_y = i / width as i32;
            let global_x = x + index_x;
            let global_y = y + index_y;

            if 0 <= global_x
                && global_x < self.1 as i32
                && 0 <= global_y
                && global_y < self.2 as i32
            {
                let global_index = (global_x as usize + global_y as usize * self.1) * 4;
                let local_index = ((width - index_x as usize - 1) + index_y as usize * width) * 4;
                if img[local_index + 3] != 0 {
                    self.0[global_index] = img[local_index];
                    self.0[global_index + 1] = img[local_index + 1];
                    self.0[global_index + 2] = img[local_index + 2];
                    self.0[global_index + 3] = 255;
                }
            }
        }
    }
}
