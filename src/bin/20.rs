advent_of_code::solution!(20);

use advent_of_code::util::{grid::*, point::*};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Node {
    coord: Point,
    distance: usize,
}

// hacked on BFS that tracks the single race path
fn get_path(grid: &Grid<char>) -> Option<Vec<Node>> {
    let start = grid.find('S')?;
    let end = grid.find('E')?;

    let mut frontier = VecDeque::default();
    frontier.push_back(Node {
        coord: start,
        distance: 0,
    });
    let mut path: Vec<Node> = vec![];

    while let Some(
        node @ Node {
            coord: point,
            distance,
        },
    ) = frontier.pop_front()
    {
        let prev = path.last().map(|n| n.coord).unwrap_or(Point(0, 0));
        path.push(node);
        if point == end {
            return Some(path);
        }
        for next in ORTHOGONAL.map(|d| point + d) {
            if grid.in_bounds(next) && grid[next] != '#' && next != prev {
                frontier.push_back(Node {
                    coord: next,
                    distance: distance + 1,
                });
            }
        }
    }
    None
}

pub fn day20(input: &str, cheat_len: usize) -> Option<usize> {
    let grid: Grid<char> = Grid::parse(input).ok()?;
    let path = get_path(&grid)?;

    // Binary Space Partitioning (sort of, maybe)
    // For every point on the path, create a list of every other point sorted
    // by manhatten distance. This will let us do a binary partition to locate
    // all possible end-points within a cheat range.
    let bsp = path
        .iter()
        .map(|a| {
            let mut list = path[a.distance..]
                .iter()
                .map(|b| (b.clone(), a.coord.distance(&b.coord)))
                .collect::<Vec<_>>();
            list.sort_by_key(|n| n.1);
            list
        })
        .collect::<Vec<_>>();

    // find all cheats which save at least 100 steps
    let mut count = 0;
    for a in &path {
        let bsp = &bsp[a.distance];
        let high = bsp.partition_point(|b| b.1 < cheat_len + 1);
        let low = bsp[..high].partition_point(|b| b.1 < 2);
        for cheat in &bsp[low..high] {
            let saved = cheat.0.distance - a.distance - cheat.1;
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
