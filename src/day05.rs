#![allow(dead_code)]

use indoc::indoc;
use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

const EXAMPLE: &str = indoc! {"
    0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2
"};

#[derive(Debug)]
struct Segment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn to_int(x: regex::Match) -> i32 {
    x.as_str().parse().unwrap()
}

fn parse(input: &str) -> Vec<Segment> {
    let line_format = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("Invalid regex");

    input
        .lines()
        .map(|x| {
            let points = line_format.captures(x).unwrap();
            Segment {
                x1: points.get(1).map_or(0, to_int),
                y1: points.get(2).map_or(0, to_int),
                x2: points.get(3).map_or(0, to_int),
                y2: points.get(4).map_or(0, to_int),
            }
        })
        .collect()
}

fn load_input(filename: &str) -> Vec<Segment> {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

fn horizontal_or_vertical(s: &Segment) -> bool {
    s.x1 == s.x2 || s.y1 == s.y2
}

fn plot_points(segments: Vec<Segment>) -> HashMap<Point, i32> {
    let mut points = HashMap::new();

    for s in segments {
        let dx = (s.x2 - s.x1).signum();
        let dy = (s.y2 - s.y1).signum();

        // iterate x1,y1 --> x2,y2. The +dx is critical to include the endpoint.
        let mut x = s.x1;
        let mut y = s.y1;
        while (x, y) != (s.x2 + dx, s.y2 + dy) {
            let count = points.entry(Point { x, y }).or_insert(0);
            *count += 1;
            x += dx;
            y += dy;
        }
    }

    return points;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let ls = parse(EXAMPLE);
        assert_eq!(10, ls.len());

        let lines = ls.into_iter().filter(horizontal_or_vertical).collect();
        let points = plot_points(lines);
        let count = points.into_values().filter(|&x| x > 1).count();

        assert_eq!(5, count);
    }

    #[test]
    fn example_2() {
        let lines = parse(EXAMPLE);
        let points = plot_points(lines);
        let count = points.into_values().filter(|&x| x > 1).count();

        assert_eq!(12, count);
    }

    #[test]
    fn part_1() {
        let ls = load_input("input/input05.txt");

        let lines = ls.into_iter().filter(horizontal_or_vertical).collect();
        let points = plot_points(lines);
        let count = points.into_values().filter(|&x| x > 1).count();

        assert_eq!(6267, count);
    }

    #[test]
    fn part_2() {
        let lines = load_input("input/input05.txt");
        let points = plot_points(lines);
        let count = points.into_values().filter(|&x| x > 1).count();

        assert_eq!(20196, count);
    }
}
