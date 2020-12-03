use std::env;
mod day1;
mod day2;
mod day3;
mod utils;

fn main() {
  let day = env::args().nth(1).unwrap_or("all".to_string());

  match day.as_str() {
    "1" => day1::solve(),
    "2" => day2::solve(),
    "3" => day3::solve(),
    "all" => {
      day1::solve();
      day2::solve();
      day3::solve();
    }
    _ => println!("Nothing for this day"),
  }
}
