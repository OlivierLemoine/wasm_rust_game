mod draw;
mod helper;

use engine::components::*;
use engine::specs::prelude::*;
use engine::systems::*;
use helper::{body, request_animation_frame};
use js_sys::*;
use log::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

struct TestMove;

use engine::specs::prelude::*;

impl<'a> System<'a> for TestMove {
    type SystemData = (Read<'a, engine::KeyPress>, WriteStorage<'a, Transform>);

    fn run(&mut self, (kp, mut transforms): Self::SystemData) {
        for t in (&mut transforms).join() {
            let speed = 5.0;
            if kp.w() {
                t.translate(engine::math::Vec2::from((0.0, speed)));
            }
            if kp.s() {
                t.translate(engine::math::Vec2::from((0.0, -speed)));
            }
            if kp.d() {
                t.translate(engine::math::Vec2::from((speed, 0.0)));
            }
            if kp.a() {
                t.translate(engine::math::Vec2::from((-speed, 0.0)));
            }
        }
    }
}

#[wasm_bindgen]
pub fn start() -> Result<(), JsValue> {
    let mut world = engine::new_world();
    init(&mut world);

    let closure = Rc::new(RefCell::new(None));
    let imediate_closure = closure.clone();
    let mut mover = TestMove;
    engine::specs::shred::RunNow::setup(&mut mover, &mut world);
    let mut physics = PhysicsSystem;
    engine::specs::shred::RunNow::setup(&mut physics, &mut world);
    let mut renderer = draw::SysRender;
    engine::specs::shred::RunNow::setup(&mut renderer, &mut world);

    let world = Rc::new(RefCell::new(world));

    {
        let world_ev_kd = world.clone();
        let closure = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
            let mut w: std::cell::RefMut<World> = world_ev_kd.borrow_mut();
            let kp: &mut engine::KeyPress = w.get_mut().unwrap();
            kp.update_from_str(ev.key().as_str(), true);
        }) as Box<dyn FnMut(_)>);

        body()?.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let world_ev_ku = world.clone();
        let closure = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
            let mut w: std::cell::RefMut<World> = world_ev_ku.borrow_mut();
            let kp: &mut engine::KeyPress = w.get_mut().unwrap();
            kp.update_from_str(ev.key().as_str(), false);
        }) as Box<dyn FnMut(_)>);

        body()?.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    *imediate_closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut w = world.borrow_mut();
        mover.run_now(&mut w);
        // physics.run_now(&mut w);
        renderer.run_now(&mut w);

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
    world
        .create_entity()
        .with(Transform::default())
        .with(RigidBodyBuilder::new().set_mass(10.0).build())
        .with(Sprite::from(vec![engine::Image::rec(
            engine::Color::red(),
            100,
            100,
        )]))
        .build();
}
