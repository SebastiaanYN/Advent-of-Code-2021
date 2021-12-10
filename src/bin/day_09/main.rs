#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::{collections::HashSet, fmt::Debug};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn is_low_point(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> bool {
    let curr = grid[i][j];

    if (i as i32 - 1 >= 0 && curr >= grid[i - 1][j])
        || (i + 1 < grid.len() && curr >= grid[i + 1][j])
        || (j as i32 - 1 >= 0 && curr >= grid[i][j - 1])
        || (j + 1 < grid[i].len() && curr >= grid[i][j + 1])
    {
        false
    } else {
        true
    }
}

fn find_basin(
    grid: &Vec<Vec<u32>>,
    i: usize,
    j: usize,
    low: u32,
    out: &mut HashSet<(usize, usize)>,
) {
    if out.contains(&(i, j)) || low == 9 {
        return;
    }

    out.insert((i, j));

    if i as i32 - 1 >= 0 && low < grid[i - 1][j] {
        find_basin(grid, i - 1, j, grid[i - 1][j], out);
    }

    if i + 1 < grid.len() && low < grid[i + 1][j] {
        find_basin(grid, i + 1, j, grid[i + 1][j], out);
    }

    if j as i32 - 1 >= 0 && low < grid[i][j - 1] {
        find_basin(grid, i, j - 1, grid[i][j - 1], out);
    }

    if j + 1 < grid[i].len() && low < grid[i][j + 1] {
        find_basin(grid, i, j + 1, grid[i][j + 1], out);
    }
}

fn part_1(input: &str) -> impl Debug {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<_>>>();

    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_low_point(&grid, i, j) {
                sum += grid[i][j] + 1;
            }
        }
    }

    sum
}

fn part_2(input: &str) -> impl Debug {
    let grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<_>>>();

    let mut lows = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if is_low_point(&grid, i, j) {
                lows.push(((i, j), grid[i][j]));
            }
        }
    }

    let mut basins = Vec::new();

    for ((i, j), low) in lows {
        let mut out = HashSet::new();
        find_basin(&grid, i, j, low, &mut out);
        basins.push(out.len());
    }

    basins.sort();
    basins.iter().rev().take(3).product::<usize>()
}
