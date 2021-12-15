#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn neighbors((i, j): (usize, usize)) -> [(usize, usize); 4] {
    [
        (i + 1, j),
        (i.wrapping_sub(1), j),
        (i, j + 1),
        (i, j.wrapping_sub(1)),
    ]
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    grid: &Vec<Vec<usize>>,
    source: (usize, usize),
    target: (usize, usize),
) -> Option<usize> {
    let mut dist = HashMap::new();
    dist.insert(source, 0);

    #[allow(non_snake_case)]
    let mut Q = BinaryHeap::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let v = (i, j);

            if v != source {
                dist.insert(v, usize::MAX);
            }

            Q.push(State {
                position: v,
                cost: *dist.get(&v)?,
            });
        }
    }

    while let Some(State { position: u, cost }) = Q.pop() {
        if u == target {
            return Some(cost);
        }

        for v in neighbors(u) {
            if let Some(&length) = grid.get(v.0).and_then(|column| column.get(v.1)) {
                let alt = *dist.get(&u)? + length;

                if alt < *dist.get(&v)? {
                    dist.insert(v, alt);
                    Q.push(State {
                        position: v,
                        cost: alt,
                    });
                }
            }
        }
    }

    None
}

fn part_1(input: &str) -> impl Debug {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let i = grid.len() - 1;
    let j = grid[i].len() - 1;

    dijkstra(&grid, (0, 0), (i, j)).unwrap()
}

fn part_2(input: &str) -> impl Debug {
    // LOL
    let grid = (0..5)
        .map(|i| {
            input
                .lines()
                .map(|line| {
                    (0..5)
                        .map(|j| {
                            line.chars()
                                .map(|c| c.to_digit(10).unwrap() as usize + i + j)
                                .map(|x| if x > 9 { x - 9 } else { x })
                                .collect::<Vec<_>>()
                        })
                        .flatten()
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<Vec<_>>>();

    let i = grid.len() - 1;
    let j = grid[i].len() - 1;

    dijkstra(&grid, (0, 0), (i, j)).unwrap()
}
