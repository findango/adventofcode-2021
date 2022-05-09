// Day 4: Giant Squid
// see https://adventofcode.com/2021/day/4

#![allow(dead_code)]

use indoc::indoc;
use regex::Regex;
use std::fs::read_to_string;

const EXAMPLE: &str = indoc! {"
    7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19

     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6

    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7
"};

// maybe make this a struct so I can add getters?
type Board = Vec<usize>;

fn to_int(x: &str) -> usize {
    x.parse().unwrap()
}

// String --> (nums, boards)
// nums: Vec<int>
// board: Vec<int> (fixed size 5x5)
fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let blank_line = Regex::new(r"\n\n").expect("Invalid regex");
    let blocks: Vec<_> = blank_line.split(&input).into_iter().collect();
    let nums: Vec<usize> = blocks[0].split(",").into_iter().map(to_int).collect();

    let boards: Vec<Board> = blocks
        .into_iter()
        .skip(1) // skip the first block of nums
        .map(|blk| blk.split_whitespace().map(to_int).collect())
        .collect();

    return (nums, boards);
}

fn load_input(filename: &str) -> (Vec<usize>, Vec<Board>) {
    let input = read_to_string(&filename).unwrap();
    parse(&input)
}

const MARKED: usize = 999999;

// Check if this board is complete, called numbers have been MARKED already.
// Boards are 5x5, check rows and cols only, no diagonals.
fn is_winner(board: &Board) -> bool {
    // check rows
    for i in 0..5 {
        let slice = &board[i * 5..i * 5 + 5];
        if slice.into_iter().all(|&x| x == MARKED) {
            return true;
        }
    }

    // check cols
    for i in 0..5 {
        let slice = &board[i..];
        if slice.into_iter().step_by(5).all(|&x| return x == MARKED) {
            return true;
        }
    }

    return false;
}

// (boards, all_nums) --> (winner_remaining_squares, called_nums, index_of_winner)
fn first_winner(boards: &Vec<Board>, nums: &Vec<usize>) -> (Board, Vec<usize>, usize) {
    for i in 1..nums.len() {
        let called_nums = &nums[0..i];

        for (i, board) in boards.iter().enumerate() {
            let marked_board: Board = board
                .into_iter()
                .map(|&x| if called_nums.contains(&x) { MARKED } else { x })
                .collect();

            if is_winner(&marked_board) {
                let unmarked_squares: Board = marked_board
                    .clone()
                    .into_iter()
                    .filter(|&x| x != MARKED)
                    .collect();
                return (unmarked_squares, called_nums.to_vec(), i);
            }
        }
    }

    // Not reachable with given input (always a winner)
    return (vec![], [].to_vec(), 0);
}

fn last_winner(boards: &Vec<Board>, nums: &Vec<usize>) -> (Board, Vec<usize>) {
    // find and remove the first winner until only one board left standing
    let mut remaining = boards.clone();
    while remaining.len() > 1 {
        let (_, _, i) = first_winner(&remaining, &nums);
        remaining.remove(i);
    }

    // last board remaining is the winner, but still need to find the numbers
    // called to make it a winner
    let (winner, nums_called, _) = first_winner(&remaining, &nums);
    return (winner, nums_called);
}

fn score(board: &Board, nums: &Vec<usize>) -> usize {
    board.into_iter().sum::<usize>() * nums.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let (nums, boards) = parse(EXAMPLE);
        assert_eq!(27, nums.len());
        assert_eq!(3, boards.len());

        let (board, called_nums, index) = first_winner(&boards, &nums);
        let score = score(&board, &called_nums);
        assert_eq!(2, index);
        assert_eq!(4512, score);
    }

    #[test]
    fn example_2() {
        let (nums, boards) = parse(EXAMPLE);
        let (board, called_nums) = last_winner(&boards, &nums);
        let score = score(&board, &called_nums);
        assert_eq!(1924, score);
    }

    #[test]
    fn part_1() {
        let (nums, boards) = load_input("input/input04.txt");
        assert_eq!(100, nums.len());
        assert_eq!(100, boards.len());

        let (board, called_nums, _) = first_winner(&boards, &nums);
        let score = score(&board, &called_nums);
        assert_eq!(16674, score);
    }

    #[test]
    fn part_2() {
        let (nums, boards) = load_input("input/input04.txt");
        let (board, called_nums) = last_winner(&boards, &nums);
        let score = score(&board, &called_nums);
        assert_eq!(7075, score);
    }
}
