mod camera;
mod collider;
mod event;
mod macros;
mod physics;
mod render;
mod transform;

pub mod types {
    pub use crate::collider::ColliderType;
}
pub mod builder {
    pub use crate::collider::ColliderBuilder;
    pub use crate::physics::RigidBodyBuilder;
    pub use crate::render::animation::AnimationBuilder;
    pub use crate::render::sprite::SpriteBuilder;
    pub use crate::transform::TransformBuilder;
}
pub mod components {
    pub use crate::collider::{AntiLayer1, Layer1};
    pub use crate::collider::{AntiLayer2, Layer2};
    pub use crate::collider::{Collider, Collisions};
    pub use crate::physics::RigidBody;
    pub use crate::render::sprite::Sprite;
    pub use crate::transform::Transform;
}
pub use camera::Camera;
pub use event::KeyPress;
pub use math;
pub use render::{color::Color, image::Image};
pub use specs;

use specs::prelude::*;

pub struct Game {
    pub world: specs::shred::World,
    physics: physics::PhysicsSystem,
    layer1: collider::Layer1System,
    layer2: collider::Layer2System,
    repulsor: collider::RepultionSystem,
    sprite: render::sprite::SpriteUpdaterSystem,
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert(event::KeyPress::default());
        world.insert(camera::Camera::default());
        world.register::<collider::Collider>();
        world.register::<collider::Collisions>();
        world.register::<physics::RigidBody>();
        world.register::<transform::Transform>();
        world.register::<render::sprite::Sprite>();

        let mut physics = physics::PhysicsSystem;
        let mut layer1 = collider::Layer1System;
        let mut layer2 = collider::Layer2System;
        let mut repulsor = collider::RepultionSystem;
        let mut sprite = render::sprite::SpriteUpdaterSystem;

        specs::shred::RunNow::setup(&mut physics, &mut world);
        specs::shred::RunNow::setup(&mut layer1, &mut world);
        specs::shred::RunNow::setup(&mut layer2, &mut world);
        specs::shred::RunNow::setup(&mut repulsor, &mut world);
        specs::shred::RunNow::setup(&mut sprite, &mut world);

        Game {
            world,
            physics,
            layer1,
            layer2,
            repulsor,
            sprite,
        }
    }

    pub fn run_sys(&mut self) {
        self.physics.run_now(&mut self.world);
        self.layer1.run_now(&mut self.world);
        self.layer2.run_now(&mut self.world);
        self.repulsor.run_now(&mut self.world);
        self.sprite.run_now(&mut self.world);
    }
}
