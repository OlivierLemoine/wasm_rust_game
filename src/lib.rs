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

#[derive(PartialEq)]
enum AnimationState {
    Idle,
    Walk,
    Jump,
}
impl Default for AnimationState {
    fn default() -> Self {
        AnimationState::Idle
    }
}
impl AnimationState {
    pub fn to_string(&self) -> String {
        match self {
            AnimationState::Idle => String::from("idle"),
            AnimationState::Walk => String::from("walk"),
            AnimationState::Jump => String::from("jump"),
        }
    }
}

#[derive(Default)]
struct Player {
    has_jump: bool,
    animation_state: AnimationState,
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
        WriteStorage<'a, Sprite>,
    );

    fn run(
        &mut self,
        (kp, collisions, mut transforms, mut rigidbodies, mut players, mut sprites): Self::SystemData,
    ) {
        for (c, t, r, p, s) in (
            &collisions,
            &mut transforms,
            &mut rigidbodies,
            &mut players,
            &mut sprites,
        )
            .join()
        {
            let speed = 2.0;
            let mut new_player_anim_state = AnimationState::Idle;
            t.face_right();

            if kp.w() {
                if c.has_hit_bottom() {
                    p.has_jump = false;
                }
                if !p.has_jump {
                    r.impulse(engine::math::Vec2::from((0.0, 50.0)));
                    p.has_jump = true;
                }
            }
            if kp.s() {}
            if kp.d() {
                new_player_anim_state = AnimationState::Walk;
                t.translate(engine::math::Vec2::from((speed, 0.0)));
            }
            if kp.a() {
                new_player_anim_state = AnimationState::Walk;
                t.translate(engine::math::Vec2::from((-speed, 0.0)));
                t.face_left();
            }

            if new_player_anim_state != p.animation_state {
                s.animation(new_player_anim_state.to_string());

                p.animation_state = new_player_anim_state;
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
        mover.run_now(&mut g.world);
        g.run_sys();
        renderer.run_now(&mut g.world);
        // deb.run_now(&mut g.world);

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
    create_block_on_grid(world, -2, 2, -1, -2);
    create_block_on_grid(world, 2, 4, -2, -3);
    create_block_on_grid(world, -5, -3, 0, -5);
    create_block_on_grid(world, -3, 7, -4, -5);
    create_block_on_grid(world, 6, 7, -3, -4);
    world
        .create_entity()
        .with(Transform::default())
        .with(RigidBodyBuilder::new().set_mass(10.0).build())
        .with(
            ColliderBuilder::new()
                .collider_type(ColliderType::Rect(32.0, 32.0))
                .build(),
        )
        .with(Collisions::default())
        .with(
            SpriteBuilder::new()
                .add_image(player_image)
                .apply_transparancy_on(engine::Color(0, 0, 0, 0))
                .register_sprite_size(32, 32)
                .add_anim_desc(vec![
                    ("idle".into(), 4, (0..13).collect()),
                    ("walk".into(), 4, (13..21).collect()),
                    ("jump".into(), 4, (65..71).collect()),
                ])
                .build(),
        )
        .with(Player::default())
        .build();
}

fn create_block_on_grid(world: &mut World, l: i32, r: i32, t: i32, b: i32) {
    let l = l as f64 * 40.0;
    let r = r as f64 * 40.0;
    let t = t as f64 * 40.0;
    let b = b as f64 * 40.0;

    // let x =
    let w = r - l;
    let h = t - b;
    let x = l + w / 2.0;
    let y = b + h / 2.0;
    create_block(world, x, y, w, h);
}

fn create_block(world: &mut World, x: f64, y: f64, w: f64, h: f64) {
    world
        .create_entity()
        .with(
            TransformBuilder::new()
                .position(engine::math::Vec2::from((x, y)))
                .build(),
        )
        .with(
            ColliderBuilder::new()
                .collider_type(ColliderType::Rect(w, h))
                .build(),
        )
        .with(Collisions::default())
        .with(Sprite::from(vec![engine::Image::rec(
            engine::Color::blue(),
            w as usize,
            h as usize,
        )]))
        .build();
}
