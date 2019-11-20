mod camera;
mod collider;
mod event;
mod physics;
mod render;
mod transform;

pub mod builder {
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

pub fn new_world() -> World {
    let mut world = World::new();
    world.insert(event::KeyPress::default());
    world.insert(camera::Camera::default());
    world.register::<collider::Collider>();
    world.register::<physics::RigidBody>();
    world.register::<transform::Transform>();
    world.register::<render::sprite::Sprite>();
    world
}
