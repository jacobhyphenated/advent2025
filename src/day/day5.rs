use super::Day;
use std::fs;

pub struct Day5;

type Inventory = (Vec<(i64, i64)>, Vec<i64>);

impl Day<Inventory> for Day5 {
    fn read_input() -> Inventory {
        let input = fs::read_to_string("resources/day5.txt").expect("file day5.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Inventory) -> impl std::fmt::Display {
        let (fresh_ranges, ingrediants) = input;
        ingrediants.iter().filter(|&ingrediant| {
            fresh_ranges.iter().any(|(start, end)| ingrediant <= end && ingrediant >= start)
        })
        .count()
    }

    fn part2(input: &Inventory) -> impl std::fmt::Display {
        // iterate through the ranges as a queue
        let mut fresh_ranges = input.0.clone();
        // create a final_range list that has all ranges with no overlap
        let mut final_ranges= vec![];

        'freshloop: while let Some(fresh) = fresh_ranges.pop() {
            let (mut start, mut end) = fresh;
            // compare the range we're inspecting to the "final" ranges
            for &(existing_start, existing_end) in final_ranges.iter() {
                // If this range is fully contained in an exiting range, discard it
                if start >= existing_start && end <= existing_end {
                    continue 'freshloop;
                }
                // If an existing range is full contained in this range,
                // split into two (non-overlapping) ranges to inspect
                if start < existing_start && end > existing_end {
                    fresh_ranges.push((existing_end + 1, end));
                    end = existing_start - 1;
                }
                // otherwise trim down the range to exclude overlaps with existing ranges
                else if start >= existing_start && start <= existing_end {
                    start = existing_end + 1;
                }
                else if end <= existing_end && end >= existing_start {
                    end = existing_start - 1;
                }
            }
            final_ranges.push((start, end));
        }
        final_ranges.iter()
            .map(|(start, end)| end - start + 1) //+ 1 because of the inclusive range
            .sum::<i64>()
    }
}

fn parse_input(input: &str) -> Inventory {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let ranges = parts[0].lines()
        .map(|line| {
            let range = line.split("-").map(|v| v.parse().unwrap()).collect::<Vec<_>>();
            (range[0], range[1])
        })
        .collect();

    let ingrediants = parts[1].lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ingrediants)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let input = parse_input(TEST);
        let result = Day5::part1(&input);
        assert_eq!("3", result.to_string());
    }

    #[test]
    fn test_part2() {
        let input = parse_input(TEST);
        let result = Day5::part2(&input);
        assert_eq!("14", result.to_string());
    }
}
