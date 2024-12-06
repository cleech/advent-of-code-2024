advent_of_code::solution!(6);

use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum DIR {
    Up,
    Down,
    Left,
    Right,
}
use DIR::*;

fn visited(grid: &Vec<Vec<char>>) -> (HashSet<((usize, usize), DIR)>, bool) {
    let mut pos = (0, 0);
    let mut dir = DIR::Up;

    // find the starting position
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '^' {
                pos = (x, y);
            }
        }
    }

    let mut visited = HashSet::new();
    loop {
        if !visited.insert((pos, dir)) {
            // looping
            return (visited, false);
        }
        let (x, y) = match dir {
            Up => {
                if pos.1 == 0 {
                    break;
                }
                (pos.0, pos.1 - 1)
            }
            Down => {
                if pos.1 == grid[0].len() {
                    break;
                }
                (pos.0, pos.1 + 1)
            }
            Left => {
                if pos.0 == 0 {
                    break;
                }
                (pos.0 - 1, pos.1)
            }
            Right => {
                if pos.0 == grid.len() {
                    break;
                }
                (pos.0 + 1, pos.1)
            }
        };
        let c = grid.get(y).and_then(|row| row.get(x));
        if c.is_none() {
            break;
        }
        match c.unwrap() {
            '#' => {
                dir = match dir {
                    Up => Right,
                    Right => Down,
                    Down => Left,
                    Left => Up,
                };
                continue;
            }
            _ => pos = (x, y),
        }
    }
    (visited, true)
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    // Some(visited(&grid).0.len())
    Some(
        visited(&grid)
            .0
            .into_iter()
            .map(|((x, y), _)| (x, y))
            .collect::<HashSet<_>>()
            .len(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);
    let p1 = visited(&grid)
        .0
        .into_iter()
        .map(|((x, y), _)| (x, y))
        .collect::<HashSet<_>>()
        .into_iter();
    let mut count = 0;
    for (x, y) in p1 {
        let c = grid[y][x];
        grid[y][x] = '#';
        let exited = visited(&grid).1;
        if !(exited) {
            count += 1
        }
        grid[y][x] = c;
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
