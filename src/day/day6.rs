use super::Day;
use std::fs;

pub struct Day6;

impl Day<String> for Day6 {
    fn read_input() -> String {
        fs::read_to_string("resources/day6.txt").expect("file day6.txt not found")
    }

    fn part1(input: &String) -> impl std::fmt::Display {
        // flip the 2d array so we can easily compute problems top down
        let problems = as_top_down(input.as_str());
        problems.iter().map(|row| {
            let operation = row.last().unwrap().as_str();
            let result = &row[0 .. row.len() - 1].iter()
                .map(|num| num.parse::<i64>().unwrap())
                .reduce(|a, b| match operation {
                    "+" => a + b,
                    "*" => a * b,
                    _ => panic!("Invalid operation")
                }).unwrap();
            *result
        })
        .sum::<i64>()
    }

    fn part2(input: &String) -> impl std::fmt::Display {
        let lines = input.lines().collect::<Vec<_>>();
        // the last line is the list of operators (+ or *)
        let operations = lines.last().unwrap().trim().split_whitespace().rev().collect::<Vec<_>>();
        let length = lines.iter().map(|line| line.len()).max().unwrap();

        // turn the remaining lines into char arrays for indexing
        let lines = &lines[0..lines.len() -1].iter()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut all_problems = vec![];
        let mut current_problemset = vec![];
        // loop right to left, top to bottom
        for i in (0 .. length).rev() {
            let mut problem_line = vec![];
            for line in lines{
                let c = line[i];
                if !c.is_whitespace(){
                    problem_line.push(c);
                }
            }
            if !problem_line.is_empty() {
                let num = problem_line.iter().collect::<String>().parse::<i64>().unwrap();
                current_problemset.push(num);
            } else {
                // a line of all whitespace separates this set of numbers/opertor from the next set
                all_problems.push(current_problemset);
                current_problemset = vec![];
            }
        }
        all_problems.push(current_problemset);

        // recombine the problem set with the associated operator
        all_problems.into_iter().zip(operations.into_iter())
            .map(|(problemset, operator)| {
                problemset.into_iter().reduce(|a, b| match operator {
                    "+" => a + b,
                    "*" => a * b,
                    _ => panic!("Invalid operation")
                }).unwrap()
            })
            .sum::<i64>()
    }
}

fn as_top_down(input: &str) -> Vec<Vec<String>> {
    let lines = input.lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut result = vec![];
    for i in 0 .. lines[0].len() {
        let mut result_line = vec![];
        for line in lines.iter() {
            result_line.push(line[i].to_string());
        }
        result.push(result_line);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        let input = TEST.to_string();
        let result = Day6::part1(&input);
        assert_eq!("4277556", result.to_string());
    }

    #[test]
    fn test_part2() {
        let input = TEST.to_string();
        let result = Day6::part2(&input);
        assert_eq!("3263827", result.to_string());
    }
}