use std::fs::read_to_string;

pub fn get_input_ints(day: u8) -> Vec<i32> {
    read_to_string(format!("../inputs/{}.txt", day))
        .unwrap()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

pub fn get_input_chars(day: u8) -> Vec<String> {
    read_to_string(format!("../inputs/{}.txt", day))
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect()
}

pub fn get_input_raw(day: u8) -> String {
    read_to_string(format!("../inputs/{}.txt", day)).unwrap()
}
