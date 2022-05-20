#![allow(dead_code)]

use indoc::indoc;
use std::fs::read_to_string;

const EXAMPLE: &str = indoc! {"
    aaaa
    bbbb
    cccc
"};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect()
}

fn load_input(filename: &str) -> Vec<String> {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let x = parse(EXAMPLE);
        assert_eq!(3, x.len());
    }

    #[test]
    fn example_2() {}

    #[test]
    fn part_1() {
        let x = load_input("input/test.txt");
        assert_eq!(5, x.len());
    }

    #[test]
    fn part_2() {}
}
