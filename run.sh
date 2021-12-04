day=$1
year=2021

fday=$(printf "%02d" "$day")
dir="./src/bin/day_$fday"

if [ "$day" -lt 1 ] || [ "$day" -gt 25 ]; then
    echo "day should be between 1 and 25"
    exit 1
fi

[ ! -d "$dir" ] && mkdir -p "$dir"

[ ! -f "$dir/main.rs" ] && echo "#[allow(unused_imports)]
use advent_of_code_2021::*;
use std::fmt::Debug;

fn main() {
    let input = include_str!(\"./input.txt\").trim_end();

    println!(\"{:?}\", part_1(input));
    println!(\"{:?}\", part_2(input));
}

fn part_1(input: &str) -> impl Debug {}

fn part_2(input: &str) -> impl Debug {}" > "$dir/main.rs"

[ ! -f "$dir/input.txt" ] && curl \
    --cookie "session=$AOC_SESSION" \
    "https://adventofcode.com/$year/day/$day/input" > "$dir/input.txt"

cargo run --bin "day_$fday"
