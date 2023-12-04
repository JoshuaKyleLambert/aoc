//Adventure 2020 day 2
use day_2::play_games;
use day_2::BagPull;

fn main() {
    let limit = BagPull {
        red: 12,
        green: 13,
        blue: 14,
    };

    println!("Game Results: \n{}", play_games(&limit));
}
