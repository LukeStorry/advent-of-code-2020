use std::fs::read_to_string;

pub fn get_input(day: u8) -> Vec<i32> {
    read_to_string(format!("../inputs/{}.txt", day))
        .unwrap()
        .trim()
        .split('\n')
        .map(|int| int.trim().parse().unwrap())
        .collect()
}
