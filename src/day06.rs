// Day 6: Lanternfish
// see https://adventofcode.com/2021/day/6

#![allow(dead_code)]

use std::fs::read_to_string;

const EXAMPLE: &str = "3,4,3,1,2";

fn parse(input: &str) -> Vec<u64> {
    input.split(",").map(|x| x.parse().unwrap()).collect()
}

fn load_input(filename: &str) -> Vec<u64> {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

// naive, literal simulation. Will not work for large populations.
fn simulate_naive(start: &Vec<u64>, days: u64) -> Vec<u64> {
    let mut population = start.clone();

    for _ in 0..days {
        // decrement each fish, or if zero reset (6) and spawn a new fish (8)
        population = population
            .into_iter()
            .flat_map(|ttl| if ttl > 0 { vec![ttl - 1] } else { vec![6, 8] })
            .collect()
    }

    return population;
}

// starting population --> histogram fish/day
fn simulate(start: &Vec<u64>, rounds: u64) -> [u64; 9] {
    // build an initial histogram, count of fish at each day
    let mut days = [0; 9];
    for f in start {
        days[*f as usize] += 1;
    }

    for _ in 0..rounds {
        // old fish reset to day 6, and spawn new fish at day 8.
        // with rotate_left(), day 0 becomes day 8, so no need to explicitly spawn
        let old_fish = days[0];
        days.rotate_left(1);
        days[6] += old_fish;
    }
    return days;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let start = parse(EXAMPLE);
        let pop = simulate_naive(&start, 3);
        let pop2 = simulate(&start, 3);
        assert_eq!(7, pop.len());
        assert_eq!(7u64, pop2.iter().sum());
    }

    #[test]
    fn example_2() {
        let start = parse(EXAMPLE);
        let pop = simulate(&start, 256);
        assert_eq!(26984457539u64, pop.iter().sum());
    }

    #[test]
    fn part_1() {
        let start = load_input("input/input06.txt");
        assert_eq!(300, start.len());
        let population = simulate(&start, 80);
        assert_eq!(379114u64, population.iter().sum());
    }

    #[test]
    fn part_2() {
        let start = load_input("input/input06.txt");
        let population = simulate(&start, 256);
        assert_eq!(1702631502303u64, population.iter().sum());
    }
}
