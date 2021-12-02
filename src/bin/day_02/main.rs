use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn part_1(input: &str) -> impl Debug {
    let (horizontal, depth) = input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            (
                line.next().unwrap(),
                line.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold((0, 0), |(horizontal, depth), line| match line {
            ("forward", x) => (horizontal + x, depth),
            ("down", x) => (horizontal, depth + x),
            ("up", x) => (horizontal, depth - x),
            _ => unreachable!(),
        });

    horizontal * depth
}

fn part_2(input: &str) -> impl Debug {
    let (horizontal, depth, _) = input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            (
                line.next().unwrap(),
                line.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold((0, 0, 0), |(horizontal, depth, aim), line| match line {
            ("forward", x) => (horizontal + x, depth + x * aim, aim),
            ("down", x) => (horizontal, depth, aim + x),
            ("up", x) => (horizontal, depth, aim - x),
            _ => unreachable!(),
        });

    horizontal * depth
}
