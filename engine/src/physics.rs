use super::collider::Collisions;
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
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, RigidBody>,
        WriteStorage<'a, Collisions>,
    );

    fn run(&mut self, (mut transforms, mut rigid_bodies, mut collisions): Self::SystemData) {
        for (t, r, c) in (&mut transforms, &mut rigid_bodies, &mut collisions).join() {
            if let Some(v) = c.0.take() {
                r.acceleration = Vec2::from((0.0, 0.0));
                r.velocity = Vec2::from((0.0, 0.0));
                *t.position_mut() += v.at;
            } else {
                r.acceleration = if r.mass > 0.0 {
                    r.force / r.mass + *GRAVITY
                } else {
                    Vec2::from((0.0, 0.0))
                };
                r.velocity += r.acceleration;
                *t.position_mut() += r.velocity;
            }
        }
    }
}
