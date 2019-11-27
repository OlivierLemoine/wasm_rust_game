use super::transform::Transform;
use lazy_static::*;
use math::Vec2;
use specs::prelude::*;

lazy_static! {
    static ref GRAVITY: Vec2<f64> = Vec2::from((0.0, -9.90 / 60.0));
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
    pub mass: f64,
    pub force: Vec2<f64>,
    pub acceleration: Vec2<f64>,
    pub velocity: Vec2<f64>,
}

impl RigidBody {
    pub fn impulse(&mut self, pulse: Vec2<f64>) {
        self.force += pulse;
    }
}

impl Component for RigidBody {
    type Storage = DenseVecStorage<Self>;
}

pub struct PhysicsSystem;

impl<'a> System<'a> for PhysicsSystem {
    type SystemData = (WriteStorage<'a, Transform>, WriteStorage<'a, RigidBody>);

    fn run(&mut self, (mut transforms, mut rigid_bodies): Self::SystemData) {
        for (t, r) in (&mut transforms, &mut rigid_bodies).join() {
            r.acceleration = if r.mass > 0.0 {
                let force = r.force;
                r.force = Vec2::from((0.0, 0.0));
                force / r.mass + *GRAVITY
            } else {
                Vec2::from((0.0, 0.0))
            };
            r.velocity += r.acceleration;

            t.position += r.velocity;
        }
    }
}
