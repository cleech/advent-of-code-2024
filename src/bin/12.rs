advent_of_code::solution!(12);

use advent_of_code::util::grid::*;
use advent_of_code::util::point::*;
use rustc_hash::FxHashSet as HashSet;

#[derive(Debug)]
struct Region<T>(T, Vec<Point>);

impl<T> Region<T>
where
    T: std::fmt::Debug + std::fmt::Display,
{
    #[inline]
    fn area(&self) -> usize {
        self.1.len()
    }

    fn perimeter(&self) -> usize {
        let set = HashSet::from_iter(self.1.iter().cloned());
        let mut acc = 0;
        for &p in self.1.iter() {
            for n in ORTHOGONAL.map(|d| p + d) {
                if !set.contains(&n) {
                    acc += 1;
                }
            }
        }
        acc
    }

    fn sides(&self) -> usize {
        // println!("----- {} -----", self.0);
        let set = HashSet::from_iter(self.1.iter().cloned());
        let mut vedges = vec![];
        let mut hedges = vec![];

        for &p in self.1.iter() {
            for d in [UP, DOWN, RIGHT, LEFT] {
                let n = p + d;
                if !set.contains(&n) {
                    match d {
                        UP => hedges.push((p.0, (n.1, p.1), UP)),
                        DOWN => hedges.push((p.0, (p.1, n.1), DOWN)),
                        RIGHT => vedges.push(((p.0, n.0), p.1, RIGHT)),
                        LEFT => vedges.push(((n.0, p.0), p.1, LEFT)),
                        _ => panic!(),
                    }
                }
            }
        }

        assert!(hedges.len() + vedges.len() == self.perimeter());

        hedges.sort_by_key(|he| (he.1 .0, he.0));
        hedges.reverse();
        vedges.sort_by_key(|ve| (ve.0 .0, ve.1));
        vedges.reverse();

        assert!(hedges.len() + vedges.len() == self.perimeter());

        // println!("----hedges----");
        let mut h2 = vec![];
        {
            let mut left = hedges.pop().unwrap();
            let mut acc = vec![];
            while let Some(right) = hedges.pop() {
                if (left.0 + 1 == right.0) && (left.1 .0 == right.1 .0) && (left.2 == right.2) {
                    acc.push(left);
                } else {
                    acc.push(left);
                    h2.push(acc);
                    acc = vec![];
                }
                left = right;
            }
            acc.push(left);
            h2.push(acc);
        }
        /*
                for segment in &h2 {
                    println!("{:?}", segment);
                }
        */

        // println!("----vedges----");
        let mut v2 = vec![];
        {
            let mut top = vedges.pop().unwrap();
            let mut acc = vec![];
            while let Some(bottom) = vedges.pop() {
                if (top.1 + 1 == bottom.1) && (top.0 .0 == bottom.0 .0) && (top.2 == bottom.2) {
                    acc.push(top);
                } else {
                    acc.push(top);
                    v2.push(acc);
                    acc = vec![];
                }
                top = bottom;
            }
            acc.push(top);
            v2.push(acc);
        }
        /*
                for segment in &v2 {
                    println!("{:?}", segment);
                }
        */

        assert!(h2.iter().flatten().count() + v2.iter().flatten().count() == self.perimeter());
        h2.len() + v2.len()
    }
}

fn flood<T>(grid: &Grid<T>) -> Vec<Region<T>>
where
    T: Copy + PartialEq,
{
    let mut visited = HashSet::default();
    let mut regions = vec![];

    for start in grid.points() {
        if visited.contains(&start) {
            continue;
        }
        let t = grid[start];
        let mut next = vec![start];
        let mut r = vec![];
        while let Some(p) = next.pop() {
            if !grid.in_bounds(p) || grid[p] != t || visited.contains(&p) {
                continue;
            }
            visited.insert(p);
            r.push(p);
            next.extend_from_slice(&ORTHOGONAL.map(|d| p + d));
        }
        regions.push(Region(t, r));
    }
    regions
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = Grid::parse::<char>(input).ok()?;
    let regions = flood(&grid);
    Some(regions.iter().map(|r| r.area() * r.perimeter()).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = Grid::parse::<char>(input).ok()?;
    let regions = flood(&grid);
    Some(regions.iter().map(|r| r.area() * r.sides()).sum())
}

/* faster solution for both parts
* edges and corners counted during region finding flood fill
* does not save off any region structures
*/
pub fn _solve(input: &str) -> Option<usize> {
    let grid = Grid::parse::<char>(input).ok()?;
    let mut visited = HashSet::default();
    let mut cost = 0;
    let mut cost2 = 0;

    for start in grid.points() {
        if visited.contains(&start) {
            continue;
        }
        let t = grid[start];
        let mut next = vec![start];
        let mut area = 0;
        let mut perimeter = 0;
        let mut corners = 0;

        let check = |p| grid.in_bounds(p) && grid[p] == t;

        while let Some(p) = next.pop() {
            if visited.contains(&p) {
                continue;
            }
            visited.insert(p);
            area += 1;
            for d in ORTHOGONAL {
                if check(p + d) {
                    next.push(p + d);
                } else {
                    /* found an edge */
                    perimeter += 1;
                    /* check for corners */
                    let r = d.clockwise();
                    if !check(p + r) || check(p + r + d) {
                        corners += 1;
                    }
                    let l = d.counter_clockwise();
                    if !check(p + l) || check(p + l + d) {
                        corners += 1;
                    }
                }
            }
        }
        /* each corner is found twice */
        corners /= 2;
        cost += area * perimeter;
        cost2 += area * corners;
    }
    println!("part1: {}", cost);
    println!("part2: {}", cost2);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));

        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(368));
    }
}
