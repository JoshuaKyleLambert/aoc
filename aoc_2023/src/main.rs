//Advent of code Day one
use std::fs::File;
use std::io::{prelude, BufRead, BufReader, Read};

fn main() {
    println!("Hello, world!");
    let mut strings: Vec<String> = Vec::new();

    match read_file_to_vec("input.txt") {
        Ok(strs) => strings = strs,
        Err(error) => print!("{}", error),
    };
    remove_letters(&mut strings);
    println!("{}", sum_strings(&mut strings));
}

fn read_file_to_vec(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn remove_letters(strings: &mut Vec<String>) {
    for c in strings {
        c.retain(|c| c.is_ascii_digit());
    }
}

fn sum_strings(strings: &mut Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    for c in strings {
        sum += get_string_value(c);
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
