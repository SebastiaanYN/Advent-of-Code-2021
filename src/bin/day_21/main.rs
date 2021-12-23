#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::{collections::HashMap, fmt::Debug};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn wrap(n: i64, max: i64) -> i64 {
    (n - 1) % max + 1
}

fn part_1(input: &str) -> impl Debug {
    let (pos_1, pos_2) = input.split_once('\n').unwrap();
    let mut pos_1 = pos_1.chars().last().unwrap().to_digit(10).unwrap() as i64;
    let mut pos_2 = pos_2.chars().last().unwrap().to_digit(10).unwrap() as i64;

    let mut score_1 = 0;
    let mut score_2 = 0;

    let mut dice_rolls = 0;
    loop {
        pos_1 = wrap(pos_1 + (3 * dice_rolls + 6), 10);
        score_1 += pos_1;
        dice_rolls += 3;

        if score_1 >= 1000 {
            return score_2 * dice_rolls;
        }

        pos_2 = wrap(pos_2 + (3 * dice_rolls + 6), 10);
        score_2 += pos_2;
        dice_rolls += 3;

        if score_2 >= 1000 {
            return score_1 * dice_rolls;
        }
    }
}

fn part_2(input: &str) -> impl Debug {
    let (pos_1, pos_2) = input.split_once('\n').unwrap();
    let pos_1 = pos_1.chars().last().unwrap().to_digit(10).unwrap() as i64;
    let pos_2 = pos_2.chars().last().unwrap().to_digit(10).unwrap() as i64;

    fn solve(
        memoization: &mut HashMap<(i64, i64, i64, i64), (i64, i64)>,
        pos_1: i64,
        pos_2: i64,
        score_1: i64,
        score_2: i64,
    ) -> (i64, i64) {
        if let Some(&x) = memoization.get(&(pos_1, pos_2, score_1, score_2)) {
            return x;
        }

        if score_2 >= 21 {
            return (0, 1);
        }

        let mut wins = (0, 0);

        for (steps, n) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let pos_1 = wrap(pos_1 + steps, 10);
            let (wins_2, wins_1) = solve(memoization, pos_2, pos_1, score_2, score_1 + pos_1);

            wins = (wins.0 + wins_1 * n, wins.1 + wins_2 * n);
        }

        memoization.insert((pos_1, pos_2, score_1, score_2), wins);
        wins
    }

    let (wins_1, wins_2) = solve(&mut HashMap::new(), pos_1, pos_2, 0, 0);
    i64::max(wins_1, wins_2)
}
