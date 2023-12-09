use std::cell::RefCell;
use std::fs::read_to_string;
use std::rc::Rc;
use crate::day8::Action::{Left, Right};

pub(crate) fn wasteland() {
    println!("=== Day 8: Haunted Wasteland ===");
    let data = read_to_string("input/day8.txt").unwrap().replace("\r\n", "\n");
    let (actions, network) = parse_input(&data);

    println!("-- Part 1: --");
    let (steps, _) = count_steps(&network, &actions, &"AAA", &"ZZZ");
    println!("It takes {steps} steps to get to ZZZ from AAA.");
    println!();
    println!("-- Part 2: --");
    let steps2 = sync_the_candidates(&network, &actions);
    println!("All the candidates starting on nodes ending in A,");
    println!("are on nodes ending in Z at the same time, after {steps2} steps.");
}

fn sync_the_candidates(network: &Vec<Rc<RefCell<NetworkNode>>>, actions: &Vec<Action>) -> u64 {
    let candidates: Vec<&Rc<RefCell<NetworkNode>>> =
        network.iter().filter(|&node| node.borrow().identifier.ends_with("A")).collect();

    let first_time: Vec<(u64, String)> = candidates.iter().map(|&c|
        count_steps(network, actions, &c.borrow().identifier, "Z")
    ).collect();

    let next_time: Vec<(u64, String)> = first_time.iter().map(|(_, target)|
        count_steps(network, actions, target, target)
    ).collect();

    let mut position = first_time[0].0;
    let step_size = next_time[0].0;
    loop {
        position += step_size;
        if !first_time.iter().enumerate().any(|(i, (start, _))| (position - start) % next_time[i].0 != 0) {
            return position
        }
    }
}

fn count_steps(network: &Vec<Rc<RefCell<NetworkNode>>>, actions: &Vec<Action>, from: &str, to: &str) -> (u64, String) {
    let mut steps = 0;
    let mut current =
        network.iter().find(|&it| it.borrow().identifier == from).unwrap().borrow().clone();
    while steps == 0 || !current.identifier.ends_with(to) {
        let next = match actions[steps % actions.len()] {
            Left => current.left.clone().unwrap(),
            Right => current.right.clone().unwrap(),
        };
        current = next.borrow().clone();
        steps += 1;
    }
    (steps as u64, String::from(current.identifier))
}

fn parse_input(input: &str) -> (Vec<Action>, Vec<Rc<RefCell<NetworkNode>>>) {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    // Parse actions:
    let actions = parts[0].chars().map(|c| match c {
        'L' => Left,
        'R' => Right,
        _ => panic!("Unexpected action: {c}")
    }).collect();

    // First pass of network nodes
    let nodes: Vec<Rc<RefCell<NetworkNode>>> = parts[1]
        .lines()
        .map(|line|
            Rc::new(RefCell::new(NetworkNode {
                identifier: String::from(&line[0..3]),
                left: None,
                right: None,
            }))
        ).collect();
    // Second pass
    parts[1].lines()
        .for_each(|line| {
            let left_node = nodes.iter().find(|&it| it.borrow().identifier == line[7..10]).unwrap();
            let right_node = nodes.iter().find(|&it| it.borrow().identifier == line[12..15]).unwrap();
            let mut current_node = nodes.iter().find(|&it| it.borrow().identifier == line[0..3]).unwrap().borrow_mut();
            current_node.left = Some(left_node.clone());
            current_node.right = Some(right_node.clone());
        });

    (actions, nodes)
}

enum Action {
    Left,
    Right,
}

#[derive(Clone)]
struct NetworkNode {
    identifier: String,
    left: Option<Rc<RefCell<NetworkNode>>>,
    right: Option<Rc<RefCell<NetworkNode>>>,
}
