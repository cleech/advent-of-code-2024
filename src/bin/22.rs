advent_of_code::solution!(22);

use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::iter::successors;

fn hash(mut n: i64) -> i64 {
    n = (n ^ (n << 6)) & 0x0ffffff;
    n = (n ^ (n >> 5)) & 0x0ffffff;
    (n ^ (n << 11)) & 0x0ffffff
}

pub fn part_one(input: &str) -> Option<i64> {
    let monkeys = input
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .and_then(|n| Ok(successors(Some(n), |&n| Some(hash(n)))))
                .ok()
        })
        .collect::<Option<Vec<_>>>()?;

    monkeys.into_par_iter().map(|i| i.skip(2000).next()).sum()
}

pub fn part_two(input: &str) -> Option<i64> {
    let monkeys = input
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .and_then(|n| Ok(successors(Some(n), |&n| Some(hash(n)))))
                .ok()
        })
        .collect::<Option<Vec<_>>>()?;

    let monkeys = monkeys.into_iter().map(|m| {
        let mut prices = FxHashMap::default();
        m.take(2001)
            .tuple_windows()
            .map(|(a, b)| (b % 10, (b % 10) - (a % 10)))
            .tuple_windows()
            .map(|(a, b, c, d)| (d.0, (a.1, b.1, c.1, d.1)))
            .for_each(|x| {
                prices.entry(x.1).or_insert(x.0);
            });
        prices
    });

    let mut prices = FxHashMap::default();
    for m in monkeys {
        for (pattern, price) in m {
            prices
                .entry(pattern)
                .and_modify(|p| *p += price)
                .or_insert(price);
        }
    }
    prices.values().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
