use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

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

pub fn __part_one(grid: &Grid<char>) -> Option<u32> {
    let start = Point(0, 0);
    let end = Point(grid.width - 1, grid.height - 1);

    let mut visited = HashSet::default();

    let mut frontier = BinaryHeap::default();
    frontier.push((Reverse(0), start));

    while let Some((Reverse(steps), point)) = frontier.pop() {
        if point == end {
            return Some(steps);
        }
        if visited.contains(&point) {
            continue;
        }
        visited.insert(point);
        for next in ORTHOGONAL.map(|d| point + d) {
            if grid.in_bounds(next) && grid[next] != '#' {
                frontier.push((Reverse(steps + 1), next));
            }
        }
    }
    None
}

pub fn _part_one(input: &str, width: isize, height: isize, limit: usize) -> Option<u32> {
    let grid = parse(input, width, height, limit)?;
    __part_one(&grid)
}

pub fn part_one(input: &str) -> Option<u32> {
    _part_one(input, 71, 71, 1024)
}

pub fn _part_two(input: &str, width: isize, height: isize, limit: usize) -> Option<String> {
    for n in (limit + 1)..=input.lines().count() {
        let grid = parse(input, width, height, n)?;
        if __part_one(&grid) == None {
            return Some(input.lines().nth(n - 1)?.to_string());
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
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
        assert_eq!(result, Some("6,1".to_string()));
    }
}
