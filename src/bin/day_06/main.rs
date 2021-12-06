#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::{collections::HashMap, fmt::Debug};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn part_1(input: &str) -> impl Debug {
    let mut fish = input
        .split(',')
        .map(|timer| timer.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..80 {
        let mut append = Vec::new();

        for timer in &mut fish {
            if *timer == 0 {
                *timer = 6;
                append.push(8);
            } else {
                *timer -= 1;
            }
        }

        fish.append(&mut append);
    }

    fish.len()
}

fn part_2(input: &str) -> impl Debug {
    let mut fish = input
        .split(',')
        .map(|timer| timer.parse::<i32>().unwrap())
        .fold(HashMap::<_, i64>::new(), |mut map, timer| {
            *map.entry(timer).or_insert(0) += 1;
            map
        });

    for _ in 0..256 {
        let zeros = *fish.get(&0).unwrap_or(&0);

        fish = fish
            .iter()
            .map(|(&timer, &count)| {
                if timer == 0 {
                    (8, zeros)
                } else {
                    (timer - 1, count)
                }
            })
            .collect();

        *fish.entry(6).or_insert(0) += zeros;
    }

    fish.values().sum::<i64>()
}
