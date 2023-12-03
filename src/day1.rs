use std::fs::read_to_string;

pub(crate) fn trebuchet() {
    println!("=== Day 1: Trebuchet?! ===");
    let input = read_to_string("input/day1.txt").expect("Input for day 1 not found");

    println!("-- Part 1: --");
    let sum: i32 = input.lines().map(|l|
        find_first_digit(l) + &*find_first_digit(l.chars().rev().collect::<String>().as_str())
    ).map(|num| num.parse::<i32>().unwrap()).sum();
    println!("The sum of all calibration values is {sum}.");

    println!("\n-- Part 2: --");
    let sum2: i32 = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e").lines().map(|l|
        find_first_digit(l) + &*find_first_digit(l.chars().rev().collect::<String>().as_str())
    ).map(|num| num.parse::<i32>().unwrap()).sum();
    println!("When we include numbers written as words, the sum is {sum2}!");
    ()
}

fn find_first_digit(s: &str) -> String {
    for c in s.chars() {
        if c.is_digit(10) {
            return c.to_string();
        }
    };
    String::from("")
}
