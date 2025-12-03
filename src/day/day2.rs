use fancy_regex::Regex;

use super::Day;
use std::fs;

pub struct Day2;

impl Day<Vec<(i64, i64)>> for Day2 {
    fn read_input() -> Vec<(i64, i64)> {
        let input = fs::read_to_string("resources/day2.txt").expect("file day2.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec<(i64, i64)>) -> impl std::fmt::Display {
        input.iter()
            .flat_map(|&(start, end)| start ..= end)
            .filter(|product| {
                let product = product.to_string();
                let half = product.len() / 2;
                &product[0..half] == &product[half..]
            })
            .sum::<i64>()
    }

    fn part2(input: &Vec<(i64, i64)>) -> impl std::fmt::Display {
        // Regex solution:
        // ^        from the start of the line
        // (\d+)    find one or more digits as a capture group (1)
        // \1       followed by an exact match to the capture group (using backreference)
        // +        one or more times
        // $        until the end of the string
        let re = Regex::new(r"^(\d+)\1+$").expect("Invalid regex");
        input.iter()
            .flat_map(|&(start, end)| start ..= end)
            .filter(|product| re.is_match(&product.to_string()).unwrap())
            .sum::<i64>()
    }
}

fn parse_input(input: &str) -> Vec<(i64, i64)> {
    input.split(",")
        .map(|range| {
            let range_parts = range.split("-")
                .map(|p| p.parse().unwrap())
                .collect::<Vec<_>>();
            (range_parts[0], range_parts[1])
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_1() {
        let input = parse_input(TEST);
        let result =  Day2::part1(&input);
        assert_eq!("1227775554", result.to_string())
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(TEST);
        let result =  Day2::part2(&input);
        assert_eq!("4174379265", result.to_string())
    }

}