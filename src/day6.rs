use std::cmp::min;
use std::fs::read_to_string;

pub(crate) fn wait_for_it() {
    println!("=== Day 6: Wait For It ===");
    let input = read_to_string("input/day6.txt").unwrap();

    println!("-- Part 1: --");
    let sum = calculate_race_leniency(parse_input_multiple(&input));
    println!("When we multiply all together we get {sum} ways.");

    println!("\n-- Part 2: --");
    let sum = calculate_race_leniency(parse_input_single(&input));
    println!("If its just one race, we get {sum} ways.");
}

fn parse_input_multiple(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];

    let lines = input.lines().collect::<Vec<&str>>();
    let num_races = (lines[0].len() - 8) / 7;
    for i in 0..num_races {
        let s_start = 9 + i * 7;
        races.push(Race {
            time: lines[0][s_start..min(s_start + 7, lines[0].len())].trim().parse().unwrap(),
            distance: lines[1][s_start..min(s_start + 7, lines[1].len())].trim().parse().unwrap(),
        });
    }

    races
}

fn parse_input_single(input: &str) -> Vec<Race> {
    let lines = input.lines().collect::<Vec<&str>>();
    vec![Race {
        time: lines[0][9..].replace(" ", "").parse().unwrap(),
        distance: lines[1][9..].replace(" ", "").parse().unwrap(),
    }]
}

fn calculate_race_leniency(races: Vec<Race>) -> u64 {
    let mut sum = 1;
    races.iter().for_each(|race| {
        let short = calc_dist_short(race.time as f64, (race.distance + 1) as f64).ceil() as u64;
        let long = race.time - short;
        let ways = long - short + 1;
        sum *= ways;
    });
    sum
}

fn calc_dist_short(time: f64, distance: f64) -> f64 {
    return (time - (time * time - (distance * 4.0)).sqrt()) / 2.0;
}

struct Race {
    time: u64,
    distance: u64,
}
