#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn inc_grid(grid: &mut Vec<Vec<u32>>, i: i32, j: i32) {
    let i = i as usize;
    let j = j as usize;

    if i < grid.len() && j < grid[i].len() {
        grid[i][j] += 1;
    }
}

fn step(grid: &mut Vec<Vec<u32>>) -> usize {
    grid.iter_mut().flatten().for_each(|x| *x += 1);

    let mut has_flashed = Vec::new();

    loop {
        let mut changed = false;

        let mut to_flash = Vec::new();
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 9 && !has_flashed.contains(&(i, j)) {
                    to_flash.push((i, j));
                }
            }
        }

        for (i, j) in to_flash {
            has_flashed.push((i, j));

            let i = i as i32;
            let j = j as i32;

            for (i, j) in [
                (i + 1, j),
                (i - 1, j),
                (i, j + 1),
                (i, j - 1),
                (i + 1, j + 1),
                (i + 1, j - 1),
                (i - 1, j + 1),
                (i - 1, j - 1),
            ] {
                inc_grid(grid, i, j);
            }

            changed = true;
        }

        if !changed {
            break;
        }
    }

    for &(i, j) in &has_flashed {
        grid[i][j] = 0;
    }

    has_flashed.len()
}

fn part_1(input: &str) -> impl Debug {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<_>>>();

    let mut total_flashes = 0;

    for _ in 0..100 {
        total_flashes += step(&mut grid);
    }

    total_flashes
}

fn part_2(input: &str) -> impl Debug {
    let mut grid = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<_>>>();

    for iteration in 1.. {
        step(&mut grid);

        if grid.iter().all(|column| column.iter().all(|&x| x == 0)) {
            return iteration;
        }
    }

    unreachable!()
}
