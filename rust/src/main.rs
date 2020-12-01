use std::env;
mod day1;
mod utils;

fn main() {
  let day = env::args().nth(1).unwrap_or("all".to_string());

  match day.as_str() {
    "1" => day1::solve(),
    "all" => day1::solve(),
    _ => println!("Nothing for this day"),
  }
}
