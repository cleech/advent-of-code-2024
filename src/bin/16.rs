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

fn _part_one(map: &Maze) -> Option<u32> {
    // BinaryHeap as a priority queue
    let mut next = BinaryHeap::default();
    next.push((Reverse(0), map.start, RIGHT));

    let mut visited = Grid::with_default(map.grid.width, map.grid.height, [false; 4]);

    while let Some((Reverse(score), point, dir)) = next.pop() {
        if point == map.end {
            return Some(score);
        }
        let index = match dir {
            UP => 0,
            DOWN => 1,
            LEFT => 2,
            RIGHT => 3,
            _ => panic!(),
        };
        if visited[point][index] {
            continue;
        }
        visited[point][index] = true;

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

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse(input)?;
    let score = _part_one(&map)?;
    /*
        let mut g = map.grid.clone();
        for h in _history {
            g[h] = 'O';
        }
        println!("{g}");
    */
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input)?;

    // BinaryHeap as a priority queue
    let mut next = BinaryHeap::default();
    next.push((Reverse(0), map.start, RIGHT, vec![]));

    let mut best = u32::MAX;
    let mut visited = HashMap::default();
    let mut solutions = HashSet::default();

    while let Some((Reverse(score), point, dir, history)) = next.pop() {
        if point == map.end {
            if score < best {
                best = score;
            }
            if score == best {
                solutions.extend(history);
                solutions.insert(point);
            }
            continue;
        }

        if let Some(&oldscore) = visited.get(&(point, dir)) {
            if score != oldscore {
                continue;
            }
        } else {
            visited.insert((point, dir), score);
        }

        if map.grid[point + dir] != '#' {
            let mut h = history.clone();
            h.push(point);
            next.push((Reverse(score + 1), point + dir, dir, h));
        }
        for d in [dir.clockwise(), dir.counter_clockwise()] {
            if map.grid[point + d] != '#' {
                let mut h = history.clone();
                h.push(point);
                next.push((Reverse(score + 1001), point + d, d, h));
            }
        }
    }
    /*
        let mut g = map.grid.clone();
        for p in &solutions {
            g[*p] = 'O';
        }
        println!("{g}");
    */
    Some(solutions.len() as u32)
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
        assert_eq!(result, Some(45));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(64));
    }
}
