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

#[derive(Clone)]
enum PlayerState {
    Idle,
    Walk,
    Jump,
    Attack(i32),
}
impl PartialEq for PlayerState {
    fn eq(&self, other: &PlayerState) -> bool {
        match (self, other) {
            (PlayerState::Idle, PlayerState::Idle) => true,
            (PlayerState::Walk, PlayerState::Walk) => true,
            (PlayerState::Jump, PlayerState::Jump) => true,
            (PlayerState::Attack(_), PlayerState::Attack(_)) => true,
            (_, _) => false,
        }
    }
}
impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Idle
    }
}
impl PlayerState {
    pub fn to_string(&self) -> String {
        match self {
            PlayerState::Idle => String::from("idle"),
            PlayerState::Walk => String::from("walk"),
            PlayerState::Jump => String::from("jump_beg"),
            PlayerState::Attack(_) => String::from("attack"),
        }
    }
}

#[derive(Default)]
struct Player {
    // has_jump: bool,
    state: PlayerState,
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
            let speed = if !kp.ShiftLeft() { 2.0 } else { 1.0 };
            let mut new_player_state = p.state.clone();

            match &mut p.state {
                PlayerState::Idle | PlayerState::Walk => {
                    new_player_state = PlayerState::Idle;
                    if kp.KeyD() {
                        new_player_state = PlayerState::Walk;
                        t.translate(engine::math::Vec2::from((speed, 0.0)));
                        t.face_right();
                    }
                    if kp.KeyA() {
                        new_player_state = PlayerState::Walk;
                        t.translate(engine::math::Vec2::from((-speed, 0.0)));
                        t.face_left();
                    }
                    if kp.KeyW() {
                        new_player_state = PlayerState::Jump;
                        r.impulse(engine::math::Vec2::from((0.0, 50.0)));
                    }
                    if kp.KeyK() {
                        new_player_state = PlayerState::Attack(15);
                    }

                    if !c.has_hit_bottom() {
                        new_player_state = PlayerState::Jump;
                    }
                }
                PlayerState::Jump => {
                    if kp.KeyD() {
                        t.translate(engine::math::Vec2::from((speed, 0.0)));
                        t.face_right();
                    }
                    if kp.KeyA() {
                        t.translate(engine::math::Vec2::from((-speed, 0.0)));
                        t.face_left();
                    }
                    if kp.KeyK() {
                        new_player_state = PlayerState::Attack(15);
                    }
                    if c.has_hit_bottom() {
                        new_player_state = PlayerState::Idle;
                    }
                }
                PlayerState::Attack(remaning_time) => {
                    *remaning_time -= 1;
                    if kp.KeyD() {
                        t.translate(engine::math::Vec2::from((speed, 0.0)));
                        t.face_right();
                    }
                    if kp.KeyA() {
                        t.translate(engine::math::Vec2::from((-speed, 0.0)));
                        t.face_left();
                    }
                    if *remaning_time < 0 {
                        if c.has_hit_bottom() {
                            new_player_state = PlayerState::Idle;
                        } else {
                            new_player_state = PlayerState::Jump;
                        }
                    }
                }
            };

            if new_player_state != p.state {
                s.animation(new_player_state.to_string());
                p.state = new_player_state;
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
            kp.update_from_str(ev.code().as_str(), true);
        }) as Box<dyn FnMut(_)>);

        body()?.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let game_ev_ku = game.clone();
        let closure = Closure::wrap(Box::new(move |ev: web_sys::KeyboardEvent| {
            let mut w: std::cell::RefMut<engine::Game> = game_ev_ku.borrow_mut();
            let kp: &mut engine::KeyPress = w.world.get_mut().unwrap();
            kp.update_from_str(ev.code().as_str(), false);
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
    create_block_on_grid(world, 0, 1, 0, -1);
    world
        .create_entity()
        .with(Transform::default())
        .with(RigidBodyBuilder::new().set_mass(10.0).build())
        .with(
            ColliderBuilder::new()
                .collider_type(ColliderType::Rect(14.0, 30.0))
                .build(),
        )
        .with(Collisions::default())
        .with(
            SpriteBuilder::new()
                .add_image(player_image.clone())
                .apply_transparancy_on(engine::Color(0, 0, 0, 0))
                .register_sprite_size(32, 32)
                .register_animation(
                    "idle".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .register_images_index((0..13).collect()),
                )
                .register_animation(
                    "walk".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .register_images_index((13..21).collect()),
                )
                .register_animation(
                    "attack".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .register_images_index((26..36).collect()),
                )
                .register_animation(
                    "jump_beg".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .no_repeat()
                        .next_animation("jump".into())
                        .register_images_index((65..69).collect()),
                )
                .register_animation(
                    "jump".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .no_repeat()
                        .register_images_index((69..70).collect()),
                )
                .register_animation(
                    "jump_end".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .no_repeat()
                        .register_images_index((70..72).collect()),
                )
                .build(),
        )
        .with(Player::default())
        .build();
    world
        .create_entity()
        .with(Transform::default())
        .with(RigidBodyBuilder::new().set_mass(10.0).build())
        .with(
            ColliderBuilder::new()
                .collider_type(ColliderType::Rect(14.0, 30.0))
                .build(),
        )
        .with(Collisions::default())
        .with(
            SpriteBuilder::new()
                .add_image(player_image)
                .apply_transparancy_on(engine::Color(0, 0, 0, 0))
                .register_sprite_size(32, 32)
                .register_animation(
                    "idle".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .register_images_index((0..13).collect()),
                )
                .register_animation(
                    "walk".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .register_images_index((13..21).collect()),
                )
                .register_animation(
                    "attack".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .register_images_index((26..36).collect()),
                )
                .register_animation(
                    "jump_beg".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .no_repeat()
                        .next_animation("jump".into())
                        .register_images_index((65..69).collect()),
                )
                .register_animation(
                    "jump".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .no_repeat()
                        .register_images_index((69..70).collect()),
                )
                .register_animation(
                    "jump_end".into(),
                    AnimationBuilder::new()
                        .change_wait_time(4)
                        .no_repeat()
                        .register_images_index((70..72).collect()),
                )
                .build(),
        )
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
            engine::Color(176, 96, 35, 255),
            w as usize,
            h as usize,
        )]))
        .build();
}
