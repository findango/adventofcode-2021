#![allow(dead_code)]

use indoc::indoc;
use std::fs::read_to_string;

const EXAMPLE: &str = indoc! {"
"};

fn parse(input: &str) -> String {
    String::from(input)
}

fn load_input(filename: &str) -> String {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(7, 7);
    }

    #[test]
    fn example_2() {}

    #[test]
    fn part_1() {}

    #[test]
    fn part_2() {}
}
