#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

type Snailfish = Vec<(u32, u8)>;

fn parse_snailfish(input: &str) -> Snailfish {
    let mut depth = 0;
    let mut snailfish = Vec::new();

    for c in input.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            _ if c.is_digit(10) => snailfish.push((c.to_digit(10).unwrap(), depth)),
            _ => {}
        }
    }

    snailfish
}

fn add(a: &Snailfish, b: &Snailfish) -> Snailfish {
    a.iter()
        .map(|&(x, depth)| (x, depth + 1))
        .chain(b.iter().map(|&(y, depth)| (y, depth + 1)))
        .collect()
}

fn explode(snailfish: &mut Snailfish) -> bool {
    for i in 0..snailfish.len() - 1 {
        let (x, x_depth) = snailfish[i];
        let (y, y_depth) = snailfish[i + 1];

        if x_depth > 4 && y_depth > 4 {
            snailfish.remove(i);
            snailfish.remove(i);

            if i > 0 {
                snailfish[i - 1].0 += x;
            }

            snailfish.insert(i, (0, x_depth - 1));

            if i + 1 < snailfish.len() {
                snailfish[i + 1].0 += y;
            }

            return true;
        }
    }

    false
}

fn split(snailfish: &mut Snailfish) -> bool {
    for i in 0..snailfish.len() {
        let (x, depth) = snailfish[i];

        if x >= 10 {
            snailfish[i] = (x / 2, depth + 1);
            snailfish.insert(i + 1, ((x as f32 / 2.0).ceil() as u32, depth + 1));

            return true;
        }
    }

    false
}

fn reduce(mut snailfish: Snailfish) -> Snailfish {
    while explode(&mut snailfish) || split(&mut snailfish) {}

    snailfish
}

fn magnitude(mut snailfish: Snailfish) -> u32 {
    fn m(snailfish: &mut Snailfish) -> bool {
        for i in 0..snailfish.len() - 1 {
            let (x, x_depth) = snailfish[i];
            let (y, y_depth) = snailfish[i + 1];

            if x_depth == y_depth {
                snailfish.remove(i);
                snailfish[i] = (3 * x + 2 * y, x_depth - 1);

                return true;
            }
        }

        false
    }

    while m(&mut snailfish) {}

    snailfish[0].0
}

fn part_1(input: &str) -> impl Debug {
    let snailfish = input
        .lines()
        .map(parse_snailfish)
        .reduce(|a, b| reduce(add(&a, &b)))
        .unwrap();

    magnitude(snailfish)
}

fn part_2(input: &str) -> impl Debug {
    let snailfishes = input.lines().map(parse_snailfish).collect::<Vec<_>>();

    let mut max = 0;

    for a in &snailfishes {
        for b in &snailfishes {
            let m = magnitude(reduce(add(a, b)));

            if m > max {
                max = m;
            }
        }
    }

    max
}
