mod context;
mod render;
mod sprite;
mod transform;
mod vector;

use js_sys::*;
use specs::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn window() -> Result<web_sys::Window, JsValue> {
    web_sys::window().ok_or(JsValue::from(Error::new(&"No window")))
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) -> Result<(), JsValue> {
    window()?.request_animation_frame(f.as_ref().unchecked_ref())?;
    Ok(())
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let mut world = World::new();
    init(&mut world);

    let closure = Rc::new(RefCell::new(None));
    let imediate_closure = closure.clone();

    *imediate_closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut renderer = render::SysRender;
        specs::shred::RunNow::setup(&mut renderer, &mut world);
        renderer.run_now(&mut world);

        request_animation_frame(closure.borrow().as_ref().unwrap()).unwrap();
    }) as Box<dyn FnMut()>));

    request_animation_frame(
        imediate_closure
            .borrow()
            .as_ref()
            .ok_or(JsValue::from(Error::new("no callback")))?,
    )?;

    Ok(())
}

fn init(world: &mut World) {
    world.register::<transform::Transform>();
    world.register::<sprite::Sprite>();

    world
        .create_entity()
        .with(transform::Transform::default())
        .with(sprite::Sprite::from(vec![sprite::Image::rec(
            sprite::Color::red(),
            100,
            100,
        )]))
        .build();
}
