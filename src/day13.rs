use std::cmp::min;
use std::fs::read_to_string;
use crate::utils::rotate_vec_of_vec;

pub(crate) fn reflections() {
    println!("=== Day 13: Point of Incidence ===");
    let data = read_to_string("input/day13.txt").unwrap().replace("\r\n", "\n");

    let patterns = parse_input(&data);
    let score = summarize_reflections(&patterns);
    println!("When we summarize the patterns the total score is {score}");

    println!();
    println!("-- Part 2: --");
    let smudge_score: i32 = patterns.iter().map(find_smudge_variation).sum();
    println!("After fixing the smudges, the total score is {smudge_score}.");
}

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input.split("\n\n").map(|part| part.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()).collect()
}

fn summarize_reflections(patterns: &Vec<Vec<Vec<char>>>) -> i32 {
    patterns.iter().map(|pattern| find_reflection(pattern, false, -1)).sum()
}

fn find_reflection(pattern: &Vec<Vec<char>>, rotated: bool, ignore: i32) -> i32 {
    let skip = if rotated { ignore } else { ignore / 100 };

    for i in 0..(pattern.len() - 1) {
        if i as i32 == skip - 1 {
            continue;
        }
        let length = min(i + 1, pattern.len() - i - 1);
        let mut is_reflection = true;
        for j in 0..length {
            // is_reflection = is_reflection && String::from_iter(&pattern[i - j]) == String::from_iter(&pattern[i + 1 + j]);
            is_reflection = is_reflection && pattern[i - j] == pattern[i + 1 + j];
        }
        if is_reflection {
            return if rotated { i + 1 } else { (i + 1) * 100 } as i32;
        }
    }

    if rotated {
        return -1
    } else {
        return find_reflection(&rotate_vec_of_vec(pattern), true, ignore)
    }
}

fn find_smudge_variation(pattern: &Vec<Vec<char>>) -> i32 {
    let mut smudge_variation = pattern.clone();
    let original_reflection = find_reflection(pattern, false, -1);
    for y in 0..pattern.len() {
        for x in 0..pattern[y].len() {
            let original_char = smudge_variation[y][x];
            smudge_variation[y][x] = match original_char {
                '#' => '.',
                '.' => '#',
                _ => panic!("Unexpected character in pattern!"),
            };
            let new_reflection = find_reflection(&smudge_variation, false, original_reflection);
            if new_reflection != -1 && new_reflection != original_reflection {
                return new_reflection;
            }
            smudge_variation[y][x] = original_char;
        }
    }
    -1
}
