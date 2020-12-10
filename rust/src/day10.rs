use std::fs::read_to_string;

fn get_input() -> Vec<i32> {
    read_to_string("../inputs/10.txt").unwrap()
        .split_whitespace()
        .flat_map(|i| i.parse())
        .collect()
}

fn get_list_of_joltages(numbers: &Vec<i32>) -> Vec<i32> {
    let mut joltages = vec!(0, *numbers.iter().max().unwrap() + 3);
    joltages.extend_from_slice(numbers);
    joltages.sort();
    joltages
}

pub fn solve() {
    let joltages = get_list_of_joltages(&get_input());
    print!("Day 9 part 1: {}\n", part_1(&joltages));
    print!("Day 9 part 2: {}\n", part_2(&joltages));
}

fn part_1(joltages: &Vec<i32>) -> usize {
    let (mut ones, mut threes) = (0, 0);
    for (a, b) in joltages.iter().zip(joltages.iter().skip(1)) {
        match b - a {
            1 => ones += 1,
            3 => threes += 1,
            _ => ()
        }
    }
    ones * threes
}


fn part_2(joltages: &Vec<i32>) -> usize {
    let len = (*joltages.iter().max().unwrap() + 1) as usize;
    let mut count_paths = vec!(0; len);
    count_paths[0] = 1;
    for &joltage in joltages {
        for possibility in joltage - 3..joltage {
            if possibility >= 0 {
                count_paths[joltage as usize] += count_paths[possibility as usize];
            }
        }
    }
    count_paths[len -1]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_first_example() {
        let nums = vec!(16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4);
        let result = part_1(&get_list_of_joltages(&nums));
        assert_eq!(result, 35);
    }

    #[test]
    fn test_part1_with_second_example() {
        let nums = vec!(28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3);
        let result = part_1(&get_list_of_joltages(&nums));
        assert_eq!(result, 220);
    }

    #[test]
    fn test_part1_with_real() {
        let nums = get_input();
        let result = part_1(&get_list_of_joltages(&nums));
        assert_eq!(result, 1856);
    }

    #[test]
    fn test_part2_with_first_example() {
        let nums = vec!(16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4);
        let result = part_2(&get_list_of_joltages(&nums));
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_with_second_example() {
        let nums = vec!(28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3);
        let result = part_2(&get_list_of_joltages(&nums));
        assert_eq!(result, 19208);
    }

    #[test]
    fn test_part2_with_real() {
        let nums = get_input();
        let result = part_2(&get_list_of_joltages(&nums));
        assert_eq!(result, 2314037239808);
    }
}
