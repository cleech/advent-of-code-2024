advent_of_code::solution!(15);

use advent_of_code::util::{grid::*, point::*};
use rustc_hash::FxHashSet;

const ROBOT: char = '@';
const BOX: char = 'O';
const BOX_LEFT: char = '[';
const BOX_RIGHT: char = ']';
const WALL: char = '#';
const SPACE: char = '.';

fn parse(input: &str) -> Option<(Grid<char>, Vec<Point>)> {
    let (prefix, suffix) = input.split_once("\n\n")?;
    let grid = Grid::parse::<char>(prefix).ok()?;
    let moves: Vec<Point> = suffix
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => Some(UP),
            'v' => Some(DOWN),
            '>' => Some(RIGHT),
            '<' => Some(LEFT),
            _ => None,
        })
        .collect::<Option<Vec<Point>>>()?;
    Some((grid, moves))
}

fn gps(map: &Grid<char>, needle: char) -> isize {
    let mut gps = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            if map[Point(x, y)] == needle {
                gps += 100 * y + x;
            }
        }
    }
    gps
}

fn _part_one(map: &mut Grid<char>, robot: &mut Point, m: Point) {
    let mut p = *robot + m;
    while map[p] == BOX || map[p] == BOX_LEFT || map[p] == BOX_RIGHT {
        p += m;
    }
    if map[p] == WALL {
        return;
    }
    assert!(map[p] == SPACE);
    while p != *robot {
        map[p] = map[p - m];
        p -= m;
    }
    map[p] = SPACE;
    *robot += m;
}

pub fn part_one(input: &str) -> Option<isize> {
    let (mut map, moves) = parse(input)?;
    let mut robot = map.find(ROBOT)?;

    for m in moves {
        // println!("{map}");
        _part_one(&mut map, &mut robot, m);
    }
    // println!("{map}");
    Some(gps(&map, BOX))
}

fn parse2(input: &str) -> Option<(Grid<char>, Vec<Point>)> {
    let (gtmp, moves) = parse(input)?;
    let grid = Grid {
        width: gtmp.width * 2,
        height: gtmp.height,
        raw: gtmp
            .raw
            .iter()
            .flat_map(|c| match *c {
                ROBOT => [ROBOT, SPACE],
                WALL => [WALL, WALL],
                SPACE => [SPACE, SPACE],
                BOX => [BOX_LEFT, BOX_RIGHT],
                _ => panic!(),
            })
            .collect(),
    };
    Some((grid, moves))
}

fn _part_two(map: &mut Grid<char>, robot: &mut Point, m: Point) {
    let p = *robot + m;
    let mut boxes = FxHashSet::default();
    let mut new = vec![];
    // check the first box
    if map[p] == BOX_LEFT {
        assert!(map[p + RIGHT] == BOX_RIGHT);
        boxes.insert(p);
        new.push((p, p + RIGHT));
    } else if map[p] == BOX_RIGHT {
        assert!(map[p + LEFT] == BOX_LEFT);
        boxes.insert(p + LEFT);
        new.push((p + LEFT, p));
    } else {
        panic!();
    }
    // let's get to pushing boxes
    while let Some(b) = new.pop() {
        if map[b.0 + m] == WALL || map[b.1 + m] == WALL {
            // whole thing is blocked by a wall
            return;
        }
        // alligned
        if map[b.0 + m] == BOX_LEFT {
            assert!(map[b.1 + m] == BOX_RIGHT);
            if boxes.insert(b.0 + m) {
                new.push((b.0 + m, b.1 + m));
            } else {
                panic!();
            }
        }
        // left offset
        if map[b.0 + m] == BOX_RIGHT {
            assert!(map[b.0 + m + LEFT] == BOX_LEFT);
            if boxes.insert(b.0 + m + LEFT) {
                new.push((b.0 + m + LEFT, b.0 + m));
            } else {
                panic!();
            }
        }
        // right offset
        if map[b.1 + m] == BOX_LEFT {
            assert!(map[b.1 + m + RIGHT] == BOX_RIGHT);
            //if map[b.1 + RIGHT] == BOX_LEFT {
            //    panic!()
            //}
            if boxes.insert(b.1 + m) {
                new.push((b.1 + m, b.1 + m + RIGHT));
            } else {
                // panic!();
            }
        }
    }
    // found all the boxes to move
    // just clear them all first
    for &b in &boxes {
        map[b] = SPACE;
        map[b + RIGHT] = SPACE;
    }
    // then place the new boxes
    for &b in &boxes {
        map[b + m] = BOX_LEFT;
        map[b + m + RIGHT] = BOX_RIGHT;
    }
    // done
    map[*robot] = SPACE;
    *robot += m;
    map[*robot] = ROBOT;
}

pub fn part_two(input: &str) -> Option<isize> {
    let (mut map, moves) = parse2(input)?;
    let mut robot = map.find(ROBOT)?;

    // println!("Initial State:");
    for m in moves {
        /*
                println!("{map}");
                println!(
                    "Move {}:",
                    match m {
                        UP => '^',
                        DOWN => 'v',
                        LEFT => '<',
                        RIGHT => '>',
                        _ => panic!(),
                    }
                );
        */
        let p = robot + m;
        // easy cases, just like part 1
        if m == LEFT || m == RIGHT || map[p] == SPACE || map[p] == WALL {
            _part_one(&mut map, &mut robot, m);
        } else {
            _part_two(&mut map, &mut robot, m);
        }
    }
    // println!("{map}");
    Some(gps(&map, BOX_LEFT))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2028));
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9021));
    }
}
