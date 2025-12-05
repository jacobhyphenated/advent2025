
use super::Day;
use std::fs;

pub struct Day3;

impl Day<Vec<Vec<u32>>> for Day3 {
    fn read_input() -> Vec<Vec<u32>> {
        let input = fs::read_to_string("resources/day3.txt").expect("file day3.txt not found");
        parse_input(&input)
    }

    // Solved in O(n) using 2 pointers
    fn part1(input: &Vec<Vec<u32>>) -> impl std::fmt::Display {
        input.iter().map(|bank|{
            let mut p1 = 0;
            let mut p2 = 0;
            for i in 0 .. bank.len() {
                if i < bank.len() - 1 && bank[i] > p1 {
                    p1 = bank[i];
                    p2 = 0;
                }
                else if bank[i] > p2 {
                    p2 = bank[i];
                }
            }
            p1 * 10 + p2
        })
        .sum::<u32>()
    }

    // welp
    // same approach, but with 12 pointers
    // use an array to track the 12 pointers
    fn part2(input: &Vec<Vec<u32>>) -> impl std::fmt::Display {
        input.iter().map(|bank|{
            // 12 pointers in an array
            let mut p = [0,0,0,0,0,0,0,0,0,0,0,0];

            for i in 0 .. bank.len() {

                // loop through the pointers and update
                for n in 0 .. 12 {
                    let offset = 12 - n;
                    if i <= bank.len() - offset && bank[i] > p[n] {
                        p[n] = bank[i];
                        // zero out the pointers to the right of the one just updated
                        for n2 in n + 1 .. 12 {
                            p[n2] = 0;
                        }
                        break;
                    } 
                }
            }
            p.into_iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<i64>().unwrap()

        })
        .sum::<i64>()
    }
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        let input = parse_input(TEST);
        let result = Day3::part1(&input);
        assert_eq!("357", result.to_string());
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST);
        let result = Day3::part2(&input);
        assert_eq!("3121910778619", result.to_string());
    }
}