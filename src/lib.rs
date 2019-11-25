mod draw;
mod helper;

use engine::specs::prelude::*;
use engine::{builder::*, components::*, types::*};
use helper::{body, request_animation_frame};
use js_sys::*;
use log::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::ImageData;

#[derive(Default)]
struct Player {
    has_jump: bool,
}
impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

struct TestMove;
impl<'a> System<'a> for TestMove {
    type SystemData = (
        Read<'a, engine::KeyPress>,
        ReadStorage<'a, Collisions>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, RigidBody>,
        WriteStorage<'a, Player>,
    );

    fn run(
        &mut self,
        (kp, collisions, mut transforms, mut rigidbodies, mut players): Self::SystemData,
    ) {
        for (c, t, r, p) in (&collisions, &mut transforms, &mut rigidbodies, &mut players).join() {
            let speed = 2.0;
            if kp.w() {
                if c.has_hit_bottom() {
                    p.has_jump = false;
                }
                if !p.has_jump {
                    r.impulse(engine::math::Vec2::from((0.0, 50.0)));
                    p.has_jump = true;
                }
            }
            if kp.s() {
                // t.translate(engine::math::Vec2::from((0.0, -speed)));
                // r.impulse(engine::math::Vec2::from((0.0, -100.0)));
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
pub fn start(player_image: ImageData) -> Result<(), JsValue> {
    let player_image = engine::Image::from_raw(
        player_image.data().to_vec(),
        player_image.width() as usize,
        player_image.height() as usize,
    );
    let mut game = engine::Game::new();
    game.world.register::<Player>();
    init(&mut game.world, player_image);

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

fn init(world: &mut World, player_image: engine::Image) {
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
                .collider_type(ColliderType::Rect(100.0, 30.0))
                .build(),
        )
        .with(Collisions::default())
        .with(Sprite::from(vec![engine::Image::rec(
            engine::Color::blue(),
            100,
            30,
        )]))
        .build();
    world
        .create_entity()
        .with(Transform::default())
        .with(RigidBodyBuilder::new().set_mass(10.0).build())
        .with(
            ColliderBuilder::new()
                // .collider_type(ColliderType::Circle(25.0))
                .collider_type(ColliderType::Rect(50.0, 50.0))
                .build(),
        )
        .with(Collisions::default())
        .with(SpriteBuilder::new().add_image(player_image).build())
        .with(Player::default())
        .build();
}
