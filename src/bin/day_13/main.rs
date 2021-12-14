#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn fold_up(grid: &Vec<Vec<bool>>, idx: usize) -> Vec<Vec<bool>> {
    grid[..idx]
        .iter()
        .zip(grid[idx..].iter().rev())
        .map(|(x, y)| x.iter().zip(y).map(|(&a, &b)| a || b).collect())
        .collect()
}

fn fold_left(grid: &Vec<Vec<bool>>, idx: usize) -> Vec<Vec<bool>> {
    grid.iter()
        .map(|row| {
            row[..idx]
                .iter()
                .zip(row[idx..].iter().rev())
                .map(|(&a, &b)| a || b)
                .collect()
        })
        .collect()
}

fn fold(grid: &Vec<Vec<bool>>, direction: char, idx: usize) -> Vec<Vec<bool>> {
    match direction {
        'x' => fold_left(grid, idx),
        'y' => fold_up(grid, idx),
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> (Vec<Vec<bool>>, Vec<(char, usize)>) {
    let (points, folds) = input.split_once("\n\n").unwrap();

    let points = points
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();

            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let width = points.iter().map(|&(x, _)| x).max().unwrap();
    let height = points.iter().map(|&(_, y)| y).max().unwrap();

    let mut grid = vec![vec![false; width + 1]; height + 1];
    points.iter().for_each(|&(x, y)| {
        grid[y][x] = true;
    });

    let folds = folds
        .lines()
        .map(|fold| {
            let (direction, idx) = fold
                .trim_start_matches("fold along ")
                .split_once('=')
                .unwrap();

            (
                direction.chars().nth(0).unwrap(),
                idx.parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    (grid, folds)
}

fn part_1(input: &str) -> impl Debug {
    let (mut grid, folds) = parse(input);

    let (direction, idx) = folds[0];
    grid = fold(&grid, direction, idx);

    grid.iter().flatten().filter(|&&x| x).count()
}

fn part_2(input: &str) -> impl Debug {
    let (mut grid, folds) = parse(input);

    for (direction, idx) in folds {
        grid = fold(&grid, direction, idx);
    }

    let output = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|&x| if x { '#' } else { '.' })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("{}", output);
}
