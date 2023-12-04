use std::fs::read_to_string;

pub(crate) fn scratch_cards() {
    println!("=== Day 4: Scratchcards ===");
    let input = read_to_string("input/day4.txt").unwrap();
    let cards = parse_cards(&input);

    let points: usize = cards.iter().map(|&winners| card_score(winners)).sum();
    println!("-- Part 1: --");
    println!("The scratch cards are worth {points} points in total.");

    let mut cards_in_hand = vec![1; cards.len()];
    for i in 0..cards_in_hand.len() {
        for i2 in 0..*cards.get(i).unwrap() {
            if i + i2 + 1 < cards.len() {
                cards_in_hand[i + i2 + 1] += 1 * cards_in_hand.get(i).unwrap();
            }
        }
    }

    let total_cards: usize = cards_in_hand.iter().sum();
    println!("\n-- Part 2: --");
    println!("We get {total_cards} number of cards in total.");
}

fn parse_cards(input: &str) -> Vec<usize> {
    input.lines().map(|sc| {
        let parts = sc.split(": ").collect::<Vec<&str>>();
        let numbers = parts[1].split(" | ").collect::<Vec<&str>>();
        let winning_numbers = numbers[0]
            .split(" ")
            .filter(|&s| s != "")
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i32>>();
        numbers[1]
            .split(" ")
            .filter(|&s| s != "")
            .map(|num| num.parse().unwrap())
            .filter(|num| winning_numbers.contains(num))
            .count()
    }).collect()
}

fn card_score(n: usize) -> usize {
    match n {
        0 => 0,
        _ => 1 << (n - 1)
    }
}
