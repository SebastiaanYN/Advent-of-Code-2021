#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn part_1(input: &str) -> impl Debug {
    input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .fold(&mut [[0; 1000]; 1000], |grid, (from, to)| {
            let (x1, y1) = from.split_once(',').unwrap();
            let (x1, y1) = (x1.parse().unwrap(), y1.parse().unwrap());

            let (x2, y2) = to.split_once(',').unwrap();
            let (x2, y2) = (x2.parse().unwrap(), y2.parse().unwrap());

            if x1 == x2 || y1 == y2 {
                for x in usize::min(x1, x2)..=usize::max(x1, x2) {
                    for y in usize::min(y1, y2)..=usize::max(y1, y2) {
                        grid[y][x] += 1;
                    }
                }
            }

            grid
        })
        .iter()
        .flatten()
        .filter(|&&x| x >= 2)
        .count()
}

fn part_2(input: &str) -> impl Debug {
    input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .fold(&mut [[0; 1000]; 1000], |grid, (from, to)| {
            let (x1, y1) = from.split_once(',').unwrap();
            let (x1, y1) = (x1.parse().unwrap(), y1.parse().unwrap());

            let (x2, y2) = to.split_once(',').unwrap();
            let (x2, y2) = (x2.parse().unwrap(), y2.parse().unwrap());

            let xs: Box<dyn Iterator<Item = usize>> = if x1 == x2 {
                Box::new(std::iter::repeat(x1))
            } else if x1 > x2 {
                Box::new((x2..=x1).rev())
            } else {
                Box::new(x1..=x2)
            };

            let ys: Box<dyn Iterator<Item = usize>> = if y1 == y2 {
                Box::new(std::iter::repeat(y1))
            } else if y1 > y2 {
                Box::new((y2..=y1).rev())
            } else {
                Box::new(y1..=y2)
            };

            xs.zip(ys).for_each(|(x, y)| {
                grid[y][x] += 1;
            });

            grid
        })
        .iter()
        .flatten()
        .filter(|&&x| x >= 2)
        .count()
}
