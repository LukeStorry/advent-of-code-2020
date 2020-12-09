use std::fs::read_to_string;

fn get_input() -> Vec<i32> {
    read_to_string("../inputs/9.txt").unwrap()
        .split_whitespace()
        .flat_map(|i| i.parse())
        .collect()
}

pub fn solve() {
    let numbers = get_input();
    print!("Day 9 part 1: {}\n", part_1(&numbers, 25));
    print!("Day 9 part 2: {}\n", part_2(&numbers, 25));
}

fn part_1(numbers: &Vec<i32>, preamble_length: usize) -> i32 {
    numbers
        .iter()
        .enumerate()
        .skip(preamble_length)
        .find_map(|(num_index, &num)| {
            for left in (num_index - preamble_length)..num_index {
                for right in (left + 1)..num_index {
                    if numbers[left] + numbers[right] == num {
                        return None;
                    }
                }
            }
            Some(num)
        })
        .unwrap()
}

fn part_2(numbers: &Vec<i32>, preamble_length: usize) -> i32 {
    let invalid_num = part_1(numbers, preamble_length);
    for left in 0..numbers.len() - 1 {
        for right in (left+1)..numbers.len() {
            let range = &numbers[left..right];
            let sum = range.iter().sum();
            if invalid_num < sum { break; };
            if invalid_num == sum { return range.iter().min().unwrap() + range.iter().max().unwrap(); }
        }
    }
    unreachable!("No Range found")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_with_example() {
        let nums = vec!(35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576);
        let result = part_1(&nums, 5);
        assert_eq!(result, 127);
    }

    #[test]
    fn test_part1_with_real() {
        let nums = get_input();
        let result = part_1(&nums, 25);
        assert_eq!(result, 15690279);
    }

    #[test]
    fn test_part2_with_example() {
        let nums = vec!(35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576);
        let result = part_2(&nums, 5);
        assert_eq!(result, 62);
    }

    #[test]
    fn test_part2_with_real() {
        let nums = get_input();
        let result = part_2(&nums, 25);
        assert_eq!(result, 2174232);
    }
}
