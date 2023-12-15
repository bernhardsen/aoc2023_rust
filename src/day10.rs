use std::fs::read_to_string;
use crate::day10::PipeShape::{Horizontal, NorthEast, NorthWest, SouthEast, SouthWest, Vertical};

pub(crate) fn pipe_layer() {
    println!("=== Day 10: Pipe Maze ===");
    let input = read_to_string("input/day10.txt").unwrap();
    let (pipe_loop, width, height) = parse_input(&input);

    println!("-- Part 1: --");
    let max_distance = pipe_loop.len() / 2;
    println!("Max distance from anywhere in the loop is {max_distance} steps.");
    println!();
    println!("-- Part 2: --");
    let count = calculate_internal_squares(&pipe_loop, width, height);
    println!("There are {count} squares inside the pipe loop.");
}

fn parse_input(input: &str) -> (Vec<Pipe>, usize, usize) {
    let all_squares = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (start_x, start_y) = find_start_pos(&all_squares);
    let (mut x, mut y) = (start_x, start_y);
    let width = all_squares.len() as usize;
    let height = all_squares[0].len() as usize;

    let mut pipe_loop = vec![];
    let mut last_x = 0;
    let mut last_y = 0;
    while pipe_loop.len() == 0 || x != start_x || y != start_y {
        if pipe_loop.len() != 0 {
            pipe_loop.push(Pipe { x, y, shape: PipeShape::from_char(all_squares[y][x]) });
        }
        match all_squares[y][x] {
            '|' =>  y = if last_y < y { y + 1 } else { y - 1 } ,
            '-' =>  x = if last_x < x { x + 1 } else { x - 1 } ,
            'L' => if last_y < y { x += 1 } else { y -= 1 },
            'J' => if last_y < y { x -= 1 } else { y -= 1 },
            '7' => if last_y > y { x -= 1 } else { y += 1 },
            'F' => if last_y > y { x += 1 } else { y += 1 },
            'S' => {
                let mut connections = match all_squares[y - 1][x] {
                    '|'|'7'|'F' => 1,
                    _ => 0,
                };
                connections += match all_squares[y][x + 1] {
                    '-'|'7'|'J' => 2,
                    _ => 0,
                };
                connections += match all_squares[y + 1][x] {
                    '|'|'J'|'L' => 4,
                    _ => 0,
                };
                connections += match all_squares[y][x - 1] {
                    '-'|'L'|'F' => 8,
                    _ => 0,
                };
                // 3, 5, 9... 10

                match connections {
                    3 => {
                        pipe_loop.push(Pipe { x, y, shape: PipeShape::from_char('L')});
                        x += 1;
                    },
                    5 => {
                        pipe_loop.push(Pipe { x, y, shape: PipeShape::from_char('|')});
                        y += 1;
                    },
                    6 => {
                        pipe_loop.push(Pipe { x, y, shape: PipeShape::from_char('F')});
                        y += 1;
                    },
                    9 => {
                        pipe_loop.push(Pipe { x, y, shape: PipeShape::from_char('J')});
                        x -= 1;
                    },
                    10 => {
                        pipe_loop.push(Pipe { x, y, shape: PipeShape::from_char('-')});
                        x += 1;
                    },
                    12 => {
                        pipe_loop.push(Pipe { x, y, shape: PipeShape::from_char('7')});
                        y += 1;
                    }
                    _ => panic!("Unable to determine shape of S pipe.")
                }
            },
            _ => (),
        }
        last_x = pipe_loop.last().unwrap().x;
        last_y = pipe_loop.last().unwrap().y;
    }

    (pipe_loop, width, height)
}

fn find_start_pos(squares: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..squares.len() {
        for x in 0..squares[y].len() {
            if squares[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn calculate_internal_squares(pipe_loop: &Vec<Pipe>, width: usize, height: usize) -> usize {
    let mut internal_squares = 0;
    for y in 0..height {
        let mut is_inside = false;
        let mut on_pipe = false;
        let mut last_corner = Horizontal;
        for x in 0..width {
            match pipe_loop.iter().find(|pipe| pipe.x == x && pipe.y == y) {
                Some(pipe) => {
                    if on_pipe {
                        if pipe.shape == NorthWest {
                            if last_corner == SouthEast {
                                is_inside = !is_inside;
                            }
                            on_pipe = false;
                        } else if pipe.shape == SouthWest {
                            if last_corner == NorthEast {
                                is_inside = !is_inside;
                            }
                            on_pipe = false;
                        }
                    } else if pipe.shape == Vertical {
                        is_inside = !is_inside;
                    } else {
                        on_pipe = true;
                        last_corner = pipe.shape.clone();
                    }
                },
                None => if is_inside { internal_squares += 1 },
            }
        }
    }
    internal_squares
}

#[derive(Clone, PartialEq)]
enum PipeShape {
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl PipeShape {
    fn from_char(chr: char) -> PipeShape {
        match chr {
            '|' => Vertical,
            '-' => Horizontal,
            'J' => NorthWest,
            'L' => NorthEast,
            '7' => SouthWest,
            'F' => SouthEast,
            _ => panic!("Unknown char '{chr}'"),
        }
    }
}

struct Pipe {
    x: usize,
    y: usize,
    shape: PipeShape,
}
