mod day1;
mod day2;
mod day3;

fn main() {
    println!("Advent of Code 2023!");
    match std::env::args().nth(1).expect("Missing day argument").as_str() {
        "1" => day1::trebuchet(),
        "2" => day2::cube_conundrum(),
        "3" => day3::gear_ratios(),
        _ => println!("Day not implemented yet!"),
    };
}
