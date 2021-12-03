// Day 2: Dive!
// see https://adventofcode.com/2021/day/2

#![allow(dead_code)]

use indoc::indoc;
use std::fs::read_to_string;

const EXAMPLE: &str = indoc! {"
    forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2
"};

type Command = (String, usize);
type Position = (usize, usize, usize);

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|x| {
            let (cmd, dist) = x.split_once(" ").unwrap();
            (cmd.to_string(), dist.parse().unwrap())
        })
        .collect()
}

fn load_input(filename: &str) -> Vec<Command> {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

fn maneuver(commands: &[Command]) -> (usize, usize) {
    let (x, depth) = commands
        .iter()
        .fold((0, 0), |acc, (dir, dist)| match dir.as_str() {
            "forward" => (acc.0 + dist, acc.1),
            "down" => (acc.0, acc.1 + dist),
            "up" => (acc.0, acc.1 - dist),
            _ => acc,
        });
    (x, depth)
}

fn maneuver2(commands: &[Command]) -> (usize, usize) {
    let start_point: Position = (0, 0, 0);
    let (x, depth, _) = commands
        .iter()
        .fold(start_point, |acc, (cmd, value)| match cmd.as_str() {
            "forward" => (acc.0 + value, acc.1 + acc.2 * value, acc.2),
            "down" => (acc.0, acc.1, acc.2 + value),
            "up" => (acc.0, acc.1, acc.2 - value),
            _ => acc,
        });
    (x, depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let commands = parse(EXAMPLE);
        let (x, depth) = maneuver(&commands);
        assert_eq!(150, x * depth);
    }

    #[test]
    fn example_2() {
        let commands = parse(EXAMPLE);
        let (x, depth) = maneuver2(&commands);
        assert_eq!(900, x * depth);
    }

    #[test]
    fn part_1() {
        let commands = load_input("input/input02.txt");
        let (x, depth) = maneuver(&commands);
        assert_eq!(1499229, x * depth);
    }

    #[test]
    fn part_2() {
        let commands = load_input("input/input02.txt");
        let (x, depth) = maneuver2(&commands);
        assert_eq!(1340836560, x * depth);
    }
}
