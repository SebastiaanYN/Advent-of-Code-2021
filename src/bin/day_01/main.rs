use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn part_1(input: &str) -> impl Debug {
    let input = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    input
        .iter()
        .zip(input.iter().skip(1))
        .filter(|(x, y)| x < y)
        .count()
}

fn part_2(input: &str) -> impl Debug {
    let input = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    input
        .iter()
        .zip(input.iter().skip(3))
        .filter(|(x, y)| x < y)
        .count()
}
