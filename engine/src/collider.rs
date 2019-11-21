use crate::physics::RigidBody;
use crate::transform::Transform;
use log::*;
use math::Vec2;
use specs::prelude::*;

pub enum ColliderType {
    Circle(f64),
    Rect(f64, f64),
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
                    let depl = rad.sqrt() - dist.sqrt();
                    Some(-line.normalize() * depl)
                } else {
                    None
                }
            }
            (ColliderType::Rect(w1, h1), ColliderType::Rect(w2, h2)) => None,
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
                        // console_log!("{:?}", v);
                        c.0 = Some(Collision { with: e2, at: v });
                        // pause();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collide_2_circle() {
        let c1 = ColliderType::Circle(1.0);
        let c2 = ColliderType::Circle(1.0);

        let p1 = Vec2::from((0.0f64, 0.0));
        let p2 = Vec2::from((2.0f64, 0.0));
        assert_eq!(c1.collide_with(&c2, p1, p2), None);
        let p2 = Vec2::from((1.0f64, 0.0));
        assert_eq!(
            c1.collide_with(&c2, p1, p2),
            Some(Vec2::from((-1.0f64, 0.0)))
        );
    }
}
