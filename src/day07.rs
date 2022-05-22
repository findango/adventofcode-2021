// Day 7: The Treachery of Whales
// see https://adventofcode.com/2021/day/7

#![allow(dead_code)]

use itertools::Itertools;
use std::fs::read_to_string;

const EXAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

fn parse(input: &str) -> Vec<i32> {
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn load_input(filename: &str) -> Vec<i32> {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn median_distance(positions: &mut Vec<i32>) -> i32 {
    let median = median(positions);
    let dists = positions.iter().map(|x| (x - median).abs());
    return dists.sum();
}

// sum of numbers up to n: n(n+1)/2
fn sum_up_to(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn fuel_cost(positions: &Vec<i32>) -> i32 {
    let (&min, &max) = positions.iter().minmax().into_option().unwrap();

    // calc the total cost for all target positions, choose the cheapest
    let costs = (min..=max).map(|target| {
        positions
            .iter()
            .map(|x| sum_up_to((target - x).abs()))
            .sum()
    });

    return costs.min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut positions = parse(EXAMPLE);
        let median = median(&mut positions);
        assert_eq!(10, positions.len());
        assert_eq!(2, median);

        let dist = median_distance(&mut positions);
        assert_eq!(37, dist);
    }

    #[test]
    fn example_2() {
        let positions = parse(EXAMPLE);
        let cost = fuel_cost(&positions);
        assert_eq!(168, cost);
    }

    #[test]
    fn part_1() {
        let mut positions = load_input("input/input07.txt");
        assert_eq!(1000, positions.len());

        let dist = median_distance(&mut positions);
        assert_eq!(345035, dist);
    }

    #[test]
    fn part_2() {
        let positions = load_input("input/input07.txt");
        let cost = fuel_cost(&positions);
        assert_eq!(97038163, cost);
    }
}
