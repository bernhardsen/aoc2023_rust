use std::collections::HashMap;
use std::fs::read_to_string;

pub(crate) fn camel_cards() {
    println!("=== Day 7: Camel Cards ===");
    let input = read_to_string("input/day7.txt").unwrap();

    println!("-- Part 1: --");
    let hands = parse_input(&input, false);
    let total_score = score_all_the_hands(&hands);
    println!("The total score of all the hands is {total_score}");

    println!("\n-- Part 2: --");
    let hands = parse_input(&input, true);
    let total_score = score_all_the_hands(&hands);
    println!("If we play with jokers, the total score is {total_score}");
}

fn score_all_the_hands(hands: &Vec<CamelHand>) -> u32 {
    let mut sorted_hands = hands.clone();
    sorted_hands.sort_unstable_by_key(|hand| (hand.score, hand.cards[0], hand.cards[1], hand.cards[2], hand.cards[3], hand.cards[4]));
    let mut i: u32 = 1;
    let mut total_score: u32 = 0;
    for hand in sorted_hands {
        total_score += hand.value * i;
        i += 1;
    }
    total_score
}

fn parse_input(input: &str, with_joker: bool) -> Vec<CamelHand> {
    input.lines().map(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        CamelHand::new(parts[0].chars().collect(), parts[1].parse().unwrap(), with_joker)
    }).collect::<Vec<CamelHand>>()
}

fn score_hand(cards: &Vec<u8>) -> u32 {
    let dupes = count_duplicates_sorted(cards);
    if dupes[0] == 5 {
        return 64;
    } else if dupes[0] == 4 {
        return 32;
    } else if dupes[0] == 3 && dupes[1] == 2 {
        return 16;
    } else if dupes[0] == 3 {
        return 8;
    } else if dupes[0] == 2 && dupes[1] == 2 {
        return 4;
    } else if dupes[0] == 2 {
        return 2;
    }
    1
}

fn count_duplicates_sorted(cards: &Vec<u8>) -> Vec<i32> {
    let mut value_count: HashMap<u8, i32> = HashMap::new();
    let non_jokers = cards.iter().filter(|&c| *c != 1u8).collect::<Vec<&u8>>();
    let joker_count = 5 - non_jokers.len() as i32;
    if joker_count == 5 {
        return vec![5];
    }

    for card in non_jokers {
        value_count.insert(*card, value_count.get(card).unwrap_or(&0) + 1);
    }
    let mut duplicates = value_count.values().map(|&v| v).collect::<Vec<i32>>();
    duplicates.sort();
    duplicates.reverse();
    duplicates[0] += joker_count;
    duplicates
}

#[derive(Clone)]
struct CamelHand {
    cards: Vec<u8>,
    value: u32,
    score: u32,
}

impl CamelHand {
    fn new(cards_chars: Vec<char>, value: u32, with_joker: bool) -> CamelHand {
        let cards: Vec<u8> = cards_chars.iter().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => if with_joker { 1 } else { 11 },
                'T' => 10,
                _ => *c as u8 - 48,
            }
        }).collect();
        let score = score_hand(&cards.clone());
        CamelHand {
            cards,
            value,
            score
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::{parse_input, score_all_the_hands, score_hand};

    #[test]
    fn test_score_hand() {
        assert_eq!(1, score_hand(&"27A83".chars().map(|c| c as u8).collect()));
        assert_eq!(2, score_hand(&"AATKJ".chars().map(|c| c as u8).collect()));
        assert_eq!(4, score_hand(&"Q6T6T".chars().map(|c| c as u8).collect()));
        assert_eq!(8, score_hand(&"6TT8T".chars().map(|c| c as u8).collect()));
        assert_eq!(8, score_hand(&"99K89".chars().map(|c| c as u8).collect()));
        assert_eq!(16, score_hand(&"K9K99".chars().map(|c| c as u8).collect()));
        assert_eq!(32, score_hand(&"777Q7".chars().map(|c| c as u8).collect()));
        assert_eq!(64, score_hand(&"44444".chars().map(|c| c as u8).collect()));
    }

    #[test]
    fn test_ranking_and_scoring_the_hands() {
        let test_data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let hands = parse_input(test_data, false);
        let score = score_all_the_hands(&hands);
        assert_eq!(6440, score);
    }

    #[test]
    fn test_with_jokers() {
        let test_data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let hands = parse_input(test_data, true);
        let score = score_all_the_hands(&hands);
        assert_eq!(5905, score);
    }
}
