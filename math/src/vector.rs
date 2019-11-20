use std::ops::{Add, AddAssign, Div, DivAssign};

#[derive(Default, Clone, Copy)]
pub struct Vec2<T>(T, T);

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
