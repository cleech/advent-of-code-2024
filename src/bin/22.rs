advent_of_code::solution!(22);

use rayon::prelude::*;

fn next(n: u64) -> u64 {
    let mut n = n;
    n = (n ^ (n << 6)) & 0x0ffffff;
    n = n ^ (n >> 5);
    n = (n ^ (n << 11)) & 0x0ffffff;
    n
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkeys: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>())
        .collect::<Result<_, _>>()
        .ok()?;
    Some(
        monkeys
            .par_iter()
            .map(|&n| {
                let mut n = n;
                for _ in 0..2000 {
                    n = next(n);
                }
                n
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkeys: Vec<u64> = input
        .lines()
        .map(|line| line.parse::<u64>())
        .collect::<Result<_, _>>()
        .ok()?;

    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
