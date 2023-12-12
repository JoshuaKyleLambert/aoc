use aoc_library::*;
use day_4::scratch_card::*;

fn main() {
    let input = read_file_to_vec("./2023/day_4/input.txt").unwrap();

    print!("Total Winnings: {}", total_winnings(parse_input(input)));
}
