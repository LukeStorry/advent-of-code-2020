use std::fs::read_to_string;

fn get_input() -> Vec<u64> {
    read_to_string("../inputs/10.txt").unwrap()
        .split_whitespace()
        .flat_map(|i| i.parse())
        .collect()
}

fn get_list_of_joltages(numbers: &Vec<u64>) -> Vec<u64> {
    let mut chain = vec!(0);
    chain.extend_from_slice(numbers.as_slice());
    chain.push(*numbers.iter().max().unwrap() + 3);
    chain.sort();
    chain
}

pub fn solve() {
    let joltages = get_list_of_joltages(&get_input());
    print!("Day 9 part 1: {}\n", part_1(&joltages));
    print!("Day 9 part 2: {}\n", part_2(&joltages));
}

fn part_1(joltages: &Vec<u64>) -> usize {
    let count = joltages
        .iter()
        .zip(joltages.iter().skip(1))
        .map(|(a, b)| b - a)
        .fold((0, 0), |acc, diff|
            match diff {
                1 => (acc.0 + 1, acc.1),
                3 => (acc.0, acc.1 + 1),
                _ => (acc.0, acc.1)
            });
    count.0 * count.1
}


fn part_2(numbers: &Vec<u64>) -> usize {
    0
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
        let result = part_2(&nums);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_with_second_example() {
        let nums = vec!(28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3);
        let result = part_2(&nums);
        assert_eq!(result, 19208);
    }

    #[test]
    fn test_part2_with_real() {
        let nums = get_input();
        let result = part_2(&nums);
        assert_eq!(result, 2314037239808);
    }
}
