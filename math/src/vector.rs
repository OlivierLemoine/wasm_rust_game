use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub struct Vec2<T>(T, T);

impl<T> Vec2<T> {
    pub fn amplitude_squared(&self) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: Copy,
    {
        self.0 * self.0 + self.1 * self.1
    }

    pub fn break_self(self) -> (T, T) {
        (self.0, self.1)
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
        Vec2(val.0, val.1)
    }
}

impl<T> Vec2<T> {
    pub fn x(&self) -> &T {
        &self.0
    }
    pub fn y(&self) -> &T {
        &self.1
    }
    pub fn x_mut(&mut self) -> &mut T {
        &mut self.0
    }
    pub fn y_mut(&mut self) -> &mut T {
        &mut self.1
    }
}

impl<T: AddAssign> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
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
        self.0 += other;
        self.1 += other;
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
        self.0 /= other;
        self.1 /= other;
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
        self.0 -= other.0;
        self.1 -= other.1;
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
        self.0 -= other;
        self.1 -= other;
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
        self.0 *= other;
        self.1 *= other;
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
        self.0 = -self.0;
        self.1 = -self.1;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vec_from_tuple() {
        let v = Vec2::from((3u32, 5));
        assert_eq!(v, Vec2(3u32, 5));
    }

    #[test]
    fn vec_add_with_vec() {
        let v1 = Vec2(3u32, 5);
        let v2 = Vec2(4u32, 1);
        assert_eq!(v1 + v2, Vec2(7u32, 6));
    }
}

impl<T: PartialOrd> PartialOrd for Vec2<T> {
    fn partial_cmp(&self, other: &Vec2<T>) -> Option<Ordering> {
        if self.0 == other.0 && self.1 == other.1 {
            Some(Ordering::Equal)
        } else if self.0 <= other.0 && self.1 <= other.1 {
            Some(Ordering::Less)
        } else if self.0 >= other.0 && self.1 >= other.1 {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

#[test]
fn ord() {
    let v1 = Vec2::from((1u32, 1));
    let v2 = Vec2::from((1u32, 1));
    assert_eq!(v1.eq(&v2), true);
    let v2 = Vec2::from((2u32, 2));
    assert_eq!(v1.le(&v2), true);
    let v2 = Vec2::from((2u32, 2));
    assert_eq!(v1.le(&v2), true);
    let v2 = Vec2::from((0u32, 2));
    assert_ne!(v1.eq(&v2), true);
    assert_ne!(v1.le(&v2), true);
    assert_ne!(v1.ge(&v2), true);
    assert_ne!(v1.lt(&v2), true);
    assert_ne!(v1.gt(&v2), true);
}
