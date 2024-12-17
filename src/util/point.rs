use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Point(pub isize, pub isize);

/* unit vectors for orthoganal directions, as points for ease of use */
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

impl Sub for Point {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1);
    }
}

impl Point {
    /* 90° rotations */
    #[inline]
    pub fn clockwise(&self) -> Self {
        Point(-self.1, self.0)
    }
    #[inline]
    pub fn counter_clockwise(&self) -> Self {
        Point(self.1, -self.0)
    }
}
