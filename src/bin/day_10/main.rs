#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::{fmt::Debug, str::Chars};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn parse(line: Chars) -> Result<Vec<char>, char> {
    let mut openings = Vec::new();

    for c in line {
        match c {
            '(' | '[' | '{' | '<' => openings.push(c),
            _ => match (openings.pop(), c) {
                (Some('('), ')') | (Some('['), ']') | (Some('{'), '}') | (Some('<'), '>') => {}
                _ => return Err(c),
            },
        }
    }

    Ok(openings)
}

fn part_1(input: &str) -> impl Debug {
    input
        .lines()
        .map(|line| parse(line.chars()))
        .filter_map(|res| res.err())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unreachable!(),
        })
        .sum::<i32>()
}

fn part_2(input: &str) -> impl Debug {
    let mut scores = input
        .lines()
        .map(|line| parse(line.chars()))
        .filter_map(|res| res.ok())
        .map(|openings| {
            openings
                .iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                })
                .fold(0i64, |curr, value| curr * 5 + value)
        })
        .collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}
