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
    type SystemData = (
        WriteStorage<'a, Transform>,
        WriteStorage<'a, RigidBody>,
        WriteStorage<'a, Collisions>,
    );

    fn run(&mut self, (mut transforms, mut rigid_bodies, mut collisions): Self::SystemData) {
        for (t, r, c) in (&mut transforms, &mut rigid_bodies, &mut collisions).join() {
            let (col_x, col_y) = (*c).take().map_or((0.0f64, 0.0f64), |v| v.at.break_self());

            r.acceleration = if r.mass > 0.0 {
                let force = r.force;
                r.force = Vec2::from((0.0, 0.0));
                force / r.mass + *GRAVITY
            } else {
                Vec2::from((0.0, 0.0))
            };
            r.velocity += r.acceleration;

            if col_x != 0.0 {
                *r.acceleration.x_mut() = 0.0;
                *r.velocity.x_mut() = 0.0;
            }
            if col_y != 0.0 {
                *r.acceleration.y_mut() = 0.0;
                *r.velocity.y_mut() = 0.0;
            }
            *t.position_mut().x_mut() += col_x;
            *t.position_mut().y_mut() += col_y;

            *t.position_mut() += r.velocity;
        }
    }
}
