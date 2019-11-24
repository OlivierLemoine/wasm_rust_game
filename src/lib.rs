mod draw;
mod helper;

use engine::specs::prelude::*;
use engine::specs::prelude::*;
use engine::{builder::*, components::*, systems::*, types::*};
use helper::{body, request_animation_frame};
use js_sys::*;
use log::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Default)]
struct Player;
struct TestMove;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

impl<'a> System<'a> for TestMove {
    type SystemData = (
        Read<'a, engine::KeyPress>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (kp, mut transforms, players): Self::SystemData) {
        for (t, _) in (&mut transforms, &players).join() {
            let speed = 2.0;
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
    let mut game = engine::Game::new();
    game.world.register::<Player>();
    init(&mut game.world);

    let closure = Rc::new(RefCell::new(None));
    let imediate_closure = closure.clone();
    let mut mover = TestMove;
    engine::specs::shred::RunNow::setup(&mut mover, &mut game.world);
    let mut renderer = draw::SysRender;
    engine::specs::shred::RunNow::setup(&mut renderer, &mut game.world);
    let mut deb = draw::DebugCollider;
    engine::specs::shred::RunNow::setup(&mut deb, &mut game.world);

    let game = Rc::new(RefCell::new(game));

    {
        let game_ev_kd = game.clone();
        let closure = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
            let mut w: std::cell::RefMut<engine::Game> = game_ev_kd.borrow_mut();
            let kp: &mut engine::KeyPress = w.world.get_mut().unwrap();
            kp.update_from_str(ev.key().as_str(), true);
        }) as Box<dyn FnMut(_)>);

        body()?.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let game_ev_ku = game.clone();
        let closure = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
            let mut w: std::cell::RefMut<engine::Game> = game_ev_ku.borrow_mut();
            let kp: &mut engine::KeyPress = w.world.get_mut().unwrap();
            kp.update_from_str(ev.key().as_str(), false);
        }) as Box<dyn FnMut(_)>);

        body()?.add_event_listener_with_callback("keyup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    *imediate_closure.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut g = game.borrow_mut();
        g.run_sys();
        mover.run_now(&mut g.world);
        renderer.run_now(&mut g.world);
        deb.run_now(&mut g.world);

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
        .with(
            TransformBuilder::new()
                .position(engine::math::Vec2::from((0.0, -100.0)))
                .build(),
        )
        .with(
            ColliderBuilder::new()
                // .collider_type(ColliderType::Circle(15.0))
                .collider_type(ColliderType::Rect(800.0, 30.0))
                .build(),
        )
        .with(Collisions::default())
        .with(Sprite::from(vec![engine::Image::rec(
            engine::Color::blue(),
            800,
            30,
        )]))
        .build();
    world
        .create_entity()
        .with(Transform::default())
        .with(RigidBodyBuilder::new().set_mass(0.0).build())
        .with(
            ColliderBuilder::new()
                // .collider_type(ColliderType::Circle(25.0))
                .collider_type(ColliderType::Rect(50.0, 50.0))
                .build(),
        )
        .with(Collisions::default())
        .with(Sprite::from(vec![engine::Image::rec(
            engine::Color::red(),
            50,
            50,
        )]))
        .with(Player)
        .build();
}
