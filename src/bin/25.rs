use advent_of_code::util::{grid::*, point::*};

advent_of_code::solution!(25);

fn key(g: Grid<char>) -> [u8; 5] {
    let mut key = [0; 5];
    for pin in 0..5 {
        for height in (0..=5).rev() {
            if g[Point(pin, 6 - height)] == '#' {
                key[pin as usize] = height as u8;
                break;
            }
        }
    }
    key
}

fn lock(g: Grid<char>) -> [u8; 5] {
    let mut key = [0; 5];
    for pin in 0..5 {
        for height in (0..=5).rev() {
            if g[Point(pin, height)] == '#' {
                key[pin as usize] = height as u8;
                break;
            }
        }
    }
    key
}

fn parse(input: &str) -> Option<(Vec<[u8; 5]>, Vec<[u8; 5]>)> {
    let mut keys = vec![];
    let mut locks = vec![];
    let schematics = input.split("\n\n");
    for schematic in schematics {
        let g = Grid::parse::<char>(schematic).ok()?;
        // println!("{}", g);
        if g[Point(0, 0)] == '.' {
            let key = key(g);
            // println!("key: {:?}\n", key);
            keys.push(key);
        } else {
            let lock = lock(g);
            // println!("lock: {:?\n}", lock);
            locks.push(lock);
        }
    }
    Some((keys, locks))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (keys, locks) = parse(input)?;
    let mut count = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(&a, &b)| a + b < 6) {
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<usize> {
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
