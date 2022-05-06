// Day 3: Binary Diagnostic
// see https://adventofcode.com/2021/day/3

#![allow(dead_code)]

use indoc::indoc;
use std::fs::read_to_string;

const EXAMPLE: &str = indoc! {"
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010
"};

fn parse(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect()
}

fn load_input(filename: &str) -> Vec<String> {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

// Add two vecs element by element.
// see https://stackoverflow.com/questions/41207666/how-do-i-add-two-rust-arrays-element-wise
fn add_vecs(a: Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    let mut z = vec![0; a.len()];
    for ((zref, aval), bval) in z.iter_mut().zip(&a).zip(&b) {
        *zref = aval + bval;
    }
    return z;
}

fn add_vecs2(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
    let mut z = vec![0; a.len()];
    for ((zref, aval), bval) in z.iter_mut().zip(a).zip(b) {
        *zref = aval + bval;
    }
    return z;
}

// String --> bit Vec
fn to_bits(x: &String) -> Vec<u32> {
    x.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

// bit Vec --> int
// there must be an easier way... I feel dumb.
fn from_bits(bs: &Vec<u32>) -> u32 {
    let mut result = 0;
    for bit in 0..bs.len() {
        result |= bs[bs.len() - bit - 1] << bit;
    }
    result
}

fn invert(xs: &Vec<u32>) -> Vec<u32> {
    xs.iter().map(|&x| if x == 1 { 0 } else { 1 }).collect()
}

// Find the "most common" value (the mode) for each bit
fn modes(xs: &Vec<Vec<u32>>) -> Vec<u32> {
    xs.iter()
        .cloned()
        .reduce(add_vecs)
        .unwrap()
        .iter()
        .map(|c| if c * 2 >= xs.len() as u32 { 1 } else { 0 })
        .collect()
}

fn gamma_epsilon(nums: &Vec<String>) -> (u32, u32) {
    let bits: Vec<Vec<u32>> = nums.iter().map(to_bits).collect();
    let mode_bits: Vec<u32> = modes(&bits);

    let gamma = from_bits(&mode_bits);
    let epsilon = from_bits(&invert(&mode_bits));

    return (gamma, epsilon);
}

fn filter_bitmask(xs: Vec<Vec<u32>>, mask: Vec<u32>, bit: usize) -> Vec<Vec<u32>> {
    xs.into_iter().filter(|x| x[bit] == mask[bit]).collect()
}

fn oxygen_co2(nums: &Vec<String>) -> (u32, u32) {
    // oxygen: reduce the list of numbers by calculating the most common bit,
    // at each position, and keeping ony the values that share that bit.
    // Must recalc the most common bit after each pass, not up front for the whole list.
    let mut i = 0;
    let mut o2_bits: Vec<Vec<u32>> = nums.iter().map(to_bits).collect();
    while o2_bits.len() > 1 {
        let most_common: Vec<u32> = modes(&o2_bits);
        o2_bits = filter_bitmask(o2_bits, most_common, i);
        i += 1;
    }

    // co2: same as above, but for the least common bit
    let mut j = 0;
    let mut co2_bits: Vec<Vec<u32>> = nums.iter().map(to_bits).collect();
    while co2_bits.len() > 1 {
        let least_common: Vec<u32> = invert(&modes(&co2_bits));
        co2_bits = filter_bitmask(co2_bits, least_common, j);
        j += 1;
    }

    let oxygen = from_bits(&o2_bits[0]);
    let co2 = from_bits(&co2_bits[0]);

    return (oxygen, co2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = parse(EXAMPLE);
        let (gamma, epsilon) = gamma_epsilon(&nums);
        assert_eq!((22, 9), (gamma, epsilon));
        assert_eq!(198, gamma * epsilon);
    }

    #[test]
    fn example_2() {
        let nums = parse(EXAMPLE);
        let (oxygen, co2) = oxygen_co2(&nums);
        assert_eq!((23, 10), (oxygen, co2));
        assert_eq!(230, oxygen * co2);
    }

    #[test]
    fn part_1() {
        let nums = load_input("input/input03.txt");
        let (gamma, epsilon) = gamma_epsilon(&nums);
        assert_eq!(1540244, gamma * epsilon);
    }

    #[test]
    fn part_2() {
        let nums = load_input("input/input03.txt");
        let (oxygen, co2) = oxygen_co2(&nums);
        assert_eq!(4203981, oxygen * co2);
    }
}
