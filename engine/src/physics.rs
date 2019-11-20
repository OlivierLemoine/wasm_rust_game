use super::transform::Transform;
use lazy_static::*;
use log::*;
use math::Vec2;
use specs::prelude::*;

lazy_static! {
    static ref GRAVITY: Vec2<f64> = Vec2::from((0.0, -9.90));
}

pub struct RigidBodyBuilder {
    mass: Option<f64>,
}

impl RigidBodyBuilder {
    pub fn new() -> Self {
        RigidBodyBuilder { mass: None }
    }

    pub fn set_mass(mut self, mass: f64) -> Self {
        self.mass = Some(mass);
        self
    }

    pub fn build(self) -> RigidBody {
        let mut res = RigidBody::default();
        res.mass = match self.mass {
            Some(x) => x,
            None => 0.0,
        };

        res
    }
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
        // console_log!("test");
        for (t, r) in (&mut transforms, &mut rigid_bodies).join() {
            r.acceleration += r.force / r.mass + *GRAVITY;
            r.velocity += r.acceleration;
            *t.position_mut() += r.velocity;
        }
    }
}
