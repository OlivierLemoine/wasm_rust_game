use crate::transform::Transform;
use math::Vec2;
use specs::prelude::*;

pub enum ColliderType {
    Circle(f64),
    None,
}

impl ColliderType {
    fn collide_with(&self, other: &ColliderType, p1: Vec2<f64>, p2: Vec2<f64>) -> bool {
        match (self, other) {
            (ColliderType::Circle(r1), ColliderType::Circle(r2)) => {
                let line = p2 - p1;
                let dist = line.amplitude_squared();
                let rad = (r1 + r2) * (r1 + r2);
                rad > dist
            }
            (_, _) => false,
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

        Collider {
            col_type: match col_type {
                Some(v) => v,
                None => ColliderType::None,
            },
            collisions: vec![],
        }
    }
}

pub struct Collision {}

pub struct Collider {
    col_type: ColliderType,
    collisions: Vec<Collision>,
}

impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}

pub struct CollideSystem;

impl<'a> System<'a> for CollideSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Collider>,
        ReadStorage<'a, Collider>,
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (entities, mut colliders, colliders_r, transforms): Self::SystemData) {
        for (e, c, t) in (&entities, &mut colliders, &transforms).join() {
            for (e2, c2, t2) in (&entities, &colliders_r, &transforms).join() {
                if e != e2 {
                    if c.col_type.collide_with(
                        &c2.col_type,
                        t.position().clone(),
                        t2.position().clone(),
                    ) {
                        c.collisions.push(Collision {});
                    }
                }
            }
        }
    }
}
