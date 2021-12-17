#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn part_1(input: &str) -> impl Debug {
    let (_, y) = input
        .trim_start_matches("target area: ")
        .split_once(", ")
        .unwrap();

    let (y1, _) = y.trim_start_matches("y=").split_once("..").unwrap();
    let y1 = y1.parse::<i32>().unwrap();

    // We can assume there is always some x_start_vel that makes us end up in the landing zone as
    // x_vel will eventually be 0
    //
    // We know you always end up at y=0 with y_vel=-y_start_vel thus we want to know what velocity
    // is needed to land at the bottom of the target area in a single step, which is when the
    // final velocity is y1=-10 and the starting velocity is abs(y1) - 1 = 9
    //
    // target area: x=20..30, y=-10..-5
    //   6 ...............#..#............
    //   5 ...........#........#..........
    //   4 ...............................
    //   3 ......#..............#......... <--
    //   2 ...............................
    //   1 ...............................
    //   0 S....................#......... <--
    //  -1 ...............................
    //  -2 ...............................
    //  -3 ...............................
    //  -4 .....................#.........
    //  -5 ....................TTTTTTTTTTT
    //  -6 ....................TTTTTTTTTTT
    //  -7 ....................TTTTTTTTTTT
    //  -8 ....................TTTTTTTTTTT
    //  -9 ....................T#TTTTTTTTT
    // -10 ....................TTTTTTTTTTT
    //
    // Summing from the starting velocity the highest point is
    // 45 = 9 + 8 + 7 + 6 + 5 + 4 + 3 + 2 + 1
    //    = 9 * (9 + 1) / 2
    //    = (abs(y1) - 1) * (abs(y1) - 1 + 1) / 2
    //    = (abs(y1) - 1) * abs(y1) / 2
    (y1.abs() - 1) * y1.abs() / 2
}

fn part_2(input: &str) -> impl Debug {
    let (x, y) = input
        .trim_start_matches("target area: ")
        .split_once(", ")
        .unwrap();

    let (x_from, x_to) = x.trim_start_matches("x=").split_once("..").unwrap();
    let (x_from, x_to) = (x_from.parse::<i32>().unwrap(), x_to.parse::<i32>().unwrap());

    let (y_from, y_to) = y.trim_start_matches("y=").split_once("..").unwrap();
    let (y_from, y_to) = (y_from.parse::<i32>().unwrap(), y_to.parse::<i32>().unwrap());

    // Example and puzzle input show that x_vel has to be somewhere between 0 and 174
    // and y_vel has to be somewhere between -123 and 122
    //
    // In this case 0 <= x_vel <= 174 and -123 <= y_vel <= 122 works
    //
    // target area: x=20..30, y=-10..-5
    // target area: x=124..174, y=-123..-86
    (0..=174)
        .map(|x_start_vel| {
            (-123..=122)
                .map(move |y_start_vel| {
                    let mut x = 0;
                    let mut y = 0;

                    let mut x_vel = x_start_vel;
                    let mut y_vel = y_start_vel;

                    loop {
                        x += x_vel;
                        y += y_vel;

                        if x_vel > 0 {
                            x_vel -= 1;
                        } else if x < 0 {
                            x_vel += 1;
                        }
                        y_vel -= 1;

                        if (x_from..=x_to).contains(&x) && (y_from..=y_to).contains(&y) {
                            break Some((x_start_vel, y_start_vel));
                        } else if x > x_to || y < y_from {
                            break None;
                        }
                    }
                })
                .flatten()
        })
        .flatten()
        .count()
}
