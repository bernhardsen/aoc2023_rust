use std::fs::read_to_string;

pub(crate) fn mirage() {
    println!("=== Day 7: Mirage Maintenance ===");
    let input = read_to_string("input/day9.txt").unwrap();
    let histories = parse_input(&input);

    println!("-- Part 1: --");
    let all_the_values: i32 = histories.iter().map(|l| calculate_next_value_of_history(l, true)).sum();
    println!("The sum of all the next values is {all_the_values}.");

    println!("\n-- Part 2: --");
    let all_the_values: i32 = histories.iter().map(|l| calculate_next_value_of_history(l, false)).sum();
    println!("The sum of all previous values is {all_the_values}.");
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line|
            line.split(" ").map(|num| num.parse().unwrap()).collect::<Vec<i32>>()
        ).collect::<Vec<Vec<i32>>>()
}

fn calculate_next_value_of_history(values: &Vec<i32>, after: bool) -> i32 {
    let mut layers = vec![values.clone()];
    while !is_all_zeros(layers.last().unwrap().as_ref()) {
        layers.push(calculate_next_layer(layers.last().unwrap().as_ref()));
    }
    if after {
        calculate_after_from_layers(layers)
    } else {
        calculate_before_from_layers(layers)
    }
}

fn calculate_after_from_layers(mut layers: Vec<Vec<i32>>) -> i32 {
    for i in (1..layers.len()).rev() {
        let next_in_prev_layer = layers[i].last().unwrap() + layers[i - 1].last().unwrap();
        layers[i - 1].push(next_in_prev_layer)
    }
    *layers[0].last().unwrap()
}

fn calculate_before_from_layers(mut layers: Vec<Vec<i32>>) -> i32 {
    layers.reverse();
    return layers.iter()
        .fold(0, |num, layer| layer.first().unwrap() - num);
}

fn calculate_next_layer(values: &Vec<i32>) -> Vec<i32> {
    let mut next = vec![];
    for i in 0..(values.len() - 1) {
        next.push(values[i + 1] - values[i])
    }
    next
}

fn is_all_zeros(values: &Vec<i32>) -> bool {
    values.iter().filter(|&item| *item != 0).count() == 0
}

#[cfg(test)]
mod tests {
    use crate::day9::{calculate_next_layer, calculate_next_value_of_history};

    #[test]
    fn test_calculate_next_layer() {
        let input = vec![10, 13, 16, 21, 30, 45];
        let result = calculate_next_layer(&input);
        assert_eq!(vec![3, 3, 5, 9, 15], result);
    }

    #[test]
    fn test_calculate_next_value_of_history() {
        assert_eq!(18, calculate_next_value_of_history(&vec![0, 3, 6, 9, 12, 15], true));
        assert_eq!(28, calculate_next_value_of_history(&vec![1, 3, 6, 10, 15, 21], true));
        assert_eq!(68, calculate_next_value_of_history(&vec![10, 13, 16, 21, 30, 45], true));
    }
    #[test]
    fn test_calculate_next_value_of_history_before() {
        assert_eq!(5, calculate_next_value_of_history(&vec![10, 13, 16, 21, 30, 45], false));
    }
}

