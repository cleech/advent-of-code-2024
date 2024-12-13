use std::ops::{Add, AddAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point(pub isize, pub isize);

pub const UP: Point = Point(0, -1);
pub const DOWN: Point = Point(0, 1);
pub const RIGHT: Point = Point(1, 0);
pub const LEFT: Point = Point(-1, 0);

pub const ORTHOGONAL: [Point; 4] = [UP, RIGHT, DOWN, LEFT];

impl From<(isize, isize)> for Point {
    #[inline]
    fn from(t: (isize, isize)) -> Self {
        Point(t.0, t.1)
    }
}

impl From<Point> for (isize, isize) {
    #[inline]
    fn from(p: Point) -> Self {
        (p.0, p.1)
    }
}

impl Add for Point {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1);
    }
}
