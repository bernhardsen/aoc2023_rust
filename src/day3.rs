use std::fs::read_to_string;
use crate::day3::ObjectType::{Gear, Part};

pub(crate) fn gear_ratios() {
    println!("=== Day 3: Gear Ratios ===");
    let input = read_to_string("input/day3.txt").unwrap();
    let objects = parse_input(&input);

    println!("-- Part 1: --");
    let sum = objects.iter()
        .filter(|part| part.object_type == Part)
        .filter(|part| objects.iter().any(|o| o.overlaps(part)))
        .map(|part| part.value)
        .sum::<usize>();

    println!("The sum of the parts touching gears is {sum}.");

    println!("\n-- Part 2: --");
    let power: usize = objects.iter()
        .filter(|gear| gear.object_type == Gear)
        .map(|gear| {
            let touching =
                objects
                    .iter()
                    .filter(|part| part.overlaps(gear))
                    .collect::<Vec<&SchematicObject>>();
            if touching.len() == 2 {
                return touching.first().unwrap().value * touching.last().unwrap().value;
            }
            0
        })
        .sum();

    println!("The sum of power of the gears is {power}.");
}

fn parse_input(input: &str) -> Vec<SchematicObject> {
    let mut objects = vec![];

    for (y, line) in input.lines().enumerate() {
        let mut x = 0;
        let width = line.len();
        let characters = line.bytes().map(|b| b as char).collect::<Vec<char>>();
        while x < width {
            match characters[x] {
                '.' => (),
                '0'..='9' => {
                    let mut length = 1;
                    while x + length < width
                        && characters[x + length] >= '0'
                        && characters[x + length] <= '9' {
                        length += 1;
                    }
                    let val = &line[x..(x + length)].parse().unwrap();
                    objects.push(SchematicObject {
                        object_type: Part,
                        position: Position { x, y },
                        size: Size {width: length, height: 1},
                        value: *val,
                    });
                    x += length - 1;
                },
                _ => {
                    objects.push(SchematicObject {
                        object_type: Gear,
                        position: Position { x: x - 1, y: y - 1 },
                        size: Size { width: 3, height: 3 },
                        value: 0,
                    });
                },
            };
            x += 1;
        }
    };

    objects
}

#[derive(PartialEq)]
enum ObjectType {
    Gear,
    Part,
}

struct Position {
    x: usize,
    y: usize,
}

struct Size {
    width: usize,
    height: usize,
}

struct SchematicObject {
    object_type: ObjectType,
    position: Position,
    size: Size,
    value: usize,
}

impl SchematicObject {
    fn overlaps(&self, other: &SchematicObject) -> bool {
        if self.object_type == other.object_type {
            return false;
        }

        return self.position.x < (other.position.x + other.size.width) &&
            (self.position.x + self.size.width) > other.position.x &&
            self.position.y < (other.position.y + other.size.height) &&
            (self.position.y + self.size.height) > other.position.y;
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::{parse_input, Position, SchematicObject, Size};
    use crate::day3::ObjectType::{Gear, Part};

    #[test]
    fn test_parse_input() {
        let test_data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let objects = parse_input(test_data);

        assert_eq!(16, objects.len());
    }

    #[test]
    fn test_overlap() {
        let part1 = SchematicObject {
            object_type: Part,
            position: Position { x: 0, y: 0 },
            size: Size { width: 3, height: 1 },
            value: 123,
        };

        let part2 = SchematicObject {
            object_type: Part,
            position: Position { x: 5, y: 0 },
            size: Size { width: 3, height: 1 },
            value: 456,
        };

        let gear = SchematicObject {
            object_type: Gear,
            position: Position { x: 2, y: 0 },
            size: Size { width: 3, height: 3 },
            value: 0,
        };

        assert_eq!(true, gear.overlaps(&part1));
        assert_eq!(false, gear.overlaps(&part2));
    }
}
