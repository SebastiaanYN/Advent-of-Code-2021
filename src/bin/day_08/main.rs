#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::{collections::HashSet, fmt::Debug};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn part_1(input: &str) -> impl Debug {
    input
        .lines()
        .map(|line| {
            let (_, outputs) = line.split_once(" | ").unwrap();

            outputs
                .split_whitespace()
                .filter(|output| {
                    [
                        /*one*/ 2, /*four*/ 4, /*seven*/ 3, /*eight*/ 7,
                    ]
                    .contains(&output.len())
                })
                .count()
        })
        .sum::<usize>()
}

fn part_2(input: &str) -> impl Debug {
    input
        .lines()
        .map(|line| {
            let (signals, outputs) = line.split_once(" | ").unwrap();

            let signals = signals
                .split_whitespace()
                .map(|signal| signal.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();

            let one = signals.iter().find(|signal| signal.len() == 2).unwrap();
            let four = signals.iter().find(|signal| signal.len() == 4).unwrap();

            outputs
                .split_whitespace()
                .map(|output| output.chars().collect::<HashSet<_>>())
                .map(|output| {
                    match (
                        output.len(),
                        output.intersection(four).count(),
                        output.intersection(one).count(),
                    ) {
                        (2, _, _) => 1,
                        (4, _, _) => 4,
                        (3, _, _) => 7,
                        (7, _, _) => 8,

                        (5, 2, _) => 2,
                        (5, 3, 1) => 5,
                        (5, 3, 2) => 3,

                        (6, 4, _) => 9,
                        (6, 3, 1) => 6,
                        (6, 3, 2) => 0,
                        _ => unreachable!(),
                    }
                })
                .rev()
                .zip(std::iter::successors(Some(1usize), |n| n.checked_mul(10)))
                .map(|(digit, n)| digit * n)
                .sum::<usize>()
        })
        .sum::<usize>()
}
