advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let dirs = [
        [(0, 0), (0, 1), (0, 2), (0, 3)],
        [(0, 0), (1, 0), (2, 0), (3, 0)],
        [(0, 0), (1, 1), (2, 2), (3, 3)],
        [(3, 0), (2, 1), (1, 2), (0, 3)],
    ];
    let mut acc = 0;
    for row in 0..grid[0].len() {
        for col in 0..grid.len() {
            acc += dirs
                .iter()
                .map(|d| {
                    d.map(|(x, y)| (row + x, col + y))
                        .map(|(x, y)| grid.get(y).and_then(|row| row.get(x)).unwrap_or(&'.'))
                        .into_iter()
                        .collect::<String>()
                })
                .filter(|s| s == "XMAS" || s == "SAMX")
                .count();
        }
    }
    Some(acc)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let dirs = [[(0, 0), (1, 1), (2, 2)], [(2, 0), (1, 1), (0, 2)]];
    let mut acc = 0;
    for row in 0..grid[0].len() - 2 {
        for col in 0..grid.len() - 2 {
            if dirs
                .iter()
                .map(|d| {
                    d.map(|(x, y)| (row + x, col + y))
                        .map(|(x, y)| grid.get(y).and_then(|row| row.get(x)).unwrap_or(&'.'))
                        .into_iter()
                        .collect::<String>()
                })
                .all(|s| s == "MAS" || s == "SAM")
            {
                acc += 1;
            }
        }
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
