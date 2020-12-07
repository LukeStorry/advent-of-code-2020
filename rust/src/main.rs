use std::env;

#[macro_use]
extern crate lazy_static;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let day = env::args().nth(1).unwrap_or("all".to_string());

    match day.as_str() {
        "1" => day1::solve(),
        "2" => day2::solve(),
        "3" => day3::solve(),
        "4" => day4::solve(),
        "5" => day5::solve(),
        "6" => day6::solve(),
        "all" => {
            day1::solve();
            day2::solve();
            day3::solve();
            day4::solve();
            day5::solve();
            day6::solve();
        }
        _ => println!("Nothing for this day"),
    }
}
