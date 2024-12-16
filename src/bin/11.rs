advent_of_code::solution!(11);

use rustc_hash::FxBuildHasher;
use std::collections::HashMap;
use std::hash::BuildHasher;

// this gets too big
/*
use std::iter::once;
fn solve(input: &str, loops: usize) -> Option<usize> {
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
    }
    Some(stones.len())
}
*/

fn _blink<T>(stone: u64, count: usize, cache: &mut HashMap<(u64, usize), usize, T>) -> usize
where
    T: BuildHasher,
{
    if count == 0 {
        1
    } else if let Some(&value) = cache.get(&(stone, count)) {
        value
    } else {
        let len = stone.checked_ilog10().unwrap_or(0) + 1;
        let next = if stone == 0 {
            _blink(1, count - 1, cache)
        } else if len % 2 == 0 {
            let n = 10_u64.pow(len / 2);
            let a = stone / n;
            let b = stone % n;
            _blink(a, count - 1, cache) + _blink(b, count - 1, cache)
        } else {
            _blink(stone * 2024, count - 1, cache)
        };
        cache.insert((stone, count), next);
        next
    }
}

fn _solve2(input: &str, loops: usize) -> Option<usize> {
    let stones = input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let mut cache = HashMap::with_capacity_and_hasher(150_000, FxBuildHasher);
    let count = stones.iter().map(|&s| _blink(s, loops, &mut cache)).sum();
    Some(count)
}

fn solve3(input: &str, loops: usize) -> Option<usize> {
    let mut stones = HashMap::with_capacity_and_hasher(5000, FxBuildHasher);
    input
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .for_each(|s| {
            stones
                .entry(s)
                .and_modify(|count| *count += 1)
                .or_insert(1_usize);
        });

    let mut next = HashMap::with_capacity_and_hasher(5000, FxBuildHasher);

    for _ in 0..loops {
        for (&stone, &count) in &stones {
            let len = stone.checked_ilog10().unwrap_or(0) + 1;
            if stone == 0 {
                next.entry(1).and_modify(|c| *c += count).or_insert(count);
            } else if len % 2 == 0 {
                let n = 10_u64.pow(len / 2);
                let a = stone / n;
                let b = stone % n;
                next.entry(a).and_modify(|c| *c += count).or_insert(count);
                next.entry(b).and_modify(|c| *c += count).or_insert(count);
            } else {
                next.entry(stone * 2024)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            };
        }
        (stones, next) = (next, stones);
        next.clear();
    }
    Some(stones.values().sum())
}

pub fn part_one(input: &str) -> Option<usize> {
    solve3(input, 25)
}
pub fn part_two(input: &str) -> Option<usize> {
    solve3(input, 75)
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
