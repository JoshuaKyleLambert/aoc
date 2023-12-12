pub struct ScratchCard {
    card_number: u32,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl ScratchCard {
    pub fn new((card_number, numbers, winning_numbers): (u32, Vec<u32>, Vec<u32>)) -> ScratchCard {
        ScratchCard {
            card_number,
            numbers,
            winning_numbers,
        }
    }

    pub fn point_total(&self) -> u32 {
        let mut points = 0;

        for number in &self.numbers {
            if self.winning_numbers.contains(number) {
                if points == 0 {
                    points = 1;
                } else {
                    points <<= 1;
                }
            }
        }

        points
    }
}

pub fn parse_input(input: Vec<String>) -> Vec<ScratchCard> {
    let mut cards: Vec<ScratchCard> = Vec::new();

    for line in input {
        let card = ScratchCard::new(parse_line(&line));
        cards.push(card);
    }

    cards
}

pub fn total_winnings(cards: Vec<ScratchCard>) -> u32 {
    let mut total = 0;

    for card in cards {
        total += card.point_total();
    }

    total
}

fn parse_line(line: &str) -> (u32, Vec<u32>, Vec<u32>) {
    let split: Vec<_> = line.split(": ").collect();
    let card_number = split
        .first()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let numbers: Vec<u32> = split
        .last()
        .unwrap()
        .split(" | ")
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()
        .iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    let winning_numbers: Vec<u32> = split
        .last()
        .unwrap()
        .split(" | ")
        .collect::<Vec<_>>()
        .last()
        .unwrap()
        .split(' ')
        .collect::<Vec<_>>()
        .iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .map(|num| num.parse::<u32>().unwrap())
        .collect();

    (card_number, numbers, winning_numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (card_number, numbers, winning_numbers) = parse_line(line);
        assert_eq!(card_number, 1);
        assert_eq!(numbers, vec![41, 48, 83, 86, 17]);
        assert_eq!(winning_numbers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn test_point_total() {
        let card = ScratchCard::new((
            1,
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
        ));
        assert_eq!(card.point_total(), 8);
    }
}
