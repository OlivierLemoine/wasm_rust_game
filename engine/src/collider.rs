use crate::physics::RigidBody;
use crate::transform::Transform;
// use log::*;
use math::Vec2;
use specs::prelude::*;
use std::ops::{Deref, DerefMut};

macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x < y {
            $x
        } else {
            y
        }
    }}
}

macro_rules! min_abs {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => {{
        let y = min!($($z),*);
        if $x.abs() < y.abs() {
            $x
        } else {
            y
        }
    }}
}

macro_rules! flag {
    ($component_name: ident, $anti_component_name: ident, $system_name: ident) => {
        #[derive(Default)]
        pub struct $component_name;
        impl Component for $component_name {
            type Storage = NullStorage<Self>;
        }
        #[derive(Default)]
        pub struct $anti_component_name;
        impl Component for $anti_component_name {
            type Storage = NullStorage<Self>;
        }
        pub struct $system_name;
        impl<'a> System<'a> for $system_name {
            type SystemData = (
                Entities<'a>,
                WriteStorage<'a, Collisions>,
                ReadStorage<'a, Collider>,
                ReadStorage<'a, Transform>,
                ReadStorage<'a, $component_name>,
                ReadStorage<'a, $anti_component_name>,
            );
            fn run(
                &mut self,
                (entities, mut collisions, colliders, transforms, flags, anti_flags): Self::SystemData,
            ) {
                for (e, c, c1, t, _) in
                    (&entities, &mut collisions, &colliders, &transforms, &flags).join()
                {
                    c.0 = Vec::new();
                    c.1 = false;
                    for (e2, c2, t2, _, _) in (&entities, &colliders, &transforms, &flags, !&anti_flags).join() {
                        if e != e2 {
                            if let Some(v) = c1.0.collide_with(
                                &c2.0,
                                t.position.clone(),
                                t2.position.clone(),
                            ) {
                                if v.y > 0.0f64 {
                                    c.1 = true;
                                }
                                c.0.push(Collision { with: e2, at: v });
                            }
                        }
                    }
                }
            }
        }
    };
}

flag!(Layer1, AntiLayer1, Layer1System);
flag!(Layer2, AntiLayer2, Layer2System);

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
            (ColliderType::Rect(w1, h1), ColliderType::Rect(w2, h2)) => {
                let w1 = *w1 / 2.0;
                let h1 = *h1 / 2.0;
                let w2 = *w2 / 2.0;
                let h2 = *h2 / 2.0;

                let b1_x_min = p1.x - w1;
                let b1_x_max = p1.x + w1;
                let b1_y_min = p1.y - h1;
                let b1_y_max = p1.y + h1;

                let b2_x_min = p2.x - w2;
                let b2_x_max = p2.x + w2;
                let b2_y_min = p2.y - h2;
                let b2_y_max = p2.y + h2;

                [
                    (b1_x_min, b1_y_min),
                    (b1_x_min, b1_y_max),
                    (b1_x_max, b1_y_min),
                    (b1_x_max, b1_y_max),
                ]
                .iter()
                .map(|(x, y)| {
                    if b2_x_min < *x && *x < b2_x_max && b2_y_min < *y && *y < b2_y_max {
                        let v1 = min_abs!(x - b2_x_min, x - b2_x_max);
                        let v2 = min_abs!(y - b2_y_min, y - b2_y_max);
                        if v2 == 0.0 || v1 != 0.0 && v1.abs() < v2.abs() {
                            Some(Vec2::from((-v1, 0.0)))
                        } else {
                            Some(Vec2::from((0.0, -v2)))
                        }
                    } else {
                        None
                    }
                })
                .flat_map(|x| x)
                .next()
                .or_else(|| {
                    [
                        (b2_x_min, b2_y_min),
                        (b2_x_min, b2_y_max),
                        (b2_x_max, b2_y_min),
                        (b2_x_max, b2_y_max),
                    ]
                    .iter()
                    .map(|(x, y)| {
                        if b1_x_min < *x && *x < b1_x_max && b1_y_min < *y && *y < b1_y_max {
                            let v1 = min_abs!(x - b1_x_min, x - b1_x_max);
                            let v2 = min_abs!(y - b1_y_min, y - b1_y_max);
                            if v2 == 0.0 || v1 != 0.0 && v1.abs() < v2.abs() {
                                Some(Vec2::from((v1, 0.0)))
                            } else {
                                Some(Vec2::from((0.0, v2)))
                            }
                        } else {
                            None
                        }
                    })
                    .flat_map(|x| x)
                    .next()
                })
            }
            (ColliderType::Rect(w, h), ColliderType::Circle(r)) => {
                let w = *w / 2.0;
                let h = *h / 2.0;

                let b_x_min = p1.x - w;
                let b_x_max = p1.x + w;
                let b_y_min = p1.y - h;
                let b_y_max = p1.y + h;

                let radius_square = r * r;

                [
                    (b_x_min, b_y_min),
                    (b_x_min, b_y_max),
                    (b_x_max, b_y_min),
                    (b_x_max, b_y_max),
                ]
                .iter()
                .map(|(x, y)| {
                    let dir = Vec2::from((*x, *y)) - p2;
                    if (dir).amplitude_squared() < radius_square {
                        Some(dir.normalize() * (r - dir.module()))
                    } else {
                        None
                    }
                })
                .flat_map(|x| x)
                .next()
            }
            (ColliderType::Circle(r), ColliderType::Rect(w, h)) => ColliderType::Rect(*w, *h)
                .collide_with(&ColliderType::Circle(*r), p1, p2)
                .map(|v| -v),
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

pub struct Collider(ColliderType);
impl Component for Collider {
    type Storage = DenseVecStorage<Self>;
}
impl Deref for Collider {
    type Target = ColliderType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Collider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default)]
pub struct Collisions(Vec<Collision>, bool);
impl Collisions {
    pub fn has_hit_bottom(&self) -> bool {
        self.1
    }
}
impl Component for Collisions {
    type Storage = DenseVecStorage<Self>;
}
impl Deref for Collisions {
    type Target = Vec<Collision>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Collisions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct RepultionSystem;
impl<'a> System<'a> for RepultionSystem {
    type SystemData = (
        WriteStorage<'a, Collisions>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, RigidBody>,
    );

    fn run(&mut self, (mut collisions, mut transforms, mut rigidbodies): Self::SystemData) {
        for (c, t, r) in (&mut collisions, &mut transforms, &mut rigidbodies).join() {
            let _: Vec<_> = (*c)
                .iter()
                .map(|v| {
                    let (col_x, col_y) = v.at.break_self();

                    if col_x != 0.0 {
                        t.position.x += col_x;
                        r.acceleration.x = 0.0;
                        r.velocity.x = 0.0;
                    }
                    if col_y != 0.0 {
                        t.position.y += col_y;
                        r.acceleration.y = 0.0;
                        r.velocity.y = 0.0;
                    }
                })
                .collect();
        }
    }
}
