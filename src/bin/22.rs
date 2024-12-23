advent_of_code::solution!(22);

use itertools::Itertools;
use rayon::prelude::*;
use rustc_hash::FxHashMap as HashMap;
use std::{
    iter::successors,
    ops::{BitAnd, BitXor, Shl, Shr},
};

fn hash<T>(mut n: T) -> T
where
    T: Shl<i32, Output = T>
        + Shr<i32, Output = T>
        + BitAnd<T, Output = T>
        + BitXor<T, Output = T>
        + From<i32>
        + Copy,
{
    let mask: T = 0x0ffffff.into();
    n = (n ^ (n << 6)) & mask;
    n = (n ^ (n >> 5)) & mask;
    (n ^ (n << 11)) & mask
}

pub fn part_one(input: &str) -> Option<i64> {
    // Vec<impl Iterator<Item = i32>>
    let monkeys = input
        .lines()
        .map(|line| {
            line.parse::<i32>()
                .map(|n| successors(Some(n), |&n| Some(hash(n))))
                .ok()
        })
        .collect::<Option<Vec<_>>>()?;

    monkeys
        .into_par_iter()
        .map(|mut i| i.nth(2000).map(|n| n as i64))
        .sum()
}

// pack 4 signed single digit values into one u32 for a hash key
fn pack(a: i32, b: i32, c: i32, d: i32) -> u32 {
    ((a + 9) << 18 | (b + 9) << 12 | (c + 9) << 6 | (d + 9)) as u32
}

pub fn part_two(input: &str) -> Option<i32> {
    // Vec<impl Iterator<Item = i32>>
    let monkeys = input
        .lines()
        .map(|line| {
            line.parse::<i32>()
                .map(|n| successors(Some(n), |&n| Some(hash(n))))
                .ok()
        })
        .collect::<Option<Vec<_>>>()?;

    let monkeys: Vec<HashMap<u32, i32>> = monkeys
        .into_par_iter()
        .map(|m| {
            let mut prices = HashMap::default();
            m.take(2001)
                .tuple_windows()
                .map(|(a, b)| (b % 10, (b % 10) - (a % 10)))
                .tuple_windows()
                .map(|(a, b, c, d)| (d.0, pack(a.1, b.1, c.1, d.1)))
                .for_each(|x| {
                    prices.entry(x.1).or_insert(x.0);
                });
            prices
        })
        .collect::<Vec<_>>();

    let mut prices = HashMap::default();
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
