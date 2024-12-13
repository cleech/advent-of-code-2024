advent_of_code::solution!(10);

use advent_of_code::util::grid::Grid;
use advent_of_code::util::point::*;

fn trailscore(grid: &mut Grid<(u32, bool)>, head: Point, part2: bool) -> u32 {
    let loc = grid[head].0;
    grid[head].1 = true;
    if loc == 9 {
        return 1;
    }

    let mut ret: u32 = 0;
    for &next in &[head + RIGHT, head + LEFT, head + DOWN, head + UP] {
        if grid.in_bounds(next) && (part2 || grid[next].1 == false) && grid[next].0 == loc + 1 {
            ret += trailscore(grid, next, part2);
        }
    }
    ret
}

pub fn solve(input: &str, part2: bool) -> Option<u32> {
    let grid: Grid<u32> = Grid::parse(input).ok()?;
    let mut score = 0;

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid[Point(x, y)] == 0 {
                let mut grid = grid.map(|&h| (h, false));
                score += trailscore(&mut grid, Point(x, y), part2);
            }
        }
    }
    Some(score)
}
pub fn part_one(input: &str) -> Option<u32> {
    solve(input, false)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));

        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(81));
    }
}
