use std::fs::read_to_string;
use std::collections::HashSet;

fn seat_id(input: &str) -> u32 {
    let (mut row_lower, mut row_upper, mut col_lower, mut col_upper): (f32, f32, f32, f32) = (0., 127., 0., 7.);
    for character in input.chars() {
        match character {
            'F' => row_upper = ((row_upper - row_lower) / 2.).floor() + row_lower,
            'B' => row_lower = ((row_upper - row_lower) / 2.).ceil() + row_lower,
            'L' => col_upper = ((col_upper - col_lower) / 2.).floor() + col_lower,
            'R' => col_lower = ((col_upper - col_lower) / 2.).ceil() + col_lower,
            un => panic!(format!("Unknown char in input: {}", un))
        }
    }
    (row_lower * 8. + col_lower) as u32
}

fn get_seats() -> HashSet<u32> {
    read_to_string("../inputs/5.txt")
        .unwrap()
        .lines()
        .map(|s| seat_id(s))
        .collect()
}

pub fn solve() {
    let seats = get_seats();
    print!("Day 5 part 1: {}\n", part_1(&seats));
    print!("Day 5 part 2: {}\n", part_2(&seats));
}

fn part_1(seats: &HashSet<u32>) -> u32 {
    *seats.iter().max().unwrap()
}

fn part_2(seats: &HashSet<u32>) -> u32 {
    let min = *seats.iter().min().unwrap();
    let max = *seats.iter().max().unwrap();

    let all_seats: HashSet<u32> = (min..max).collect();

    *all_seats.difference(seats).next().unwrap()
}

#[cfg(test)]
mod tests {
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
