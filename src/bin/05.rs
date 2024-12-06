advent_of_code::solution!(5);

use std::cmp::Ordering;
use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut data = input.lines();

    let rules = data
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|rule| rule.split_once('|').unwrap())
        .map(|(a, b)| (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap()))
        .collect::<Vec<(u32, u32)>>();

    let updates = data
        .by_ref()
        .skip_while(|line| line.is_empty())
        .map(|update| {
            update
                .split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, updates)
}

/*
fn check_rule(update: &[u32], rule: &(u32, u32)) -> bool {
    let p1 = update.iter().position(|&x| x == rule.0);
    let p2 = update.iter().position(|&x| x == rule.1);
    if p1.is_none() || p2.is_none() {
        return true;
    }
    p1 < p2
}

pub fn _part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    Some(
        updates
            .iter()
            .filter(|update| rules.iter().all(|rule| check_rule(update, rule)))
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}
*/

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let cmp: HashMap<(u32, u32), bool> = rules
        .iter()
        .cloned()
        .flat_map(|(a, b)| [((a, b), true), ((b, a), false)])
        .collect();
    Some(
        updates
            .iter()
            .filter(|&update| update.is_sorted_by(|&a, &b| *cmp.get(&(a, b)).unwrap()))
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

/*
fn apply_rules(update: &[u32], rules: &[(u32, u32)]) -> Vec<u32> {
    let mut fixed = update.to_vec();
    while !rules.iter().all(|rule| check_rule(&fixed, rule)) {
        for rule in rules {
            let p1 = fixed.iter().position(|&x| x == rule.0);
            let p2 = fixed.iter().position(|&x| x == rule.1);
            if p1.is_none() || p2.is_none() || p1 < p2 {
                continue;
            }
            fixed.swap(p1.unwrap(), p2.unwrap());
            // println!("# {:?}", rule);
            // println!(". {:?}", fixed);
        }
    }
    fixed
}

pub fn _part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    Some(
        updates
            .iter()
            .filter(|update| rules.iter().any(|rule| !check_rule(update, rule)))
            // .inspect(|x| println!("{:?}", x))
            .map(|update| apply_rules(update, &rules))
            // .inspect(|x| println!("{:?}", x))
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}
*/

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_input(input);
    let cmp: HashMap<(u32, u32), bool> = rules
        .iter()
        .cloned()
        .flat_map(|(a, b)| [((a, b), true), ((b, a), false)])
        .collect();
    Some(
        updates
            .iter()
            .filter(|&update| !update.is_sorted_by(|&a, &b| *cmp.get(&(a, b)).unwrap()))
            .map(|update| {
                let mut fixed = update.clone();
                fixed.sort_by(|&a, &b| match cmp.get(&(a, b)) {
                    Some(true) => Ordering::Less,
                    Some(false) => Ordering::Greater,
                    None => panic!(),
                });
                fixed
            })
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
