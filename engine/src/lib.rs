mod collider;
mod event;
mod physics;
mod render;
mod transform;

pub use specs;
pub mod components {
    pub use crate::collider::Collider;
    pub use crate::physics::{RigidBody, RigidBodyBuilder};
    pub use crate::render::sprite::Sprite;
    pub use crate::transform::Transform;
}
pub mod systems {
    pub use crate::physics::PhysicsSystem;
}
pub use crate::render::{color::Color, image::Image};

use specs::prelude::*;

pub fn new_world() -> World {
    let mut world = World::new();
    world.register::<collider::Collider>();
    world.register::<physics::RigidBody>();
    world.register::<transform::Transform>();
    world.register::<render::sprite::Sprite>();
    world
}
