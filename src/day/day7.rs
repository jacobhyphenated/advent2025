use super::Day;
use std::{collections::{HashMap, HashSet}, fs};
use crate::util::grid::prelude::*;

pub struct Day7;

impl Day<Vec2d<char>> for Day7 {
    fn read_input() -> Vec2d<char> {
        let input = fs::read_to_string("resources/day7.txt").expect("file day7.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec2d<char>) -> impl std::fmt::Display {
        let start = input.find(&'S').unwrap();
        let mut tachyons = HashSet::new();
        tachyons.insert(start);
        let mut split_count = 0;
        while !tachyons.is_empty() {
            let mut split_tachyons = HashSet::new();
            for tachyon in tachyons.into_iter() {
                if let Some(next) = input.next_point(tachyon, Directions::Down) {
                    if input[next] == '^' {
                        split_count += 1;
                        split_tachyons.insert(Point::new(next.x - 1, next.y));
                        split_tachyons.insert(Point::new(next.x + 1, next.y));
                    } else {
                        split_tachyons.insert(next);
                    }
                }
            }
            tachyons = split_tachyons;
        }
        split_count
    }

    fn part2(input: &Vec2d<char>) -> impl std::fmt::Display {
        let start = input.find(&'S').unwrap();
        let mut tachyon_timeline = HashMap::new();
        tachyon_timeline.insert(start, 1);
        let grid_height = input.grid.len() as i32 / input.line_len;
        for _ in 0 .. grid_height - 1{
            let mut new_timeline = HashMap::new();
            for (tachyon, count) in tachyon_timeline.into_iter() {
                let next = input.next_unbounded(tachyon, Directions::Down);
                if input[next] == '^' {
                    *new_timeline.entry(Point::new(next.x - 1, next.y)).or_insert(0) += count;
                    *new_timeline.entry(Point::new(next.x + 1, next.y)).or_insert(0) += count;
                } else {
                    *new_timeline.entry(next).or_insert(0) += count;
                }
            }
            tachyon_timeline = new_timeline;
        }
        tachyon_timeline.values().sum::<i64>()
    }
}

fn parse_input(input: &str) -> Vec2d<char> {
    let chars = input.lines()
        .flat_map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();
    let line_len = input.lines().next().unwrap().len() as i32;
    Vec2d { grid: chars, line_len }
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let input = parse_input(TEST);
        let result = Day7::part1(&input);
        assert_eq!("21", result.to_string());
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST);
        let result = Day7::part2(&input);
        assert_eq!("40", result.to_string());
    }
}
