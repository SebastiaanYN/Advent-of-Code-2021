#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

type Board = Vec<Vec<(i32, bool)>>;

fn parse_input(input: &str) -> (Vec<i32>, Vec<Board>) {
    let (numbers, boards) = input.split_once("\n\n").unwrap();

    let numbers = numbers
        .split(',')
        .map(|number| number.parse::<i32>().unwrap())
        .collect();

    let boards = boards
        .split("\n\n")
        .map(|board| {
            board
                .lines()
                .map(|row| {
                    row.split_whitespace()
                        .map(|column| (column.parse::<i32>().unwrap(), false))
                        .collect()
                })
                .collect()
        })
        .collect();

    (numbers, boards)
}

fn mark_number(board: &Board, number: i32) -> Board {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|&(row, set)| (row, set || row == number))
                .collect()
        })
        .collect()
}

fn is_complete(board: &Board) -> bool {
    board.iter().any(|row| row.iter().all(|&(_, set)| set))
        || board
            .iter()
            .map(|row| row.iter())
            .transpose()
            .any(|column| column.iter().all(|&&(_, set)| set))
}

fn sum_unmarked(board: &Board) -> i32 {
    board
        .iter()
        .flatten()
        .filter(|&&(_, set)| !set)
        .map(|&(num, _)| num)
        .sum()
}

fn part_1(input: &str) -> impl Debug {
    let (numbers, mut boards) = parse_input(input);

    for number in numbers {
        boards = boards
            .iter()
            .map(|board| mark_number(board, number))
            .collect();

        if let Some(board) = boards.iter().find(|board| is_complete(board)) {
            return number * sum_unmarked(board);
        }
    }

    0
}

fn part_2(input: &str) -> impl Debug {
    let (numbers, mut boards) = parse_input(input);

    let mut last_win = None;

    for number in numbers {
        boards = boards
            .iter()
            .map(|board| mark_number(board, number))
            .collect();

        if let Some(board) = boards.iter().find(|board| is_complete(board)) {
            last_win = Some((board.clone(), number));
        }

        boards = boards
            .iter()
            .filter(|board| !is_complete(board))
            .map(|board| board.clone())
            .collect();
    }

    if let Some((board, number)) = last_win {
        number * sum_unmarked(&board)
    } else {
        0
    }
}
