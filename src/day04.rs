use std::collections::{HashMap, HashSet};
use std::fmt;

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    owned_numbers: Vec<u32>,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "id: {}", self.id)
    }
}

pub fn main() {
    let input = include_str!("../input/day04.txt");
    println!("PART 1: {}", part1(input));
    println!("PART 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let cards = input.trim_end().split('\n').map(parse_card);
    cards
        .map(|card| {
            let count = count_winning_numbers(&card);
            if count == 0 {
                0
            } else {
                2_usize.pow((count - 1).try_into().unwrap())
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let cards = input
        .trim_end()
        .split('\n')
        .map(parse_card)
        .map(|card| (card.id, card))
        .collect::<HashMap<_, _>>();

    fn all_cards(
        mut cards_frequencies: HashMap<u32, u32>,
        current_card: u32,
        map_cards: &HashMap<u32, Card>,
    ) -> HashMap<u32, u32> {
        if map_cards.get(&current_card).is_none() {
            cards_frequencies
        } else {
            let card = map_cards.get(&current_card).unwrap();
            let count_winning_numbers = count_winning_numbers(card);
            if count_winning_numbers > 0 {
                let count: u32 = count_winning_numbers.try_into().unwrap();
                let new_cards_range = (card.id + 1)..=(card.id + count);
                let &&current_card_frequence = &cards_frequencies.get(&current_card).unwrap();
                for id in new_cards_range {
                    *cards_frequencies.get_mut(&id).unwrap() += current_card_frequence;
                }
                all_cards(cards_frequencies, current_card + 1, map_cards)
            } else {
                all_cards(cards_frequencies, current_card + 1, map_cards)
            }
        }
    }

    let frequencies = cards
        .keys()
        .map(|&id| (id, 1))
        .collect::<HashMap<u32, u32>>();
    all_cards(frequencies, 1, &cards).values().sum()
}

fn parse_card(raw_card: &str) -> Card {
    let (header, card_numbers) = raw_card.split_once(": ").unwrap();

    let (raw_winning_numbers, raw_owned_numbers) = card_numbers.split_once('|').unwrap();

    let id = header.replace("Card", "").trim().parse().unwrap();
    Card {
        id,
        winning_numbers: raw_winning_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
        owned_numbers: raw_owned_numbers
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect(),
    }
}

fn count_winning_numbers(card: &Card) -> usize {
    let w_set: HashSet<u32> = HashSet::from_iter(card.winning_numbers.clone());
    let o_set: HashSet<u32> = HashSet::from_iter(card.owned_numbers.clone());
    w_set.intersection(&o_set).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_inputs_part_1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn sample_inputs_part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input), 30);
    }

    #[test]
    fn solutions() {
        let input = include_str!("../input/day04.txt");
        assert_eq!(part1(input), 26914);
        assert_eq!(part2(input), 13080971);
    }
}
