use super::Day;
use std::fs;
use crate::util::grid::prelude::*;

pub struct Day4;

impl Day<Vec2d<bool>> for Day4 {
    fn read_input() -> Vec2d<bool> {
        let input = fs::read_to_string("resources/day4.txt").expect("file day4.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec2d<bool>) -> impl std::fmt::Display {
        input.grid.iter().enumerate()
            .filter(|&(_, paper)| *paper)
            .filter(|(idx, _)| {
                let point = input.idx_to_point(*idx);
                let num_adjacent = count_adjacent(input, point);
                num_adjacent < 4
            })
            .count()
    }

    fn part2(input: &Vec2d<bool>) -> impl std::fmt::Display {
        let mut grid = input.clone();
        let mut total_removed = 0;
        loop {
            let to_remove = grid.grid.iter().enumerate()
                .filter(|&(_, paper)| *paper)
                .map(|(idx, _)| grid.idx_to_point(idx))
                .filter(|&point| count_adjacent(&grid, point) < 4)
                .collect::<Vec<_>>();
            if to_remove.len() == 0 {
                break;
            }
            total_removed += to_remove.len();
            to_remove.into_iter().for_each(| point | grid[point] = false);
        }
        total_removed
    }
}

fn count_adjacent(grid: &Vec2d<bool>, point: Point) -> usize {
    const DIRECTIONS: [Directions; 8] = [Directions::Down, Directions::DownLeft, Directions::DownRight, Directions::Left,
        Directions::Right, Directions::Up, Directions::UpLeft, Directions::UpRight];
    DIRECTIONS.iter()
        .filter_map(|&d| grid.next_point(point, d))
        .filter(|&adjacent_point| grid[adjacent_point])
        .count()
}

fn parse_input(input: &str) -> Vec2d<bool> {
    let bools = input.lines()
        .flat_map(|line| line.trim().chars().map(|c| c == '@').collect::<Vec<_>>())
        .collect();
    let line_len = input.lines().next().unwrap().len() as i32;
    Vec2d { grid: bools, line_len }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        let input = parse_input(TEST);
        let result = Day4::part1(&input);
        assert_eq!("13", result.to_string());
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST);
        let result = Day4::part2(&input);
        assert_eq!("43", result.to_string());
    }
}
