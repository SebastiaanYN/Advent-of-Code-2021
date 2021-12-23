#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn pad_grid<T: Copy>(grid: &Vec<Vec<T>>, count: usize, x: T) -> Vec<Vec<T>> {
    let mut output = vec![vec![x; grid[0].len() + count * 2]; count];
    output.append(
        &mut grid
            .iter()
            .map(|line| {
                let mut output = vec![x; count];
                output.append(&mut line.clone());
                output.append(&mut vec![x; count]);
                output
            })
            .collect::<Vec<Vec<_>>>(),
    );
    output.append(&mut vec![vec![x; grid[0].len() + count * 2]; count]);
    output
}

fn get_pixels(image: &Vec<Vec<char>>, i: usize, j: usize, bg: char) -> usize {
    let g = |i: usize, j: usize| image.get(i).and_then(|row| row.get(j));

    [
        g(i.wrapping_sub(1), j.wrapping_sub(1)),
        g(i.wrapping_sub(1), j),
        g(i.wrapping_sub(1), j.wrapping_add(1)),
        g(i, j.wrapping_sub(1)),
        g(i, j),
        g(i, j.wrapping_add(1)),
        g(i.wrapping_add(1), j.wrapping_sub(1)),
        g(i.wrapping_add(1), j),
        g(i.wrapping_add(1), j.wrapping_add(1)),
    ]
    .iter()
    .fold(0, |acc, c| {
        acc << 1
            | match c {
                Some('#') => 1,
                None if bg == '#' => 1,
                _ => 0,
            }
    })
}

fn enhance(iea: &Vec<char>, image: &Vec<Vec<char>>, bg: char) -> Vec<Vec<char>> {
    let mut output = image.clone();

    for i in 0..image.len() {
        for j in 0..image[i].len() {
            let pixels = get_pixels(&image, i, j, bg);
            output[i][j] = iea[pixels];
        }
    }

    output
}

fn enhance_n(n: usize, iea: &Vec<char>, image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut image = pad_grid(image, n, '.');

    for i in 0..n {
        image = enhance(iea, &image, if i % 2 == 0 { '.' } else { '#' });
    }

    image
}

fn parse(input: &str) -> (Vec<char>, Vec<Vec<char>>) {
    let (iea, image) = input.split_once("\n\n").unwrap();

    let iea = iea.chars().collect::<Vec<_>>();

    let image = image
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    (iea, image)
}

fn part_1(input: &str) -> impl Debug {
    let (iea, image) = parse(input);

    enhance_n(2, &iea, &image)
        .iter()
        .flatten()
        .filter(|&&c| c == '#')
        .count()
}

fn part_2(input: &str) -> impl Debug {
    let (iea, image) = parse(input);

    enhance_n(50, &iea, &image)
        .iter()
        .flatten()
        .filter(|&&c| c == '#')
        .count()
}
