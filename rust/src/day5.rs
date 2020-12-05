use std::fs::read_to_string;

fn seat_id(input: &str) -> i32 {
    let (mut row_lower, mut row_upper, mut col_lower, mut col_upper) = (0, 127, 0, 7);
    for character in input.chars() {
        match character {
            'F' => row_upper = ((row_upper - row_lower) as f32 / 2.).floor() as i32 + row_lower,
            'B' => row_lower = ((row_upper - row_lower) as f32 / 2.).ceil() as i32 + row_lower,
            'L' => col_upper = ((col_upper - col_lower) as f32 / 2.).floor() as i32 + col_lower,
            'R' => col_lower = ((col_upper - col_lower) as f32 / 2.).ceil() as i32 + col_lower,
            un => panic!(format!("Unknown char in input: {}", un))
        }
    }
    row_lower * 8 + col_lower
}

fn get_seats() -> Vec<i32> {
    read_to_string("../inputs/5.txt")
        .unwrap()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|s| seat_id(s))
        .collect()
}

pub fn solve() {
    let seats = get_seats();
    print!("Day 5 part 1: {}\n", part_1(&seats));
    print!("Day 5 part 2: {}\n", part_2(&seats));
}

fn part_1(seats: &Vec<i32>) -> i32 {
    *seats.iter().max().unwrap()
}

fn part_2(seats: &Vec<i32>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn test_part1() {
        let seats = get_seats();
        assert_eq!(part_1(&seats), 894);
    }


    #[test]
    fn test_part2() {
        let seats = get_seats();
        assert_eq!(part_2(&seats), 579);
    }
}
