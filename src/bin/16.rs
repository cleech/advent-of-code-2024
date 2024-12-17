advent_of_code::solution!(16);

use advent_of_code::util::{grid::Grid, point::*};
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use std::{cmp::Reverse, collections::BinaryHeap};

struct Maze {
    grid: Grid<char>,
    start: Point,
    end: Point,
}

fn parse(input: &str) -> Option<Maze> {
    let grid = Grid::parse::<char>(input).ok()?;
    let start = grid.find('S')?;
    let end = grid.find('E')?;
    Some(Maze { grid, start, end })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input)?;

    // BinaryHeap as a priority queue
    let mut next = BinaryHeap::default();
    next.push((Reverse(0), map.start, RIGHT));

    let mut visited = HashSet::default();

    while let Some((Reverse(score), point, dir)) = next.pop() {
        if point == map.end {
            return Some(score);
        }
        if !visited.insert((point, dir)) {
            continue;
        }
        map.grid[point] = 'O';
        if map.grid[point + dir] != '#' {
            next.push((Reverse(score + 1), point + dir, dir));
        }
        for d in [dir.clockwise(), dir.counter_clockwise()] {
            if map.grid[point + d] != '#' {
                next.push((Reverse(score + 1001), point + d, d));
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse(input)?;

    // BinaryHeap as a priority queue
    let mut next = BinaryHeap::default();
    next.push((Reverse(0), map.start, RIGHT));

    let mut visited = HashMap::default();
    // let mut best = u32::MAX;
    let mut best = part_one(input)?;
    let mut solutions = 0;

    while let Some((Reverse(score), point, dir)) = next.pop() {
        if score > best {
            continue;
        }
        if point == map.end {
            if score < best {
                best = score;
            }
            solutions += 1;
            continue;
        }

        if let Some(&oldscore) = visited.get(&(point, dir)) {
            if oldscore > score {
                continue;
            } else {
                visited
                    .entry((point, dir))
                    .and_modify(|s| *s = score.min(*s));
            }
        } else {
            visited.insert((point, dir), score);
        }

        if map.grid[point + dir] != '#' {
            next.push((Reverse(score + 1), point + dir, dir));
        }
        for d in [dir.clockwise(), dir.counter_clockwise()] {
            if map.grid[point + d] != '#' {
                next.push((Reverse(score + 1001), point + d, d));
            }
        }
    }
    println!("{}", map.grid);
    println!("{solutions} solutions are optimal");
    Some(solutions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(7036));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        // assert_eq!(result, Some(7036));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        // assert_eq!(result, Some(11048));
    }
}
