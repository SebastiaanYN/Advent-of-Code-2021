#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn part_1(input: &str) -> impl Debug {
    let (gamma, epsilon) = input
        .lines()
        .map(|line| line.chars())
        .transpose()
        .map(|line| {
            let (zeros, ones) = line.iter().fold((0, 0), |(zeros, ones), c| match c {
                '0' => (zeros + 1, ones),
                '1' => (zeros, ones + 1),
                _ => unreachable!(),
            });

            (
                if zeros > ones { '0' } else { '1' },
                if zeros < ones { '0' } else { '1' },
            )
        })
        .fold(
            (String::new(), String::new()),
            |(mut gamma, mut epsilon), (g, e)| {
                gamma.push(g);
                epsilon.push(e);

                (gamma, epsilon)
            },
        );

    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}

fn part_2(input: &str) -> impl Debug {
    fn solve<'a>(lines: &Vec<&'a str>, cond: impl Fn((i32, i32)) -> char) -> String {
        if lines.len() == 1 {
            return lines[0].to_owned();
        }

        let x = lines
            .iter()
            .map(|line| (line.chars().nth(0).unwrap(), &line[1..]))
            .collect::<Vec<_>>();

        let (zeros, ones) = x.iter().fold((0, 0), |(zeros, ones), (c, _)| match c {
            '0' => (zeros + 1, ones),
            '1' => (zeros, ones + 1),
            _ => unreachable!(),
        });

        let keep = cond((zeros, ones));

        keep.to_string()
            + &solve(
                &x.into_iter()
                    .filter(|(c, _)| *c == keep)
                    .map(|(_, remainder)| remainder)
                    .collect::<Vec<_>>(),
                cond,
            )
    }

    let lines = input.lines().collect::<Vec<_>>();

    let gamma = solve(&lines, |(zeros, ones)| {
        if zeros == ones {
            '1'
        } else if zeros > ones {
            '0'
        } else {
            '1'
        }
    });

    let epsilon = solve(&lines, |(zeros, ones)| {
        if zeros == ones {
            '0'
        } else if zeros < ones {
            '0'
        } else {
            '1'
        }
    });

    u32::from_str_radix(&gamma, 2).unwrap() * u32::from_str_radix(&epsilon, 2).unwrap()
}
