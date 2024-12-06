advent_of_code::solution!(1);

use std::collections::HashMap;
use std::iter::zip;

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .unzip::<_, _, Vec<_>, Vec<_>>();

    left.sort();
    right.sort();
    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    Some(zip(&left, &right).map(|(&x, &y)| x.abs_diff(y)).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_input(input);
    let freq = right.iter().fold(HashMap::new(), |mut map, &val| {
        map.entry(val).and_modify(|frq| *frq += 1).or_insert(1);
        map
    });
    Some(left.iter().map(|v| v * freq.get(v).unwrap_or(&0)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
