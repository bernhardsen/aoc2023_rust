use std::collections::HashMap;
use std::fs::read_to_string;

pub(crate) fn lava_floor() {
    println!("=== Day 16: The Floor Will Be Lava ===");
    let data = read_to_string("input/day16.txt").unwrap();
    let floor = parse_input(&data);
    let width = floor[0].len() as i32;
    let height = floor.len() as i32;
    println!("-- Part 1: --");
    let visited = ray_trace(&floor, Beam { x: -1, y: 0, direction: Direction::Right });
    let energized_count = visited.len();
    println!("We energize {energized_count} squares when we start in the top left corner.");

    println!();
    println!("-- Part 2: --");
    let mut max_energy = energized_count;
    for x in 0..width {
        let energized = ray_trace(&floor, Beam { x, y: -1, direction: Direction::Down }).len();
        if energized > max_energy {
            max_energy = energized;
        }
        let energized = ray_trace(&floor, Beam { x, y: height, direction: Direction::Up }).len();
        if energized > max_energy {
            max_energy = energized;
        }
    }
    for y in 0..height {
        let energized = ray_trace(&floor, Beam { x: -1, y, direction: Direction::Right }).len();
        if energized > max_energy {
            max_energy = energized;
        }
        let energized = ray_trace(&floor, Beam { x: width, y, direction: Direction::Left }).len();
        if energized > max_energy {
            max_energy = energized;
        }
    }
    println!("The most amount of squars we can energize is {max_energy}.");
}

fn parse_input(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

fn ray_trace(floor: &Vec<Vec<char>>, initial_beam: Beam) -> HashMap<(i32, i32), u8> {
    let mut visited = HashMap::new();
    let mut beams = vec![initial_beam];
    let width = floor[0].len() as i32;
    let height = floor.len() as i32;
    while !beams.is_empty() {
        let beam = beams.pop().unwrap();
        let next_x = match beam.direction {
            Direction::Right => beam.x + 1,
            Direction::Left => beam.x - 1,
            Direction::Up|Direction::Down => beam.x,
        };
        let next_y = match beam.direction {
            Direction::Right|Direction::Left => beam.y,
            Direction::Up => beam.y - 1,
            Direction::Down => beam.y + 1,
        };

        let mut next_directions = vec![];
        if next_y >= 0 && next_x >= 0 && next_x < width && next_y < height {
            match floor[next_y as usize][next_x as usize] {
                '|' => {
                    if beam.x - next_x != 0 {
                        next_directions.push(Direction::Up);
                        next_directions.push(Direction::Down);
                    } else {
                        next_directions.push(beam.direction);
                    }
                },
                '-' => {
                    if beam.y - next_y != 0 {
                        next_directions.push(Direction::Right);
                        next_directions.push(Direction::Left);
                    } else {
                        next_directions.push(beam.direction);
                    }
                },
                '/' => {
                    next_directions.push(match beam.direction {
                        Direction::Right => Direction::Up,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Up => Direction::Right,
                    });
                },
                '\\' => {
                    next_directions.push(match beam.direction {
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Up => Direction::Left,
                    });
                },
                _ => next_directions.push(beam.direction),
            }
            for direction in next_directions {
                match visited.get_mut(&(next_x, next_y)) {
                    Some(val) => {
                        if *val & direction.to_u8() == 0 {
                            beams.push(Beam { x: next_x, y: next_y, direction });
                            *val += direction.to_u8();
                        }
                    },
                    None => {
                        beams.push(Beam { x: next_x, y: next_y, direction });
                        visited.insert((next_x, next_y), direction.to_u8());
                    }
                }
            }
        }
    }
    visited
}

#[derive(Copy, Clone)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn to_u8(&self) -> u8 {
        match self {
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 4,
            Direction::Up => 8,
        }
    }
}

struct Beam {
    x: i32,
    y: i32,
    direction: Direction,
}
