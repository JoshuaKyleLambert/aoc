use crate::{number::Number, symbol::Symbol};
use aoc_library::read_file_to_vec;

#[derive(Default)]
pub struct Schematic {
    contents: Vec<String>,
    part_numbers: Vec<Number>,
    all_numbers: Vec<Number>,
    all_symbols: Vec<Symbol>,
    size_x: u32,
    size_y: u32,
}

pub fn process_input(path: &str) -> Schematic {
    let contents = read_file_to_vec(path).unwrap();
    Schematic::new(contents)
}

impl Schematic {
    pub fn new(contents: Vec<String>) -> Self {
        let mut schematic: Schematic = Schematic {
            contents,
            ..Default::default()
        };
        schematic.process_contents();
        schematic
    }

    pub fn part_number_sum(&self) -> u32 {
        self.part_numbers.iter().map(|num| num.value).sum()
    }

    pub fn line_length(&self) -> u32 {
        let len = if let Some(string) = self.contents.first() {
            string.len()
        } else {
            0
        };

        len.try_into().unwrap()
    }

    pub fn file_size(&self) -> u32 {
        self.contents.len().try_into().unwrap()
    }

    pub fn process_contents(&mut self) {
        self.size_x = self.line_length();
        self.size_y = self.file_size();

        for (y, line) in self.contents.iter().enumerate() {
            let y: u32 = y.try_into().unwrap();

            let mut numbers = Schematic::extract_numbers_from_string(line, y);
            self.all_numbers.append(&mut numbers);

            let mut symbols = Schematic::extract_symbols_from_string(line, y);
            self.all_symbols.append(&mut symbols);
        }

        self.part_numbers.append(&mut self.scan_for_part_numbers());
    }

    fn extract_symbols_from_string(string: &str, y: u32) -> Vec<Symbol> {
        let mut symbols = Vec::new();

        for (index, c) in (0_u32..).zip(string.chars()) {
            let symbol = match c {
                '*' | '#' | '%' | '-' | '$' | '@' | '/' | '&' | '=' | '+' => Symbol {
                    pos: (index, y),
                    symbol: c,
                },
                _ => continue,
            };

            symbols.push(symbol);
        }

        symbols
    }

    fn extract_numbers_from_string(string: &str, line: u32) -> Vec<Number> {
        let mut number = Number::default();
        let mut numbers = Vec::new();
        let mut current_number: u32 = 0;
        let mut num_found: bool = false;

        for (index, c) in (0_u32..).zip(string.chars()) {
            if c.is_ascii_digit() {
                num_found = true;
                let (x, y) = &mut number.pos;
                if *x == 0 {
                    *x = index
                }

                *y = line;
                number.length += 1;

                if current_number == 0 {
                    current_number = c.to_digit(10).unwrap();
                } else {
                    current_number *= 10;
                    current_number += c.to_digit(10).unwrap();
                }
            }

            let last_char_was_number = index == string.len() as u32 - 1 && num_found;
            let end_of_number_found = num_found && !c.is_ascii_digit();

            if end_of_number_found || last_char_was_number {
                number.value = current_number;
                numbers.push(number);
                number = Number::default();
                current_number = 0;
                num_found = false;
            }
        }
        numbers
    }

    pub fn get_char_at_coord(&self, x: u32, y: u32) -> Option<char> {
        if x >= self.size_x || y >= self.size_y {
            return None;
        }

        self.contents[y as usize].chars().nth(x as usize)
    }

    pub fn scan_for_part_numbers(&self) -> Vec<Number> {
        let mut part_numbers = Vec::new();
        for number in &self.all_numbers {
            if self
                .all_symbols
                .iter()
                .any(|symbol| self.symbol_is_next_to_number(symbol, number))
            {
                println!("Found part number: {:?}", number);
                part_numbers.push(*number);
            } else {
                println!("{:?} Is not a part number", number)
            }
        }

        part_numbers
    }

    fn symbol_is_next_to_number(&self, symbol: &Symbol, number: &Number) -> bool {
        let (x, y) = symbol.pos;
        let (num_x, num_y) = number.pos;

        let mut sub_x = 1;
        let mut sub_y = 1;

        if num_x == 0 {
            sub_x = 0;
        }
        if num_y == 0 {
            sub_y = 0;
        }

        (x >= num_x - sub_x && x <= num_x + number.length) && (y >= num_y - sub_y && y <= num_y + 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::schematic;

    use super::*;

    #[test]
    fn test_scan_current_line_for_symbol() {}

    #[test]
    fn test_scan_next_line_for_symbol() {}

    #[test]
    fn test_extract_symbols_from_string() {
        let string = "1.2......33.....45...678...9.10";
        let symbols = Schematic::extract_symbols_from_string(string, 0);
        assert!(symbols.is_empty());

        let string = "...0*...";
        let symbols = Schematic::extract_symbols_from_string(string, 0);
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0].pos, (4, 0));

        let string = "..*.#..";
        let symbols = Schematic::extract_symbols_from_string(string, 0);
        assert_eq!(symbols.len(), 2);
        assert_eq!(symbols[0].pos, (2, 0));
        assert_eq!(symbols[0].symbol, '*');
        assert_eq!(symbols[1].pos, (4, 0));
        assert_eq!(symbols[1].symbol, '#');

        let string = ".......................153..988....502..842.........588.....441.468......481..........314...715.57............................163..992..512.";
        let symbols = Schematic::extract_symbols_from_string(string, 0);
        assert_eq!(symbols.len(), 0);

        let string = "............805............*......#.............%...............*........=......%......................#......*.............-....#....*.....";
        let symbols = Schematic::extract_symbols_from_string(string, 0);
        assert_eq!(symbols.len(), 11);
        assert_eq!(symbols[0].pos, (27, 0));
        assert_eq!(symbols[0].symbol, '*');
    }

    #[test]
    fn test_extract_numbers_from_string_with_periods() {
        let string = "1.2......33.....45...678...9.10";
        let numbers = Schematic::extract_numbers_from_string(string, 0);
        assert_eq!(numbers.len(), 7);
        assert_eq!(numbers[0].value, 1);
        assert_eq!(numbers[1].value, 2);
        assert_eq!(numbers[2].value, 33);
        assert_eq!(numbers[3].value, 45);
        assert_eq!(numbers[4].value, 678);
        assert_eq!(numbers[5].value, 9);
        assert_eq!(numbers[6].value, 10);
    }

    #[test]
    fn test_extract_numbers_from_string_ends_with_periods() {
        let string = "1.2......33.....45...678...9.10.....";
        let numbers = Schematic::extract_numbers_from_string(string, 0);
        assert_eq!(numbers.len(), 7);
        assert_eq!(numbers[0].value, 1);
        assert_eq!(numbers[1].value, 2);
        assert_eq!(numbers[2].value, 33);
        assert_eq!(numbers[3].value, 45);
        assert_eq!(numbers[4].value, 678);
        assert_eq!(numbers[5].value, 9);
        assert_eq!(numbers[6].value, 10);
    }

    #[test]
    fn test_extract_numbers_from_string_starts_and_ends_with_periods() {
        let string = "......1.2......33.....45...678...9.10.....";
        let numbers = Schematic::extract_numbers_from_string(string, 0);
        assert_eq!(numbers.len(), 7);
        assert_eq!(numbers[0].value, 1);
        assert_eq!(numbers[1].value, 2);
        assert_eq!(numbers[2].value, 33);
        assert_eq!(numbers[3].value, 45);
        assert_eq!(numbers[4].value, 678);
        assert_eq!(numbers[5].value, 9);
        assert_eq!(numbers[6].value, 10);
    }
}
