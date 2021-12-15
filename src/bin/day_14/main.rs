#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::{collections::HashMap, fmt::Debug};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

type Pair = (char, char);

fn parse(input: &str) -> (HashMap<Pair, i64>, HashMap<Pair, char>, char) {
    let (template, pairs) = input.split_once("\n\n").unwrap();
    let last_char = template.chars().last().unwrap();

    let template = template.chars().zip(template.chars().skip(1)).fold(
        HashMap::<_, i64>::new(),
        |mut acc, curr| {
            *acc.entry(curr).or_insert(0) += 1;
            acc
        },
    );

    let pairs = pairs
        .lines()
        .map(|line| {
            let (p, e) = line.split_once(" -> ").unwrap();

            (
                (p.chars().nth(0).unwrap(), p.chars().nth(1).unwrap()),
                e.chars().nth(0).unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();

    (template, pairs, last_char)
}

fn solve(input: &str, steps: usize) -> i64 {
    let (mut template, pairs, last_char) = parse(input);

    for _ in 0..steps {
        template = template
            .into_iter()
            .fold(HashMap::new(), |mut acc, (pair, count)| {
                let element = *pairs.get(&pair).unwrap();

                *acc.entry((pair.0, element)).or_insert(0) += count;
                *acc.entry((element, pair.1)).or_insert(0) += count;

                acc
            });
    }

    let mut counts = template
        .into_iter()
        .fold(HashMap::new(), |mut acc, ((c, _), count)| {
            *acc.entry(c).or_insert(0) += count;
            acc
        });
    *counts.entry(last_char).or_insert(0) += 1;

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part_1(input: &str) -> impl Debug {
    solve(input, 10)
}

fn part_2(input: &str) -> impl Debug {
    solve(input, 40)
}
