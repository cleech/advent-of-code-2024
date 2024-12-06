advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").ok()?;
    Some(
        re.captures_iter(input)
            .map(|c| {
                let (_, [a, b]) = c.extract();
                a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)|do\(\)|don't\(\)").unwrap();
    let (_, p2) = re
        .captures_iter(input)
        .fold((true, 0), |(enabled, acc), c| match &c[0] {
            "do()" => (true, acc),
            "don't()" => (false, acc),
            _ => {
                if enabled {
                    let a = &c[1].parse::<u32>().unwrap();
                    let b = &c[2].parse::<u32>().unwrap();
                    (true, acc + (a * b))
                } else {
                    (false, acc)
                }
            }
        });
    Some(p2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
