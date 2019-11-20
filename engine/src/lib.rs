mod camera;
mod collider;
mod event;
mod physics;
mod render;
mod transform;

pub mod types {
    pub use crate::collider::ColliderType;
}
pub mod builder {
    pub use crate::collider::ColliderBuilder;
    pub use crate::physics::RigidBodyBuilder;
    pub use crate::transform::TransformBuilder;
}
pub mod components {
    pub use crate::collider::Collider;
    pub use crate::physics::RigidBody;
    pub use crate::render::sprite::Sprite;
    pub use crate::transform::Transform;
}
pub mod systems {
    pub use crate::physics::PhysicsSystem;
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
}

impl Game {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert(event::KeyPress::default());
        world.insert(camera::Camera::default());
        world.register::<collider::Collider>();
        world.register::<physics::RigidBody>();
        world.register::<transform::Transform>();
        world.register::<render::sprite::Sprite>();

        let mut physics = physics::PhysicsSystem;

        specs::shred::RunNow::setup(&mut physics, &mut world);

        Game { world, physics }
    }

    pub fn run_sys(&mut self) {
        self.physics.run_now(&mut self.world);
    }
}
