use super::Day;
use std::{collections::{HashMap, HashSet}, fs};

pub struct Day8;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub struct Point3d {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3d {
    fn euclid_distance(&self, other: &Point3d) -> i64 {
        let square = (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2);
        // technically this is squared euclid, but it should be fine for this problem
        // this issue is the f64 is not Ord, so can't be naturally sorted. Stupid floating points.
        square
    }
}

impl Day<Vec<Point3d>> for Day8 {
    fn read_input() -> Vec<Point3d> {
        let input = fs::read_to_string("resources/day8.txt").expect("file day8.txt not found");
        parse_input(&input)
    }

    fn part1(input: &Vec<Point3d>) -> impl std::fmt::Display {
        let mut distances: HashMap<(Point3d, Point3d), i64> = HashMap::new();
        for i in 0 .. input.len() - 1 {
            for j in i + 1 .. input.len() {
                distances.insert((input[i], input[j]), input[i].euclid_distance(&input[j]));
            }
        }
        let mut pairs= distances.keys().collect::<Vec<_>>();
        pairs.sort_by_key(|&pair| distances[pair]);

        let mut circuits: Vec<HashSet<Point3d>> = vec![];
        for pair in &pairs[0..1000] {
            add_circuit_connection(pair, &mut circuits);
        }

        circuits.sort_by_key(|c| c.len());
        circuits.reverse();
        let first_3 = &circuits[0..3];
        first_3.iter()
            .map(|c| c.len())
            .product::<usize>()
    }

    fn part2(input: &Vec<Point3d>) -> impl std::fmt::Display {
        let mut distances: HashMap<(Point3d, Point3d), i64> = HashMap::new();
        for i in 0 .. input.len() - 1 {
            for j in i + 1 .. input.len() {
                distances.insert((input[i], input[j]), input[i].euclid_distance(&input[j]));
            }
        }
        let mut pairs= distances.keys().collect::<Vec<_>>();
        pairs.sort_by_key(|&pair| distances[pair]);

        let mut circuits: Vec<HashSet<Point3d>> = vec![];
        let mut i = 0;
        loop  {
            add_circuit_connection(pairs[i], &mut circuits);
            if circuits.len() == 1 && circuits[0].len() == input.len() {
                break;
            }
            i += 1;
        }
        let (p1, p2) = pairs[i];
        p1.x * p2.x

    }
}

fn add_circuit_connection(pair: &(Point3d, Point3d), circuits: &mut Vec<HashSet<Point3d>>) {
    let (p1, p2) = pair;

    // I wanted to use iter_mut().find(...) here, but you can't have 2 mutable references
    let mut p1_index = None;
    let mut p2_index = None;
    for i in 0 .. circuits.len() {
        if circuits[i].contains(p1) {
            p1_index = Some(i);
        }
        if circuits[i].contains(p2) {
            p2_index = Some(i);
        }
    }

    if let Some(p1_circuit) = p1_index {
        if let Some(p2_circuit) = p2_index {
            if p1_circuit != p2_circuit {
                // if the two points already exists in different circuits,
                // then we need to combine the 2 into 1 circuit
                // Make a new one an remove the 2 previous
                let new_circuit = circuits[p1_circuit].union(&circuits[p2_circuit])
                    .map(|p| p.clone())
                    .collect::<HashSet<_>>();
                circuits.remove(p1_circuit.max(p2_circuit));
                circuits.remove(p1_circuit.min(p2_circuit));
                circuits.push(new_circuit);
            }
        } else {
            circuits[p1_circuit].insert(p2.clone());
        }
    } else if let Some(p2_circuit) = p2_index {
        circuits[p2_circuit].insert(p1.clone());
    }
    else {
        let mut new_circuit = HashSet::new();
        new_circuit.insert(p1.clone());
        new_circuit.insert(p2.clone());
        circuits.push(new_circuit);
    }
}

fn parse_input(input: &str) -> Vec<Point3d> {
    input.lines()
        .map(|line| {
            let parts = line.trim()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            Point3d{
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}