use std::fs::read_to_string;

pub fn get_input(day: u8) -> Vec<i32> {
    read_to_string(format!("../inputs/{}.txt", day))
        .unwrap()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}
