use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Default)]
pub struct Game {
    pub number: u32,
    pub pulls: Vec<BagPull>,
}

impl Game {
    pub fn is_possible_by_limit(&self, limit: &BagPull) -> bool {
        for pull in self.pulls.iter() {
            if pull.red > limit.red || pull.green > limit.green || pull.blue > limit.blue {
                return false;
            }
        }
        true
    }

    pub fn fewest_cubes_needed(&self) -> BagPull {
        let mut max_colors = BagPull::default();

        for pull in self.pulls.iter() {
            if pull.red > max_colors.red {
                max_colors.red = pull.red;
            }
            if pull.green > max_colors.green {
                max_colors.green = pull.green;
            }
            if pull.blue > max_colors.blue {
                max_colors.blue = pull.blue;
            }
        }

        max_colors
    }
}

pub fn power_of_set(set: &BagPull) -> u32 {
    set.red * set.green * set.blue
}

pub fn sum_of_powers_of_games(games: &[Game]) -> u32 {
    let mut sum = 0;
    for game in games.iter() {
        let power = power_of_set(&game.fewest_cubes_needed());
        sum += power;
    }
    sum
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BagPull {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

pub fn read_file_to_vec(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}

pub fn play_games(limit: &BagPull) -> String {
    let mut strings: Vec<String> = Vec::new();
    match read_file_to_vec("input.txt") {
        Ok(strs) => strings = strs,
        Err(error) => print!("{}", error),
    };

    let mut results = String::from("Sum of possible games ");

    let games = process_all_lines_into_games(&strings);
    let sum = sum_possible_games(&games, limit);
    results.push_str(sum.to_string().as_str());

    let sum_of_powers = sum_of_powers_of_games(&games);

    results.push_str("\nSum of powers of games ");
    results.push_str(sum_of_powers.to_string().as_str());

    results
}

pub fn sum_possible_games(games: &[Game], limit: &BagPull) -> u32 {
    let mut sum = 0;
    for game in games.iter() {
        if game.is_possible_by_limit(limit) {
            sum += game.number;
        }
    }
    sum
}

pub fn process_all_lines_into_games(lines: &[String]) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for line in lines.iter() {
        let game = process_line_into_game(line);
        games.push(game);
    }
    games
}

pub fn process_line_into_game(line: &str) -> Game {
    let (game, pulls) = split_into_game_and_pulls(line);
    let game_number = extract_game_number(game);
    let pulls = extract_pulls(pulls);
    let pull_vec = build_bag_pulls_vec(&pulls);

    Game {
        number: game_number,
        pulls: pull_vec,
    }
}

pub fn split_into_game_and_pulls(line: &str) -> (&str, &str) {
    let game = line.split(':').next().unwrap();
    let pulls = line.split(':').last().unwrap();
    (game, pulls)
}

pub fn extract_game_number(line: &str) -> u32 {
    if let Some(num) = line.split(' ').last() {
        num.parse::<u32>().unwrap()
    } else {
        0
    }
}

pub fn extract_pulls(line: &str) -> Vec<&str> {
    line.split(';').collect()
}

pub fn build_bag_pulls_vec(pulls: &[&str]) -> Vec<BagPull> {
    let mut pulls_vec: Vec<BagPull> = Vec::new();
    for pull in pulls.iter() {
        let cubes = extract_cube_count(pull);
        pulls_vec.push(cubes);
    }

    pulls_vec
}

pub fn extract_cube_count(line: &str) -> BagPull {
    let mut colors: Vec<&str> = line.split(',').collect();
    let mut pull = BagPull::default();

    for color in colors.iter_mut() {
        *color = color.trim();
        let color_split: Vec<&str> = color.split(' ').collect();
        let color_count = color_split[0].parse::<u32>().unwrap();
        let color_name = color_split[1];

        match color_name {
            "red" => pull.red = color_count,
            "green" => pull.green = color_count,
            "blue" => pull.blue = color_count,
            _ => (),
        }
    }

    pull
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_to_vec() {
        let mut strings: Vec<String> = Vec::new();
        match read_file_to_vec("input.txt") {
            Ok(strs) => strings = strs,
            Err(error) => print!("{}", error),
        };
        assert_eq!(strings.len(), 100);
    }

    #[test]
    fn test_extract_game_number() {
        let game_1 = extract_game_number("Game 1");
        let game_22 = extract_game_number("Game 22");
        let game_100 = extract_game_number("blah blah blah 100");

        assert_eq!(game_1, 1);
        assert_eq!(game_22, 22);
        assert_eq!(game_100, 100);
    }

    #[test]
    fn test_extract_pulls_returns_vec_of_bag_pulls() {
        let string =
            "4 red, 1 green, 15 blue; 6 green, 2 red, 10 blue; 7 blue, 6 green, 4 red; 12 blue, 10 green, 3 red";
        let pulls = extract_pulls(string);
        assert_eq!(pulls[0], "4 red, 1 green, 15 blue");
    }

    #[test]
    fn test_extract_cubes_has_all_colors() {
        let string = "4 red, 1 green, 15 blue";
        let cubes: BagPull = extract_cube_count(string);
        assert_eq!(cubes.red, 4);
        assert_eq!(cubes.green, 1);
        assert_eq!(cubes.blue, 15);
    }

    #[test]
    fn test_extract_cubes_has_one_color() {
        let string = "1 red";
        let cubes: BagPull = extract_cube_count(string);
        assert_eq!(cubes.red, 1);
        assert_eq!(cubes.green, 0);
        assert_eq!(cubes.blue, 0);
    }

    #[test]
    fn test_process_line_returns_game() {
        let line = "Game 1: 4 red, 1 green, 15 blue; 6 green, 2 red, 10 blue; 7 blue, 6 green, 4 red; 12 blue, 10 green, 3 red";
        let limit = BagPull {
            red: 12,
            green: 13,
            blue: 14,
        };
        let game = process_line_into_game(line);
        assert_eq!(game.number, 1);
        assert!(game.is_possible_by_limit(&limit));
        assert_eq!(game.pulls[0].red, 4);
        assert_eq!(game.pulls[0].green, 1);
        assert_eq!(game.pulls[0].blue, 15);
        assert_eq!(game.pulls[3].red, 3);
        assert_eq!(game.pulls[3].green, 10);
        assert_eq!(game.pulls[3].blue, 12);
        assert_eq!(game.pulls.len(), 4);
    }

    #[test]
    fn test_impossible_game() {
        let line = "Game 1: 4 red, 1 green, 15 blue; 6 green, 2 red, 10 blue; 7 blue, 6 green, 4 red; 12 blue, 10 green, 3 red";
        let limit = BagPull {
            red: 3,
            green: 3,
            blue: 3,
        };
        let game = process_line_into_game(line);

        game.is_possible_by_limit(&limit);
        assert!(!game.is_possible_by_limit(&limit));
    }
}
