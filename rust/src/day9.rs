use std::fs::read_to_string;
use itertools::Itertools;

fn get_input() -> Vec<i32> {
    read_to_string("../inputs/9.txt").unwrap()
        .split_whitespace()
        .flat_map(|i| i.parse())
        .collect()
}

pub fn solve() {
    let numbers = get_input();
    print!("Day 9 part 1: {}\n", part_1(&numbers, 25));
    // print!("Day 9 part 2: {}\n", part_2(&numbers));
}

fn part_1(numbers: &Vec<i32>, preamble_length: usize) -> i32 {
    *numbers
        .iter()
        .enumerate()
        .skip(preamble_length)
        .find(|(i, &n)|
            !numbers.iter()
                .skip(i - preamble_length)
                .take(preamble_length)
                .permutations(2)
                .map(|x| x[0] + x[1])
                .collect::<Vec<_>>()
                .contains(&n)
        )
        .unwrap()
        .1
}


// fn part_2(program: &Program) -> i32 {
//    
//     0
// }


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

    // #[test]
    // fn test_part2_with_example() {
    //     let data = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
    //     let code = parse(&data);
    //     let result = part_2(&code);
    //     assert_eq!(result, 8);
    // }
    // 
    // #[test]
    // fn test_part2_with_real() {
    //     let data = read_to_string("../inputs/8.txt").unwrap();
    //     let code = parse(&data);
    //     let result = part_2(&code);
    //     assert_eq!(result, 797);
    // }
}
