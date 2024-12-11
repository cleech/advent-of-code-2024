advent_of_code::solution!(10);

mod grid {
    use crate::Point;

    use std::fmt::Debug;
    use std::ops::{Index, IndexMut};
    use std::str::FromStr;

    pub struct Grid<T> {
        pub width: isize,
        pub height: isize,
        raw: Vec<T>,
    }

    impl Grid<u8> {
        pub fn _raw(input: &str) -> Self {
            let bytes: Vec<_> = input.lines().map(str::as_bytes).collect();
            let height = bytes.len() as isize;
            let width = bytes[0].len() as isize;
            let mut raw = Vec::with_capacity((width * height) as usize);
            bytes.iter().for_each(|slice| raw.extend_from_slice(slice));
            Grid { width, height, raw }
        }
    }

    impl<T: FromStr> Grid<T>
    where
        <T as FromStr>::Err: Debug,
    {
        pub fn parse(input: &str) -> Self {
            let v: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
            let height = v.len() as isize;
            let width = v[0].len() as isize;
            let data = v
                .into_iter()
                .flat_map(|l| l.into_iter().map(|c| c.to_string().parse::<T>().unwrap()))
                .collect();
            Grid {
                width,
                height,
                raw: data,
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

        pub fn in_bounds(&self, (x, y): Point) -> bool {
            x >= 0 && x < self.width && y >= 0 && y < self.height
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
}

use grid::Grid;

mod point {
    pub type Point = (isize, isize);
}

use point::Point;

fn trailscore(grid: &mut Grid<(u32, bool)>, head: Point, part2: bool) -> u32 {
    let loc = grid[head].0;
    grid[head].1 = true;
    if loc == 9 {
        return 1;
    }

    let mut ret: u32 = 0;
    let (x, y) = head;
    for &next in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
        if grid.in_bounds(next) && (part2 || grid[next].1 == false) && grid[next].0 == loc + 1 {
            ret += trailscore(grid, next, part2);
        }
    }
    ret
}

pub fn solve(input: &str, part2: bool) -> Option<u32> {
    let grid: Grid<u32> = Grid::parse(input);
    let mut score = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid[(x, y)] == 0 {
                let mut grid = grid.map(|&h| (h, false));
                score += trailscore(&mut grid, (x, y), part2);
            }
        }
    }
    Some(score)
}
pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(81));
    }
}
