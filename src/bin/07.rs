advent_of_code::solution!(7);

use itertools::{repeat_n, Itertools};
use std::collections::VecDeque;

struct Equation {
    value: u64,
    inputs: Vec<u64>,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    N(u64),
    Add,
    Mul,
    Con,
}
use Op::*;

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|s| {
            let (value, inputs) = s.split_once(':').unwrap();
            let value = value.parse::<u64>().unwrap();
            let inputs = inputs
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            Equation { value, inputs }
        })
        .collect::<Vec<_>>()
}

fn eval(eq: &mut VecDeque<Op>) -> Option<u64> {
    let mut left = match eq.pop_front()? {
        N(n) => n,
        _ => panic!(),
    };
    while let Some(op) = eq.pop_front() {
        let right = match eq.pop_front()? {
            N(n) => n,
            _ => panic!(),
        };
        left = match op {
            Add => left + right,
            Mul => left * right,
            Con => {
                let s = left.to_string() + &right.to_string();
                s.parse::<u64>().unwrap()
            }
            _ => panic!(),
        }
    }
    Some(left)
}

fn solveable(eq: &Equation, o: &[Op]) -> bool {
    for ops in repeat_n(o, eq.inputs.len() - 1).multi_cartesian_product() {
        let mut e = eq
            .inputs
            .iter()
            .map(|&n| N(n))
            .interleave(ops.into_iter().cloned())
            .collect::<VecDeque<_>>();
        if eval(&mut e).unwrap() == eq.value {
            return true;
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let x = equations
        .into_iter()
        .filter(|e| solveable(e, &[Add, Mul]))
        .map(|e| e.value)
        .sum();
    Some(x)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let x = equations
        .into_iter()
        .filter(|e| solveable(e, &[Add, Mul, Con]))
        .map(|e| e.value)
        .sum();
    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
