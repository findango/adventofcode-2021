// Day 1: Sonar Sweep
// see https://adventofcode.com/2021/day/1

#![allow(dead_code)]

use indoc::indoc;
use std::fs::read_to_string;

const EXAMPLE: &str = indoc! {"
    199
    200
    208
    210
    200
    207
    240
    269
    260
    263
"};

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

fn load_input(filename: &str) -> Vec<usize> {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

fn filter_increasing(list: &[usize]) -> Vec<&[usize]> {
    list.windows(2).filter(|p| p[0] < p[1]).collect()
}

fn sum_3(list: &[usize]) -> Vec<usize> {
    list.windows(3).map(|w| w.iter().sum()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let depths = parse(EXAMPLE);
        let increasing = filter_increasing(&depths);
        assert_eq!(7, increasing.iter().count());
    }

    #[test]
    fn example_2() {
        let depths = parse(EXAMPLE);
        let windows = sum_3(&depths);
        let increasing = filter_increasing(&windows);
        assert_eq!(5, increasing.iter().count());
    }

    #[test]
    fn part_1() {
        let depths = load_input("input/input01.txt");
        let increasing = filter_increasing(&depths);
        assert_eq!(1288, increasing.iter().count());
    }

    #[test]
    fn part_2() {
        let depths = load_input("input/input01.txt");
        let windows = sum_3(&depths);
        let increasing = filter_increasing(&windows);
        assert_eq!(1311, increasing.iter().count());
    }
}
