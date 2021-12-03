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
    commands
        .iter()
        .fold((0, 0), |(x, depth), (dir, dist)| match dir.as_str() {
            "forward" => (x + dist, depth),
            "down" => (x, depth + dist),
            "up" => (x, depth - dist),
            _ => (x, depth),
        })
}

fn maneuver2(commands: &[Command]) -> (usize, usize, usize) {
    commands
        .iter()
        .fold((0, 0, 0), |(x, depth, aim), (cmd, value)| {
            match cmd.as_str() {
                "forward" => (x + value, depth + aim * value, aim),
                "down" => (x, depth, aim + value),
                "up" => (x, depth, aim - value),
                _ => (x, depth, aim),
            }
        })
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
        let (x, depth, _) = maneuver2(&commands);
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
        let (x, depth, _) = maneuver2(&commands);
        assert_eq!(1340836560, x * depth);
    }
}
