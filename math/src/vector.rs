use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn amplitude_squared(&self) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Copy,
    {
        self.x * self.x + self.y * self.y
    }

    pub fn break_self(self) -> (T, T) {
        (self.x, self.y)
    }
}

impl Vec2<f64> {
    pub fn normalize(self) -> Self {
        self / self.module()
    }

    pub fn module(&self) -> f64 {
        self.amplitude_squared().sqrt()
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(val: (T, T)) -> Self {
        Vec2 { x: val.0, y: val.1 }
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl<T: AddAssign> Add for Vec2<T> {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}
impl<T: AddAssign + Copy> AddAssign<T> for Vec2<T> {
    fn add_assign(&mut self, other: T) {
        self.x += other;
        self.y += other;
    }
}
impl<T: AddAssign + Copy> Add<T> for Vec2<T> {
    type Output = Self;

    fn add(mut self, other: T) -> Self {
        self += other;
        self
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, other: T) {
        self.x /= other;
        self.y /= other;
    }
}
impl<T: DivAssign + Copy> Div<T> for Vec2<T> {
    type Output = Self;

    fn div(mut self, other: T) -> Self {
        self /= other;
        self
    }
}

impl<T: SubAssign> SubAssign for Vec2<T> {
    fn sub_assign(&mut self, other: Vec2<T>) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl<T: SubAssign> Sub for Vec2<T> {
    type Output = Self;

    fn sub(mut self, other: Vec2<T>) -> Self {
        self -= other;
        self
    }
}
impl<T: SubAssign + Copy> SubAssign<T> for Vec2<T> {
    fn sub_assign(&mut self, other: T) {
        self.x -= other;
        self.y -= other;
    }
}
impl<T: SubAssign + Copy> Sub<T> for Vec2<T> {
    type Output = Self;

    fn sub(mut self, other: T) -> Self {
        self -= other;
        self
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, other: T) {
        self.x *= other;
        self.y *= other;
    }
}

impl<T: MulAssign + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(mut self, other: T) -> Self {
        self *= other;
        self
    }
}

impl<T: Neg<Output = T>> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(mut self) -> Self {
        self.x = -self.x;
        self.y = -self.y;
        self
    }
}
