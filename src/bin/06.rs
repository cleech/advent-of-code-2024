advent_of_code::solution!(6);

// use rayon::prelude::*;
use std::thread;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(Vec::from).collect()
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
use Dir::*;
/*
struct Grid {
    raw: Vec<Vec<u8>>,
    pos: (usize, usize),
}

impl Grid {
    fn from_str(input: &str) -> Grid {
        Grid {
            raw: input.lines().map(Vec::from).collect(),
            pos: (0, 0),
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.raw[y][x]
    }
}
*/

fn visited(grid: &[Vec<u8>]) -> Option<Vec<Vec<[bool; 4]>>> {
    let mut pos = (0, 0);
    let mut dir = Up;

    // find the starting position
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == b'^' {
                pos = (x, y);
            }
        }
    }

    let mut visited = vec![vec![[false; 4]; grid[0].len()]; grid[0].len()];
    loop {
        if visited[pos.1][pos.0][dir as usize] {
            // looping
            return None;
        } else {
            visited[pos.1][pos.0][dir as usize] = true;
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
            b'#' => {
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
    Some(visited)
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let s = visited(&grid)?;
    Some(
        s.iter()
            .flat_map(|row| row.iter())
            .filter(|x| x.iter().any(|b| *b))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = parse_input(input);
    let _p1 = visited(&grid)?;

    let p1: Vec<(usize, usize, [bool; 4])> = _p1
        .into_iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.into_iter()
                .enumerate()
                .map(move |(x, dirs)| (x, y, dirs))
        })
        .collect();

    let mut count = 0;
    let cpus = thread::available_parallelism().unwrap().get();

    thread::scope(|s| {
        let threads: Vec<_> = p1
            .chunks(p1.len().div_ceil(cpus))
            .map(|chunk| {
                let mut test = grid.clone();
                s.spawn(move || -> usize {
                    chunk
                        .iter()
                        .filter(|(x, y, dirs)| {
                            if !dirs.iter().any(|&b| b) {
                                return false;
                            }
                            // let mut test = grid.clone();
                            let saved = test[*y][*x];
                            test[*y][*x] = b'#';
                            let exited = visited(&test).is_some();
                            test[*y][*x] = saved;
                            !exited
                        })
                        .count()
                })
            })
            .collect();
        count += threads
            .into_iter()
            .map(|t| t.join().unwrap())
            .sum::<usize>();
    });
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
