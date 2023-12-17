use std::cmp::{max, min};
use std::fs::read_to_string;
use crate::utils::rotate_vec_of_vec;

pub(crate) fn cosmic() {
    println!("=== Day 11: Cosmic Expansion ===");
    let data = read_to_string("input/day11.txt").unwrap();
    let universe = parse_data(data);
    let exp_rows = find_expansions(&universe);
    let rotated = rotate_vec_of_vec(&universe.clone());
    let exp_cols = find_expansions(&rotated);
    let galaxies = find_galaxies(&universe);
    println!("-- Part 1: --");
    let total_distance = calculate_total_distance(&galaxies, &exp_cols, &exp_rows, 1);
    println!("The total distance between the galaxies is {total_distance}.");
    println!();
    println!("-- Part 2: --");
    let older_distance = calculate_total_distance(&galaxies, &exp_cols, &exp_rows, 999_999);
    println!("If the universe is older, then the total distance will be {older_distance}.");
}

fn find_expansions(universe: &Vec<Vec<char>>) -> Vec<usize> {
    let mut expansions = vec![];
    for i in 0..universe.len() {
        if universe[i].iter().all(|&c| c == '.') {
            expansions.push(i)
        }
    }
    expansions
}

fn parse_data(data: String) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

fn calculate_total_distance(galaxies: &Vec<Galaxy>, exp_cols: &Vec<usize>, exp_rows: &Vec<usize>, expansion_size: u64) -> u64 {
    let mut total_distance = 0u64;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let mut crossed = exp_cols.iter().filter(|&&exp|
                min(galaxies[i].x, galaxies[j].x) < exp &&
                    exp < max(galaxies[i].x, galaxies[j].x)
            ).count() as u64;
            crossed += exp_rows.iter().filter(|&&exp|
                min(galaxies[i].y, galaxies[j].y) < exp &&
                    exp < max(galaxies[i].y, galaxies[j].y)
            ).count() as u64;
            total_distance += galaxies[i].distance_to(&galaxies[j]) as u64 + crossed * expansion_size;
        }
    }
    total_distance
}

fn find_galaxies(universe: &Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut galaxies = vec![];
    for y in 0..universe.len() {
        for x in 0..universe[y].len() {
            if universe[y][x] == '#' {
                galaxies.push(Galaxy { x, y });
            }
        }
    }
    galaxies
}

struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn distance_to(&self, other: &Galaxy) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
