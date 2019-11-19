pub mod collider;
pub mod physics;
pub mod render;
pub mod transform;

use specs::prelude::*;

pub fn register_components(world: &mut World) {
    world.register::<collider::Collider>();
    world.register::<physics::RigidBody>();
    world.register::<render::sprite::Sprite>();
}
