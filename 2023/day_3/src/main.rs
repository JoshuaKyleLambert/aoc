use day_3::schematic::process_input;

fn main() {
    let schematic = process_input("./2023/day_3/input.txt");
    println!("Sum of part numbers: {}", schematic.part_number_sum());
    println!("Sum of gear ratios: {}", schematic.gear_ratio_sum());
}
