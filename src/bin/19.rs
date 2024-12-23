advent_of_code::solution!(19);

fn parse(input: &str) -> Option<(Vec<&str>, Vec<&str>)> {
    let (prefix, suffix) = input.split_once("\n\n")?;
    let patterns = prefix.split(", ").collect::<Vec<_>>();
    let designs = suffix.lines().collect::<Vec<_>>();
    Some((patterns, designs))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (patterns, designs) = parse(input)?;
    let mut count = 0;

    for design in designs {
        let mut reachable = vec![false; design.len() + 1];
        reachable[0] = true;
        for i in 0..design.len() {
            if reachable[i] {
                for pattern in &patterns {
                    if design[i..].starts_with(pattern) {
                        reachable[i + pattern.len()] = true;
                    }
                }
            }
        }
        if reachable[design.len()] {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, designs) = parse(input)?;
    let mut count = 0;

    for design in designs {
        let mut reachable = vec![0; design.len() + 1];
        reachable[0] = 1;
        for i in 0..design.len() {
            if reachable[i] > 0 {
                for pattern in &patterns {
                    if design[i..].starts_with(pattern) {
                        reachable[i + pattern.len()] += reachable[i];
                    }
                }
            }
        }
        count += reachable[design.len()];
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
