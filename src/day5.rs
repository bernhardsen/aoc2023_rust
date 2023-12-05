use std::fs::read_to_string;

pub(crate) fn seed_locator() {
    println!("=== Day 5: If You Give A Seed A Fertilizer ===");
    let input = read_to_string("input/day5.txt").unwrap();

    let binding = input.replace("\r\n", "\n");
    let mut data_parts = binding.split("\n\n").collect::<Vec<&str>>();

    let seeds = parse_seeds(data_parts[0]);
    data_parts.remove(0);
    let mappers = parse_mappers(&data_parts);

    println!("-- Part 1: --");
    let lowest_part1 = seeds.iter().map(|&start|
        mappers.iter().fold(start, |location, mappers| look_up_location(mappers, location))
    ).min().unwrap();
    println!("The lowest possible location is {lowest_part1}.");

    println!("\n-- Part 2: --");
    let lowest_part2 = seeds
        .chunks(2)
        .map(|parts| vec![LongRange{ start: parts[0], end: parts[0] + parts[1] }])
        .map(|all_ranges| all_ranges.iter().map(|start| {
                mappers.iter().fold(vec![start.clone()], |ranges, mapper|
                    ranges.iter()
                        .map(|r| range_to_range(mapper, r))
                        .flatten()
                        .collect()
                ).iter().map(|r| r.start).min().unwrap()
            }).min().unwrap()
        ).min().unwrap();
    println!("Of all the ranges, {lowest_part2} is the lowest location.");
}

fn look_up_location(mappers: &Vec<LocationMapper>, location: i64) -> i64 {
    match mappers.iter().find(|&m| location >= m.source_start && location < m.source_start + m.length) {
        Some(mapper) => location + mapper.translation,
        None => location,
    }
}

fn range_to_range(mappers: &Vec<LocationMapper>, org_range: &LongRange) -> Vec<LongRange> {
    let mut range = org_range.clone();
    let mut output: Vec<LongRange> = vec![];
    loop {
        match mappers.iter().find(|m| range.start >= m.source_start && range.start < m.source_start + m.length) {
            Some(mapper) => {
                if range.start - mapper.source_start + range.end - range.start <= mapper.length {
                    output.push(LongRange {
                        start: range.start + mapper.translation,
                        end: range.end + mapper.translation
                    });
                    return output;
                }
                output.push(LongRange {
                    start: range.start + mapper.translation,
                    end: mapper.source_start + mapper.length + mapper.translation
                });
                range.start = mapper.source_start + mapper.length;
            },
            None => {
                let next = mappers.iter().map(|m| m.source_start).filter(|&n| n > range.start).min().unwrap_or(-1);
                if next == -1 {
                    output.push(range.clone());
                    return output;
                }
                output.push(LongRange { start: range.start, end: next - 1});
                range.start = next;
            }
        }
    }
}

fn parse_seeds(data: &str) -> Vec<i64> {
    data.split(": ")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse_mappers(data: &Vec<&str>) -> Vec<Vec<LocationMapper>> {
    return data.iter().map(|&map_data| {
        let mut lines = map_data.lines().collect::<Vec<&str>>();
        lines.remove(0);
        return lines.iter().map(|line| {
            let values = line.split(" ").map(|v| v.parse::<i64>().unwrap()).collect::<Vec<i64>>();
            return LocationMapper {
                source_start: values[1],
                translation: values[0] as i64 - values[1],
                length: values[2],
            };
        }).collect();
    }).collect::<Vec<Vec<LocationMapper>>>()
}

struct LocationMapper {
    source_start: i64,
    translation: i64,
    length: i64,
}

#[derive(Clone)]
struct LongRange {
    start: i64,
    end: i64,
}
