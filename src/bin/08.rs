advent_of_code::solution!(8);

use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;
use rustc_hash::FxHashSet as HashSet;

fn parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(i32, i32)>>) {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut antenna = HashMap::default();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            let c = grid[y][x];
            if c.is_ascii_alphanumeric() {
                antenna
                    .entry(c)
                    .or_insert(vec![])
                    .push((x as i32, y as i32));
            }
        }
    }
    (grid, antenna)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, antenna) = parse_input(input);
    let mut antinodes = HashSet::default();

    for (_a, pos) in antenna {
        pos.iter().combinations(2).for_each(|pair| {
            let dx = pair[0].0 - pair[1].0;
            let dy = pair[0].1 - pair[1].1;
            let an1 = (pair[0].0 + dx, pair[0].1 + dy);
            let an2 = (pair[1].0 - dx, pair[1].1 - dy);
            if an1.0 >= 0 && an1.0 < grid[0].len() as i32 && an1.1 >= 0 && an1.1 < grid.len() as i32
            {
                // grid[an1.1 as usize][an1.0 as usize] = '#';
                antinodes.insert(an1);
            }
            if an2.0 >= 0 && an2.0 < grid[0].len() as i32 && an2.1 >= 0 && an2.1 < grid.len() as i32
            {
                // grid[an2.1 as usize][an2.0 as usize] = '#';
                antinodes.insert(an2);
            }
        });
    }
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (grid, antenna) = parse_input(input);
    let mut antinodes = HashSet::default();

    for (_a, pos) in antenna {
        pos.iter().combinations(2).for_each(|pair| {
            let dx = pair[0].0 - pair[1].0;
            let dy = pair[0].1 - pair[1].1;
            let mut n = 0;
            loop {
                let an1 = (pair[0].0 + n * dx, pair[0].1 + n * dy);
                if an1.0 >= 0
                    && an1.0 < grid[0].len() as i32
                    && an1.1 >= 0
                    && an1.1 < grid.len() as i32
                {
                    // grid[an1.1 as usize][an1.0 as usize] = '#';
                    antinodes.insert(an1);
                } else {
                    break;
                }
                n += 1;
            }
            n = 0;
            loop {
                let an2 = (pair[1].0 - n * dx, pair[1].1 - n * dy);
                if an2.0 >= 0
                    && an2.0 < grid[0].len() as i32
                    && an2.1 >= 0
                    && an2.1 < grid.len() as i32
                {
                    // grid[an2.1 as usize][an2.0 as usize] = '#';
                    antinodes.insert(an2);
                } else {
                    break;
                }
                n += 1;
            }
        });
    }
    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
