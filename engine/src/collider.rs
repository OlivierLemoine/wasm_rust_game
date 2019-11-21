use crate::physics::RigidBody;
use crate::transform::Transform;
use log::*;
use math::Vec2;
use specs::prelude::*;

pub enum ColliderType {
    Circle(f64),
    None,
}
impl ColliderType {
    fn collide_with(
        &self,
        other: &ColliderType,
        p1: Vec2<f64>,
        p2: Vec2<f64>,
    ) -> Option<Vec2<f64>> {
        match (self, other) {
            (ColliderType::Circle(r1), ColliderType::Circle(r2)) => {
                let line = p2 - p1;
                let dist = line.amplitude_squared();
                let rad = (r1 + r2) * (r1 + r2);
                if rad > dist {
                    let depl = dist.sqrt() - rad.sqrt();
                    Some(-line - depl)
                } else {
                    None
                }
            }
            (_, _) => None,
        }
    }
}

pub struct ColliderBuilder {
    col_type: Option<ColliderType>,
}
impl ColliderBuilder {
    pub fn new() -> Self {
        ColliderBuilder { col_type: None }
    }

    pub fn collider_type(mut self, c: ColliderType) -> Self {
        self.col_type = Some(c);
        self
    }

    pub fn build(self) -> Collider {
        let ColliderBuilder { col_type } = self;

        Collider(match col_type {
            Some(v) => v,
            None => ColliderType::None,
        })
    }
}

pub struct Collision {
    pub with: specs::world::Entity,
    pub at: Vec2<f64>,
}

pub struct Collider(pub ColliderType);
impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
#[derive(Default)]
pub struct Collisions(pub Option<Collision>);
impl Component for Collisions {
    type Storage = DenseVecStorage<Self>;
}

pub struct CollideSystem;
impl<'a> System<'a> for CollideSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Collisions>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, RigidBody>,
    );

    fn run(
        &mut self,
        (entities, mut collisions, colliders, transforms, rigidbodies): Self::SystemData,
    ) {
        for (e, c, c1, t, _) in (
            &entities,
            &mut collisions,
            &colliders,
            &transforms,
            &rigidbodies,
        )
            .join()
        {
            for (e2, c2, t2) in (&entities, &colliders, &transforms).join() {
                if e != e2 {
                    if let Some(v) =
                        c1.0.collide_with(&c2.0, t.position().clone(), t2.position().clone())
                    {
                        console_log!("{:?}", v);
                        c.0 = Some(Collision { with: e2, at: v });
                        pause();
                    }
                }
            }
        }
    }
}
