use itertools::Itertools;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u64> {
    let mut computers = HashSet::default();
    let mut links = HashMap::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-')?;
        computers.insert(a);
        computers.insert(b);
        links
            .entry(a)
            .and_modify(|lan: &mut Vec<&str>| lan.push(b))
            .or_insert(vec![b]);
        links
            .entry(b)
            .and_modify(|lan: &mut Vec<&str>| lan.push(a))
            .or_insert(vec![a]);
    }

    let mut count = 0;
    for a in computers {
        let a_neigh = links.get(a)?;
        for &b in a_neigh {
            if b > a {
                let b_neigh = links.get(b)?;
                for &c in b_neigh {
                    if c > b {
                        if a_neigh.contains(&c) {
                            if a.as_bytes()[0] == b't'
                                || b.as_bytes()[0] == b't'
                                || c.as_bytes()[0] == b't'
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut computers = HashSet::default();
    let mut links = HashMap::default();
    for line in input.lines() {
        let (a, b) = line.split_once('-')?;
        computers.insert(a);
        computers.insert(b);
        links
            .entry(a)
            .and_modify(|lan: &mut Vec<&str>| lan.push(b))
            .or_insert(vec![b]);
        links
            .entry(b)
            .and_modify(|lan: &mut Vec<&str>| lan.push(a))
            .or_insert(vec![a]);
    }

    for size in (3..=computers.len()).rev() {
        'outer: for clique in computers.iter().cloned().combinations(size) {
            for a in &clique[..] {
                let neigh = links.get(a)?;
                for b in &clique[..] {
                    if *a == *b {
                        continue;
                    }
                    if !neigh.contains(b) {
                        continue 'outer;
                    }
                }
            }
            let mut ret = clique.clone();
            ret.sort();
            return Some(ret.join(","));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
