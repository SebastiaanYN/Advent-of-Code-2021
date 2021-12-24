#[allow(unused_imports)]
use advent_of_code_2021::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, one_of},
    IResult,
};
use std::{collections::HashMap, fmt::Debug, ops::RangeInclusive};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

type Range = (i64, i64);
type XYZRange = (Range, Range, Range);

fn parse_range(s: &str) -> IResult<&str, Range> {
    let (s, _) = one_of("xyz")(s)?;
    let (s, _) = tag("=")(s)?;
    let (s, from) = complete::i64(s)?;
    let (s, _) = tag("..")(s)?;
    let (s, to) = complete::i64(s)?;

    Ok((s, (from, to)))
}

fn parse_line(s: &str) -> IResult<&str, (bool, XYZRange)> {
    let (s, on_off) = alt((tag("on"), tag("off")))(s)?;
    let (s, _) = tag(" ")(s)?;

    let (s, x_range) = parse_range(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, y_range) = parse_range(s)?;
    let (s, _) = tag(",")(s)?;
    let (s, z_range) = parse_range(s)?;

    Ok((s, (on_off == "on", (x_range, y_range, z_range))))
}

fn valid_range((from, to): Range) -> bool {
    !(to < -50 || from > 50)
}

fn create_range((from, to): Range) -> RangeInclusive<i64> {
    i64::max(from, -50)..=i64::min(to, 50)
}

fn part_1(input: &str) -> impl Debug {
    let steps = input.lines().map(|line| parse_line(line).unwrap().1);

    // -50..=50 is 101 items
    let mut grid = vec![vec![vec![false; 101]; 101]; 101];

    // loops go brrrrr
    for (state, (x_range, y_range, z_range)) in steps {
        if valid_range(x_range) && valid_range(y_range) && valid_range(z_range) {
            for x in create_range(x_range) {
                for y in create_range(y_range) {
                    for z in create_range(z_range) {
                        if x >= -50 && x <= 50 && y >= -50 && y <= 50 && z >= -50 && z <= 50 {
                            grid[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = state;
                        }
                    }
                }
            }
        }
    }

    grid.into_iter().flatten().flatten().filter(|&x| x).count()
}

fn intersection((x1, y1, z1): XYZRange, (x2, y2, z2): XYZRange) -> XYZRange {
    let x3 = (i64::max(x1.0, x2.0), i64::min(x1.1, x2.1));
    let y3 = (i64::max(y1.0, y2.0), i64::min(y1.1, y2.1));
    let z3 = (i64::max(z1.0, z2.0), i64::min(z1.1, z2.1));

    (x3, y3, z3)
}

fn part_2(input: &str) -> impl Debug {
    let steps = input.lines().map(|line| parse_line(line).unwrap().1);

    let mut cuboids = HashMap::new();

    // Find all intersections of the current cuboid with previous cuboids
    // then set that area to the opposite of the current area
    //
    // The following input can be represented in 1 dimension
    // on x=6..15,y=1..1,z=1..1
    // on x=8..10,y=1..1,z=1..1
    // off x=13..14,y=1..1,z=1..1
    // off x=13..14,y=1..1,z=1..1
    // off x=13..14,y=1..1,z=1..1
    //
    //  on .....##########.....    |
    //   1 .....##########.....  1 | store the cuboid
    //   - .....##########.....    |
    //
    //  on .......###..........    |
    //   1 .....##########.....  1 |
    //   2 .......!!!.......... -1 | overlaps with (1)
    //   3 .......###..........  1 | store the cuboid
    //   - .....##########.....    |
    //
    // off ............##......    |
    //   1 .....##########.....  1 |
    //   2 .......!!!.......... -1 |
    //   3 .......###..........  1 |
    //   4 ............!!...... -1 | overlaps with (1)
    //   - .....#######..#.....    |
    //
    // off ............##......    | turn off the same area as before
    //   1 .....##########.....  1 |
    //   2 .......!!!.......... -1 |
    //   3 .......###..........  1 |
    //   4 ............!!......  0 | overlaps with (4) so we subtract -1 from it which gives 0
    //   5 ............!!...... -1 | overlaps with (1)
    //   - .....#######..#.....    |
    //
    // off ............##......    | turn off the same area as before
    //   1 .....##########.....  1 |
    //   2 .......!!!.......... -1 |
    //   3 .......###..........  1 |
    //   4 ............!!......  0 | overlaps with (4) so we subtract 0 from it which gives 0
    //   5 ............!!......  0 | overlaps with (5) so we subtract -1 from it which gives 0
    //   6 ............!!...... -1 | overlaps with (1)
    //   - .....#######..#.....    |
    for (on, ranges) in steps {
        for (cuboid, count) in cuboids.clone() {
            let (x_range, y_range, z_range) = intersection(ranges, cuboid);

            if x_range.0 <= x_range.1 && y_range.0 <= y_range.1 && z_range.0 <= z_range.1 {
                *cuboids.entry((x_range, y_range, z_range)).or_insert(0) -= count;
            }
        }

        if on {
            *cuboids.entry(ranges).or_insert(0) += 1;
        }
    }

    cuboids
        .iter()
        .map(|((x_range, y_range, z_range), count)| {
            (x_range.1 - x_range.0 + 1)
                * (y_range.1 - y_range.0 + 1)
                * (z_range.1 - z_range.0 + 1)
                * count
        })
        .sum::<i64>()
}
