advent_of_code::solution!(11);

use gxhash::{HashMap, HashMapExt};
use rayon::prelude::*;
use std::iter::once;

fn _solve(input: &str, loops: usize) -> Option<usize> {
    let mut stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..loops {
        stones = stones
            .into_iter()
            .flat_map(|s| {
                let len = s.checked_ilog10().unwrap_or(0) + 1;
                if s == 0 {
                    Box::new(once(1_u64)) as Box<dyn Iterator<Item = u64>>
                } else if len % 2 == 0 {
                    let n = 10_u64.pow(len / 2);
                    let a = s / n;
                    let b = s % n;
                    Box::new(once(a).chain(once(b))) as Box<dyn Iterator<Item = u64>>
                } else {
                    Box::new(once(s * 2024_u64)) as Box<dyn Iterator<Item = u64>>
                }
            })
            .collect::<Vec<_>>();
        // dbg!(&stones);
    }
    Some(stones.len())
}

fn blink(stone: u64, count: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if count == 0 {
        1
    } else if let Some(&value) = cache.get(&(stone, count)) {
        value
    } else {
        let len = stone.checked_ilog10().unwrap_or(0) + 1;
        let next = if stone == 0 {
            blink(1, count - 1, cache)
        } else if len % 2 == 0 {
            let n = 10_u64.pow(len / 2);
            let a = stone / n;
            let b = stone % n;
            blink(a, count - 1, cache) + blink(b, count - 1, cache)
        } else {
            blink(stone * 2024, count - 1, cache)
        };
        cache.insert((stone, count), next);
        next
    }
}

fn solve2(input: &str, loops: usize) -> Option<usize> {
    let mut cache = HashMap::with_capacity(150_000);
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let count = stones.iter().map(|&s| blink(s, loops, &mut cache)).sum();
    Some(count)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve2(input, 25)
}
pub fn part_two(input: &str) -> Option<usize> {
    solve2(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }
}
