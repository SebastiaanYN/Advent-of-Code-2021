day=$1
year=2021

fday=$(printf "%02d" "$day")
dir="./src/bin/day_$fday"

if [ "$day" -lt 1 ] || [ "$day" -gt 25 ]; then
    echo "day should be between 1 and 25"
    exit 1
fi

[ ! -d "$dir" ] && mkdir -p "$dir"

[ ! -f "$dir/main.rs" ] && echo "fn main() {
    println!(\"Day $day\");
}" > "$dir/main.rs"

[ ! -f "$dir/input.txt" ] && curl \
    --cookie "session=$AOC_SESSION" \
    "https://adventofcode.com/$year/day/$day/input" > "$dir/input.txt"

cargo run --bin "day_$fday"
