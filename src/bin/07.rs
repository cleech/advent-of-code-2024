advent_of_code::solution!(7);

use rayon::prelude::*;

//use itertools::{repeat_n, Itertools};
//use std::collections::VecDeque;

struct Equation {
    value: u64,
    inputs: Vec<u64>,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    //     N(u64),
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

/*
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
                let n = 10u64.pow(right.ilog10() + 1);
                left * n + right
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
*/

fn solveable_1(acc: u64, values: &[u64], target: u64, o: &[Op]) -> bool {
    if values.len() == 0 {
        return acc == target;
    }
    return solveable_1(acc + values[0], &values[1..], target, o)
        || solveable_1(acc * values[0], &values[1..], target, o);
}

fn solveable_2(acc: u64, values: &[u64], target: u64, o: &[Op]) -> bool {
    if values.len() == 0 {
        return acc == target;
    }
    return solveable_2(acc + values[0], &values[1..], target, o)
        || solveable_2(acc * values[0], &values[1..], target, o)
        || {
            let n = 10u64.pow(values[0].ilog10() + 1);
            solveable_2(acc * n + values[0], &values[1..], target, o)
        };
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let x = equations
        .into_par_iter()
        // .filter(|e| solveable(e, &[Add, Mul]))
        .filter(|e| solveable_1(0, &e.inputs, e.value, &[Add, Mul]))
        .map(|e| e.value)
        .sum();
    Some(x)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_input(input);
    let x = equations
        .into_par_iter()
        // .filter(|e| solveable(e, &[Add, Mul, Con]))
        .filter(|e| solveable_2(0, &e.inputs, e.value, &[Add, Mul, Con]))
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
