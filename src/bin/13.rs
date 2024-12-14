use priority_queue::PriorityQueue;
use sscanf::sscanf;
use std::{cmp::Reverse, str::FromStr};

advent_of_code::solution!(13);

struct Claw {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl FromStr for Claw {
    type Err = sscanf::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = line.lines().collect();
        let a = sscanf!(lines[0], "Button A: X+{i64}, Y+{i64}")?;
        let b = sscanf!(lines[1], "Button B: X+{i64}, Y+{i64}")?;
        let prize = sscanf!(lines[2], "Prize: X={i64}, Y={i64}")?;
        Ok(Claw { a, b, prize })
    }
}

fn parse(input: &str, error: i64) -> Vec<Claw> {
    input
        .split("\n\n")
        .filter_map(|chunk| Claw::from_str(chunk).ok())
        .map(|c| Claw {
            a: c.a,
            b: c.b,
            prize: (c.prize.0 + error, c.prize.1 + error),
        })
        .collect()
}

/* slow search method */
fn solve(claw: &Claw) -> Option<i64> {
    let cost = |(a, b)| 3 * a + b;
    let score = |(a, b)| (a * claw.a.0 + b * claw.b.0, a * claw.a.1 + b * claw.b.1);

    let mut pq = PriorityQueue::new();
    for attempt in [(1_i64, 0_i64), (0_i64, 1_i64)] {
        pq.push(attempt, Reverse(cost(attempt)));
    }

    while let Some((next, c)) = pq.pop() {
        let pos = score(next);
        if pos == claw.prize {
            return Some(c.0);
        }
        if pos.0 > claw.prize.0 || pos.1 > claw.prize.1 {
            continue;
        }
        let n = (next.0 + 1, next.1);
        let m = (next.0, next.1 + 1);
        pq.push(n, Reverse(cost(n)));
        pq.push(m, Reverse(cost(m)));
    }
    None
}

/* smart linear algebra way */
fn math(claw: &Claw) -> Option<i64> {
    // A*a_x + B*B_x = p_x
    // A*a_y + B*B_y = p_y
    let (a_x, a_y) = claw.a;
    let (b_x, b_y) = claw.b;
    let (p_x, p_y) = claw.prize;
    // https://en.wikipedia.org/wiki/Cramer%27s_rule
    let a = (p_x * b_y - b_x * p_y) / (a_x * b_y - b_x * a_y);
    let b = (a_x * p_y - p_x * a_y) / (a_x * b_y - b_x * a_y);
    // check to see if there is a solution
    if (a * a_x + b * b_x, a * a_y + b * b_y) == (p_x, p_y) {
        Some((3 * a + b) as i64)
    } else {
        None
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let claws = parse(input, 0);
    // Some(claws.iter().filter_map(|c| solve(c)).sum::<i64>())
    Some(claws.iter().filter_map(|c| math(c)).sum::<i64>())
}

pub fn part_two(input: &str) -> Option<i64> {
    let claws = parse(input, 10000000000000_i64);
    Some(claws.iter().filter_map(|c| math(c)).sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }
}
