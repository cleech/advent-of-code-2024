advent_of_code::solution!(20);

use advent_of_code::util::{grid::*, point::*};

// Path follower for the single track.
// This only works because there are no branches.
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

// there's no start, because we store Cheats by start position anyway
struct Cheat {
    // end point of the cheat,
    // as an index into path (or distance from the start)
    end: usize,
    // manhatten distance from the cheat start to end
    // or cheat time cost to apply to time saved calculation
    distance: usize,
}

pub fn day20(input: &str, cheat_len: usize) -> Option<usize> {
    let grid: Grid<char> = Grid::parse(input).ok()?;
    let path: Vec<Point> = get_path(&grid)?;

    // Binary Space Partitioning (sort of, maybe?)
    // For every point on the path, create a list of every other point (saved as
    // an index on the path, no longer a Point in the Grid) sorted by manhatten
    // distance on the Grid. These are potential Cheats.
    // This will let us do a binary partition to locate all possible end-points
    // within a cheat range.
    let bsp: Vec<Vec<Cheat>> = path
        .iter()
        .enumerate()
        // enumeration == path index == time from start
        .map(|(ia, a)| {
            // only look forward in the path
            // there's no point in cheating backwards
            let mut list: Vec<Cheat> = path[ia..]
                .iter()
                .enumerate()
                .map(|(ib, b)| Cheat {
                    end: ia + ib,
                    distance: a.manhatten_distance(&b),
                })
                .collect();
            list.sort_by_key(|n| n.distance);
            list
        })
        .collect();

    // find all cheats which save at least 100 steps
    let mut count = 0;
    for (ia, _a) in path.iter().enumerate() {
        let bsp = &bsp[ia];
        let high = bsp.partition_point(|cheat| cheat.distance < cheat_len + 1);
        let low = bsp[..high].partition_point(|cheat| cheat.distance < 2);
        for cheat in &bsp[low..high] {
            // replace the path distance between start and end point with the
            // grid distance ignoring obstructions to calculate time saved
            let saved = cheat.end - ia - cheat.distance;
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
