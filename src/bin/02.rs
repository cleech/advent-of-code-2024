advent_of_code::solution!(2);

use itertools::Itertools;

fn is_safe(report: &[i32]) -> bool {
    let mut ds = report.iter().tuple_windows().map(|(a, b)| b - a).peekable();
    let dir = ds.peek().unwrap().signum();

    ds.all(|d| match d.signum() {
            0 => false,
            s @ (1 | -1) => s == dir,
            _ => panic!(),
        } && d.abs() < 4)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                let report = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();
                is_safe(&report)
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                let report = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<_>>();

                is_safe(&report) || {
                    (0..report.len())
                        .map(|index| {
                            let mut new_report = report.clone();
                            new_report.remove(index);
                            new_report
                        })
                        .any(|nr| is_safe(&nr))
                }
            })
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
