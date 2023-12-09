//Advent of code Day one
use aoc_library::read_file_to_vec;

fn main() {
    let mut strings: Vec<String> = Vec::new();
    let mut strings_unchanged: Vec<String> = Vec::new();
    match read_file_to_vec("input.txt") {
        Ok(strs) => {
            strings = strs.clone();
            strings_unchanged = strs;
        }
        Err(error) => print!("{}", error),
    };
    replace_spelled_numbers(&mut strings);
    remove_letters(&mut strings);
    println!(
        "The calculated sum is: {}",
        sum_strings(&mut strings, &strings_unchanged)
    );
}

fn replace_spelled_numbers(input: &mut Vec<String>) {
    for c in input {
        *c = c
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
            .replace("zero", "z0o");
    }
}

fn remove_letters(strings: &mut Vec<String>) {
    for c in strings {
        c.retain(|c| c.is_ascii_digit());
    }
}

fn sum_strings(strings: &mut [String], strings_unchanged: &[String]) -> u32 {
    let mut sum: u32 = 0;
    for (i, c) in strings.iter().enumerate() {
        let value = get_string_value(c);
        println!(
            "{}. {} - {} => {} + {} = {}",
            i + 1,
            strings_unchanged[i],
            c,
            value,
            sum,
            sum + value
        );
        sum += value;
    }
    sum
}

fn get_string_value(string: &String) -> u32 {
    string.chars().nth(0).unwrap().to_digit(10).unwrap() * 10
        + string
            .chars()
            .nth(string.len() - 1)
            .unwrap()
            .to_digit(10)
            .unwrap()
}
