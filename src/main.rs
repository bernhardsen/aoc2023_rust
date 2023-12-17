mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day16;
mod day13;
mod utils;

fn main() {
    println!("Advent of Code 2023!");
    match std::env::args().nth(1).expect("Missing day argument").as_str() {
        "1" => day1::trebuchet(),
        "2" => day2::cube_conundrum(),
        "3" => day3::gear_ratios(),
        "4" => day4::scratch_cards(),
        "5" => day5::seed_locator(),
        "6" => day6::wait_for_it(),
        "7" => day7::camel_cards(),
        "8" => day8::wasteland(),
        "9" => day9::mirage(),
        "10" => day10::pipe_layer(),
        "11" => day11::cosmic(),
        "13" => day13::reflections(),
        "16" => day16::lava_floor(),
        _ => println!("Day not implemented yet!"),
    };
}
