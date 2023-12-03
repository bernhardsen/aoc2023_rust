use std::fs::read_to_string;

pub(crate) fn cube_conundrum() {
    println!("=== Day 2: Cube Conundrum ===");
    let input = read_to_string("input/day2.txt").unwrap();
    let games = parse_games(&input);

    println!("-- Part 1: --");
    let possible_games: Vec<&Game> = games.iter().filter_map(|game| {
        for round in game.rounds.clone().into_iter() {
            if round.red > 12 || round.green > 13 || round.blue > 14 {
                return None
            }
        }
        return Some(game);
    }).collect();

    let sum: i32 = possible_games.iter().map(|&game| game.id ).sum();

    println!("{} of the games are possible.", possible_games.len());
    println!("The sum of the ids is {sum}.");

    println!("\n-- Part 2: --");
    let powers: i32 = games.iter().map(|game| {
        let red = game.rounds.iter().map(|round| round.red).max().unwrap();
        let green = game.rounds.iter().map(|round| round.green).max().unwrap();
        let blue = game.rounds.iter().map(|round| round.blue).max().unwrap();

        return red * green * blue;
    }).sum();

    println!("The power of the cubes is {} in total.", powers);
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines().map(|line| {
        let parts = line.split(":").collect::<Vec<&str>>();
        let id = parts[0].split(" ").collect::<Vec<&str>>()[1].parse().unwrap();

        let rounds = parts[1].split(";").map(|round_input| {
            let mut red = 0;
            let mut blue = 0;
            let mut green = 0;
            round_input.split(",").for_each(|cubes| {
                let cube_data = cubes.trim().split(" ").collect::<Vec<&str>>();
                let count = cube_data[0].parse().unwrap();
                match cube_data[1] {
                    "red" => { red = count },
                    "green" => { green = count },
                    "blue" => { blue = count },
                    _ => panic!("Unknown cube color: {}", cube_data[1]),
                }
            });
            Round { red, green, blue }
        }).collect::<Vec<Round>>();

        Game::new(id, rounds)
    }).collect()
}

#[derive(Clone)]
struct Game {
    id: i32,
    rounds: Vec<Round>
}

impl Game {
    fn new(id: i32, rounds: Vec<Round>) -> Game {
        Game { id, rounds }
    }
}

#[derive(Clone)]
struct Round {
    green: i32,
    red: i32,
    blue: i32,
}

#[cfg(test)]
mod tests {
    use crate::day2::parse_games;

    #[test]
    fn test_parse_input() {
        let test_data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = parse_games(&test_data);
        assert_eq!(5, games.len());
        assert_eq!(1, games[0].id);
        assert_eq!(2, games[1].id);
        assert_eq!(13, games[2].rounds[1].green);
    }
}
