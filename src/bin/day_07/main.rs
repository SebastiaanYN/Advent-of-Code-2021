#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn median<T: Ord>(x: &mut [T]) -> &T {
    x.sort();
    &x[x.len() / 2]
}

fn gauss(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn part_1(input: &str) -> impl Debug {
    let mut input = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let m = *median(&mut input);
    input.iter().map(|x| (x - m).abs()).sum::<i32>()
}

fn part_2(input: &str) -> impl Debug {
    let input = input
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..=max)
        .map(|x| input.iter().map(|y| gauss((x - y).abs())).sum::<i32>())
        .min()
        .unwrap()
}
