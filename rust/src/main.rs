use std::env;
mod utils;
mod day1;

fn main() {
  let day = env::args().nth(1).unwrap_or("all".to_string());

  match day.as_str() {
    "1" => day1::part1(),
    "all" => day1::part1(),
    _ => println!("Nothing for this day"),
  }
}
