use advent_of_code::util::{grid::Grid, point::Point};
use itertools::Itertools;
use sscanf::{scanf, Error};
use std::cmp::Ordering::*;

advent_of_code::solution!(14);

type Robot = (i32, i32, i32, i32);

fn parse(input: &str) -> Result<Vec<Robot>, Error> {
    input
        .lines()
        .map(|l| scanf!(l, "p={i32},{i32} v={i32},{i32}"))
        .collect()
}

fn print_grid(robots: &[Robot], w: i32, h: i32) {
    let mut grid: Grid<char> = Grid::new(w as isize, h as isize, '⬛');
    for (px, py, _dx, _dy) in robots.iter() {
        grid[Point(*px as isize, *py as isize)] = '🤖';
    }
    for y in 0..grid.height {
        println!(
            "{}",
            &grid.raw[(y * grid.width) as usize..((y + 1) * (grid.width)) as usize]
                .iter()
                .collect::<String>()
        );
    }
    println!();
}

pub fn rmove((x, y, dx, dy): &Robot, t: i32, (w, h): (i32, i32)) -> Robot {
    (
        (x + dx * t).rem_euclid(w),
        (y + dy * t).rem_euclid(h),
        *dx,
        *dy,
    )
}

pub fn _part_one(input: &str, (w, h): (i32, i32)) -> Option<i32> {
    let robots = parse(input).ok()?;

    let robots = robots
        .iter()
        .map(|r| rmove(r, 100, (w, h)))
        .collect::<Vec<_>>();

    robots
        .iter()
        .map(|(x, y, _, _)| match (x.cmp(&(w / 2)), y.cmp(&(h / 2))) {
            (Equal, _) => (0, 0, 0, 0),
            (_, Equal) => (0, 0, 0, 0),
            (Less, Less) => (1, 0, 0, 0),
            (Greater, Less) => (0, 1, 0, 0),
            (Less, Greater) => (0, 0, 1, 0),
            (Greater, Greater) => (0, 0, 0, 1),
        })
        .reduce(|(a, b, c, d), (da, db, dc, dd)| (a + da, b + db, c + dc, d + dd))
        .map(|(a, b, c, d)| a * b * c * d)
}

pub fn part_one(input: &str) -> Option<i32> {
    _part_one(input, (101, 103))
}

pub fn part_two(input: &str) -> Option<i32> {
    let w = 101;
    let h = 103;

    let robots = parse(input).ok()?;

    let (tx, ty, _vx, _vy) = (0..w.max(h))
        .map(|t| {
            let (xs, ys): (Vec<i32>, Vec<i32>) = robots
                .iter()
                .map(|r| rmove(r, t, (w, h)))
                .map(|(x, y, _dx, _dy)| (x, y))
                .unzip();
            let mean_x = xs.iter().sum::<i32>() / xs.len() as i32;
            let mean_y = ys.iter().sum::<i32>() / ys.len() as i32;
            // variance of x and y position at time t
            let vx = xs.iter().map(|x| (mean_x - *x).pow(2)).sum::<i32>() / xs.len() as i32;
            let vy = ys.iter().map(|y| (mean_y - *y).pow(2)).sum::<i32>() / ys.len() as i32;
            (t, t, vx, vy)
        })
        .fold(
            (0, 0, i32::MAX, i32::MAX),
            |(tx, ty, mvx, mvy), (t, _, vx, vy)| match (vx < mvx, vy < mvy) {
                (false, false) => (tx, ty, mvx, mvy),
                (true, false) => (t, ty, vx, mvy),
                (false, true) => (tx, t, mvy, vy),
                (true, true) => (t, t, vx, vy),
            },
        );
    // dbg!(tx, ty, vx, vy);

    // find t where (tx + t*h) % (w*h) == ty + t*h) % (w*h)
    // x = 101t + zx, y = 103t + zy, x=y % (10403)
    // 51 = 101^-1 mod 103 = 103^-1 mod 101
    // 51 = 101^-1 % 103 = 103^-1 % 101
    // 51 * 101 = 5151, 51 * 103 = 5253
    let t = (5253 * tx + 5151 * ty) % 10403;

    let r = robots
        .iter()
        .map(|r| rmove(r, t, (w, h)))
        .collect::<Vec<_>>();
    print_grid(&r, w, h);

    Some(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = _part_one(
            &advent_of_code::template::read_file("examples", DAY),
            (11, 7),
        );
        assert_eq!(result, Some(12));
    }
}
