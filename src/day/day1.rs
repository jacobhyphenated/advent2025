use super::Day;
use std::fs;

pub struct Day1;

impl Day<Vec<Rotation>> for Day1 {
    fn read_input() -> Vec<Rotation> {
        let input = fs::read_to_string("resources/day1.txt").expect("file day1.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec<Rotation>) -> impl std::fmt::Display {
        let mut dial = Dial::new();
        let mut zero_count = 0;
        for rotation in input {
            dial.turn(rotation);
            if dial.get_position() == 0 {
                zero_count += 1;
            }
        }
        zero_count
    }

    fn part2(input: &Vec<Rotation>) -> impl std::fmt::Display {
        let mut dial = Dial::new();
        let mut zero_count = 0;
        for rotation in input {
            zero_count += dial.turn_zero_count(rotation);
        }
        zero_count
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum Direction {
    Right,
    Left,
}

impl Direction {
    fn from_str(letter: &str) -> Self {
        match letter {
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("Invalid direction {letter}")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Rotation {
    direction: Direction,
    amount: i32,
}

#[derive(Debug, Clone)]
struct Dial {
    raw_position: i32,
}

impl Dial {
    fn new() -> Self {
        Self { raw_position: 50 }
    }

    fn turn(&mut self, rotation: &Rotation) {
        match rotation.direction {
            Direction::Left => self.raw_position -= rotation.amount,
            Direction::Right => self.raw_position += rotation.amount
        }
    }

    /// Adds extra logic to the turn that tracks how often the position passes through 0.
    /// Normalizes the raw_position onto the 0-99 scale.
    fn turn_zero_count(&mut self, rotation: &Rotation) -> i32 {
        let initial_position = self.raw_position;
        self.turn(rotation);
        let mut wraps = (self.raw_position / 100).abs();
        if initial_position != 0 && self.raw_position <= 0 {
            wraps += 1;
        }
        self.raw_position = self.get_position();
        wraps
    }

    fn get_position(&self) -> i32 {
        self.raw_position.rem_euclid(100)
    }
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input.lines().map( |l| {
        let direction = Direction::from_str(&l[..1]);
        let amount = l[1..].parse().unwrap();
        Rotation {
            direction,
            amount
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    
    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day1::part1(&input);
        assert_eq!("3", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day1::part2(&input);
        assert_eq!("6", result.to_string())
    }

}