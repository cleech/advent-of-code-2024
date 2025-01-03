use super::point::Point;

use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    pub raw: Vec<T>,
}

impl Grid<()> {
    pub fn parse<T>(input: &str) -> Result<Grid<T>, T::Err>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        let v: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let height = v.len() as isize;
        let width = v[0].len() as isize;
        let data = v
            .into_iter()
            .flat_map(|l| l.into_iter().map(|c| c.to_string().parse::<T>()))
            .collect::<Result<Vec<_>, T::Err>>()?;
        Ok(Grid {
            width,
            height,
            raw: data,
        })
    }
}

impl Grid<u8> {
    pub fn _parse(input: &str) -> Self {
        let bytes: Vec<_> = input.lines().map(str::as_bytes).collect();
        let height = bytes.len() as isize;
        let width = bytes[0].len() as isize;
        let mut raw = Vec::with_capacity((width * height) as usize);
        bytes.iter().for_each(|slice| raw.extend_from_slice(slice));
        Grid { width, height, raw }
    }
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(width: isize, height: isize) -> Grid<T> {
        Grid {
            width,
            height,
            raw: vec![T::default(); (width * height) as usize],
        }
    }
}

impl<T: Copy> Grid<T> {
    pub fn with_default(width: isize, height: isize, default: T) -> Grid<T> {
        Grid {
            width,
            height,
            raw: vec![default; (width * height) as usize],
        }
    }
}

impl<T> Grid<T> {
    pub fn map<B, F>(&self, f: F) -> Grid<B>
    where
        F: FnMut(&T) -> B,
    {
        let raw: Vec<B> = self.raw.iter().map(f).collect();
        Grid {
            width: self.width,
            height: self.height,
            raw,
        }
    }

    pub fn in_bounds(&self, Point(x, y): Point) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        (0..self.width)
            .cartesian_product(0..self.height)
            .map(|t| Point(t.0, t.1))
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, goal: T) -> Option<Point> {
        self.raw
            .iter()
            .position(|item| *item == goal)
            .map(|index| Point((index as isize) % self.width, (index as isize) / self.width))
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for y in 0..(self.height as usize) {
            for x in 0..(self.width as usize) {
                write!(f, "{}", self[Point(x as isize, y as isize)])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;
    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self.raw[(self.width * index.1 + index.0) as usize]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.raw[(self.width * index.1 + index.0) as usize]
    }
}
