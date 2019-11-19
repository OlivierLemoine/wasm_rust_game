use super::transform::Transform;
use crate::math::vector::Vec2;
use lazy_static::*;
use specs::prelude::*;

lazy_static! {
    static ref GRAVITY: Vec2<f64> = Vec2::from((0.0, -9.90));
}

#[derive(Default)]
pub struct RigidBody {
    mass: f64,
    force: Vec2<f64>,
    acceleration: Vec2<f64>,
    velocity: Vec2<f64>,
}

impl RigidBody {}

impl Component for RigidBody {
    type Storage = DenseVecStorage<Self>;
}

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Transform>, WriteStorage<'a, RigidBody>);

    fn run(&mut self, (mut transforms, mut rigid_bodies): Self::SystemData) {
        for (t, r) in (&mut transforms, &mut rigid_bodies).join() {
            r.acceleration += r.force / r.mass + *GRAVITY;
            r.velocity += r.acceleration;
            *t.position_mut() += r.velocity;
        }
    }
}
