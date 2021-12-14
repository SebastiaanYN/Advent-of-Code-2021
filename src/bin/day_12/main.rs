#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn dfs<'a>(
    graph: &Vec<(&'a str, &'a str)>,
    path: &mut Vec<&'a str>,
    cave: &'a str,
    mut has_visited_small: bool,
) -> i32 {
    if cave == "end" {
        return 1;
    }

    if cave.chars().all(|c| c.is_lowercase()) && path.contains(&cave) {
        if cave == "start" || has_visited_small {
            return 0;
        } else {
            has_visited_small = true;
        }
    }

    path.push(cave);

    let total = graph
        .iter()
        .filter(|&&(from, _)| from == cave)
        .map(|&(_, to)| dfs(graph, path, to, has_visited_small))
        .sum();

    path.pop();
    total
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();

            [(from, to), (to, from)]
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn part_1(input: &str) -> impl Debug {
    let graph = parse(input);

    dfs(&graph, &mut Vec::new(), "start", true)
}

fn part_2(input: &str) -> impl Debug {
    let graph = parse(input);

    dfs(&graph, &mut Vec::new(), "start", false)
}
