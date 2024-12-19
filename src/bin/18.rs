use std::collections::VecDeque;

use advent_of_code::util::{
    grid::Grid,
    point::{Point, ORTHOGONAL},
};
use rustc_hash::FxHashSet as HashSet;
use sscanf::scanf;

advent_of_code::solution!(18);

fn parse(input: &str, width: isize, height: isize, limit: usize) -> Option<Grid<char>> {
    let mut grid = Grid::with_default(width, height, '.');
    for line in input.lines().take(limit) {
        let (x, y) = scanf!(line, "{isize},{isize}").ok()?;
        grid[(x, y).into()] = '#';
    }
    Some(grid)
}

pub fn shortest_path(grid: &Grid<char>) -> Option<u32> {
    let start = Point(0, 0);
    let end = Point(grid.width - 1, grid.height - 1);

    let mut frontier = VecDeque::default();
    frontier.push_back((0, start));

    let mut visited = HashSet::default();
    visited.insert(start);

    while let Some((steps, point)) = frontier.pop_front() {
        if point == end {
            return Some(steps);
        }
        for next in ORTHOGONAL.map(|d| point + d) {
            if grid.in_bounds(next) && grid[next] != '#' && visited.insert(next) {
                frontier.push_back((steps + 1, next))
            };
        }
    }
    None
}

pub fn _part_one(input: &str, width: isize, height: isize, limit: usize) -> Option<u32> {
    let grid = parse(input, width, height, limit)?;
    shortest_path(&grid)
}

pub fn part_one(input: &str) -> Option<u32> {
    _part_one(input, 71, 71, 1024)
}

pub fn _part_two(input: &str, width: isize, height: isize, limit: usize) -> Option<Point> {
    let mut grid = parse(input, width, height, limit)?;
    let input = input
        .lines()
        .map(|line| scanf!(line, "{isize},{isize}").map(|pair| Point(pair.0, pair.1)))
        .collect::<Result<Vec<_>, _>>()
        .ok()?;
    let mut low = limit;
    let mut high = input.len();
    while low < high {
        let mid = (low + high) / 2;
        for p in low..=mid {
            grid[input[p]] = '#';
        }
        if shortest_path(&grid).is_some() {
            low = mid + 1;
        } else {
            for p in low..=mid {
                grid[input[p]] = '.';
            }
            high = mid;
        }
    }
    Some(input[low])
}

pub fn part_two(input: &str) -> Option<Point> {
    _part_two(input, 71, 71, 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(
            &advent_of_code::template::read_file("examples", DAY),
            7,
            7,
            12,
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = _part_two(
            &advent_of_code::template::read_file("examples", DAY),
            7,
            7,
            12,
        );
        assert_eq!(result, Some(Point(6, 1)));
    }
}
