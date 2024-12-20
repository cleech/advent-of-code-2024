advent_of_code::solution!(20);

use advent_of_code::util::{grid::*, point::*};

fn get_path(grid: &Grid<char>) -> Option<Vec<Point>> {
    let start = grid.find('S')?;
    let end = grid.find('E')?;

    let mut path: Vec<Point> = vec![start];
    // special case start, it can move in any direction
    let mut direction = ORTHOGONAL.into_iter().find(|d| grid[start + *d] != '#')?;
    let mut position = start + direction;
    while position != end {
        path.push(position);
        // for the rest of the path, don't backtrack
        direction = [
            direction,
            direction.clockwise(),
            direction.counter_clockwise(),
        ]
        .into_iter()
        .find(|&d| grid[position + d] != '#')?;
        position += direction;
    }
    path.push(end);
    Some(path)
}

pub fn day20(input: &str, cheat_len: usize) -> Option<usize> {
    let grid: Grid<char> = Grid::parse(input).ok()?;
    let path = get_path(&grid)?;

    // Binary Space Partitioning (sort of, maybe)
    // For every point on the path, create a list of every other point (as an index to the path,
    // which is equal to the distance from the start) sorted
    // by manhatten distance. This will let us do a binary partition to locate
    // all possible end-points within a cheat range.
    let bsp = path
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let mut list = path[i..]
                .iter()
                .enumerate()
                .map(|(j, b)| (i + j, a.distance(&b)))
                .collect::<Vec<_>>();
            list.sort_by_key(|n| n.1);
            list
        })
        .collect::<Vec<_>>();

    // find all cheats which save at least 100 steps
    let mut count = 0;
    for (i, a) in path.iter().enumerate() {
        let bsp = &bsp[i];
        let high = bsp.partition_point(|b| b.1 < cheat_len + 1);
        let low = bsp[..high].partition_point(|b| b.1 < 2);
        for cheat in &bsp[low..high] {
            let saved = cheat.0 - i - cheat.1;
            if saved >= 100 {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_one(input: &str) -> Option<usize> {
    day20(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    day20(input, 20)
}
