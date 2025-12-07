use super::Day;
use std::fs;
use crate::util::grid::prelude::*;

pub struct Day7;

impl Day<Vec2d<char>> for Day7 {
    fn read_input() -> Vec2d<char> {
        let input = fs::read_to_string("resources/day7.txt").expect("file day7.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec2d<char>) -> impl std::fmt::Display {
        todo!()
    }

    fn part2(input: &Vec2d<char>) -> impl std::fmt::Display {
        todo!()
    }
}

fn parse_input(input: &str) -> Vec2d<char> {
    let chars = input.lines()
        .flat_map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();
    let line_len = input.lines().next().unwrap().len() as i32;
    Vec2d { grid: chars, line_len }
}