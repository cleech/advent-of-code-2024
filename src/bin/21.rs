advent_of_code::solution!(21);

use std::iter;

use advent_of_code::util::point::*;
use itertools::Itertools;
use rustc_hash::FxHashMap as HashMap;

fn numeric_keypad(start: char, end: char) -> Vec<String> {
    let button = |c| match c {
        '9' => Point(2, 0),
        '8' => Point(1, 0),
        '7' => Point(0, 0),
        '6' => Point(2, 1),
        '5' => Point(1, 1),
        '4' => Point(0, 1),
        '3' => Point(2, 2),
        '2' => Point(1, 2),
        '1' => Point(0, 2),
        'A' => Point(2, 3),
        '0' => Point(1, 3),
        _ => unreachable!(),
    };
    let a = button(start);
    let b = button(end);

    let Point(x, y) = b - a;
    // println!("{pos} => {b} = ({x},{y})");

    let xs = match x.signum() {
        0 => String::new(),
        1 => ">".to_owned(),
        -1 => "<".to_owned(),
        _ => unreachable!(),
    }
    .repeat(x.abs() as usize);
    let ys = match y.signum() {
        0 => String::new(),
        1 => "v".to_owned(),
        -1 => "^".to_owned(),
        _ => unreachable!(),
    }
    .repeat(y.abs() as usize);

    let mut output = vec![];

    if !(a.1 == 3 && b.0 == 0) {
        let mut s1 = String::new();
        s1.push_str(&xs);
        s1.push_str(&ys);
        s1.push_str("A");
        output.push(s1);
    }

    if !(a.0 == 0 && b.1 == 3) {
        let mut s2 = String::new();
        s2.push_str(&ys);
        s2.push_str(&xs);
        s2.push_str("A");
        output.push(s2);
    }

    output
}

fn directional_keypad(start: char, end: char) -> Vec<String> {
    let button = |c| match c {
        '^' => Point(1, 0),
        '<' => Point(0, 1),
        'v' => Point(1, 1),
        '>' => Point(2, 1),
        'A' => Point(2, 0),
        _ => unreachable!(),
    };
    let a = button(start);
    let b = button(end);

    let Point(x, y) = b - a;
    // println!("{pos} => {b} = ({x},{y})");

    let xs = match x.signum() {
        0 => String::new(),
        1 => ">".to_owned(),
        -1 => "<".to_owned(),
        _ => unreachable!(),
    }
    .repeat(x.abs() as usize);
    let ys = match y.signum() {
        0 => String::new(),
        1 => "v".to_owned(),
        -1 => "^".to_owned(),
        _ => unreachable!(),
    }
    .repeat(y.abs() as usize);

    let mut output = vec![];
    if !(a.1 == 0 && b.0 == 0) {
        let mut s1 = String::new();
        s1.push_str(&xs);
        s1.push_str(&ys);
        s1.push_str("A");
        output.push(s1);
    }

    if !(a.0 == 0 && b.1 == 0) {
        let mut s2 = String::new();
        s2.push_str(&ys);
        s2.push_str(&xs);
        s2.push_str("A");
        output.push(s2);
    }

    output
}

fn npad(input: &str, robots: usize) -> usize {
    let mut acc = 0;
    // println!("= {} =", input);
    for (a, b) in iter::once('A').chain(input.chars()).tuple_windows() {
        let sequences = numeric_keypad(a, b);
        // println!("* {b}");
        // println!("- {:?}", sequences);
        acc += sequences
            .into_iter()
            .map(|s| dpad(&s, robots))
            // .inspect(|x| println!("  {}", x))
            .min()
            .unwrap();
    }
    acc
}

fn dpad(input: &str, robots: usize) -> usize {
    let mut acc = 0;
    // println!("== {} ==", input);
    for (a, b) in iter::once('A').chain(input.chars()).tuple_windows() {
        let sequences = directional_keypad(a, b);
        let depth = robots - 1;
        if depth > 0 {
            // println!("** {b}");
            // println!("-- {:?}", sequences);
            acc += sequences
                .into_iter()
                .map(|s| dpad(&s, depth))
                // .inspect(|x| println!("    {}", x))
                .min()
                .unwrap();
        } else {
            // println!("*** {b}");
            // println!("--- {:?}", sequences);
            acc += sequences
                .into_iter()
                .map(|s| s.len())
                // .inspect(|x| println!("      {}", x))
                .min()
                .unwrap();
        }
    }
    acc
}

fn day21(input: &str, robots: usize) -> Option<usize> {
    let mut sum = 0;
    for code in input.lines() {
        let c = &code[..code.len() - 1].parse::<usize>().ok()?;
        let s = npad(code, robots);
        // println!("{} {}", c, s);
        sum += c * s;
    }
    Some(sum)
}

pub fn part_one(input: &str) -> Option<usize> {
    day21(input, 2)
}

pub fn part_two(input: &str) -> Option<usize> {
    day21(input, 25)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }
}
