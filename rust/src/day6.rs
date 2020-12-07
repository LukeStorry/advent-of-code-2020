use std::fs::read_to_string;
use std::collections::HashSet;


pub fn solve() {
    let answers = read_to_string("../inputs/6.txt").unwrap();
    print!("Day 5 part 1: {}\n", part_1(&answers));
    print!("Day 5 part 2: {}\n", part_2(&answers));
}

fn part_1(answers: &str) -> usize {
    answers.split("\n\n")
        .map(|group| {
            group.lines()
                .flat_map(|line| line.chars())
                .collect::<HashSet<char>>()
                .len()
        }).sum()
}

fn part_2(answers: &str) -> usize {
    answers.split("\n\n")
        .map(|group| {
            group.lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .enumerate()
                .fold(HashSet::new(),
                      |acc: HashSet<char>, (i, answers)|
                          if i == 0 {
                              answers
                          } else {
                              acc.intersection(&answers)
                                  .map(|c| *c)
                                  .collect::<HashSet<char>>()
                          },
                )
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answers = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(part_1(&answers), 11);
    }

    #[test]
    fn test_part1_real() {
        let answers = read_to_string("../inputs/6.txt").unwrap();
        assert_eq!(part_1(&answers), 6170);
    }

    #[test]
    fn test_part2() {
        let answers = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(part_2(&answers), 6);
    }

    #[test]
    fn test_part2_real() {
        let answers = read_to_string("../inputs/6.txt").unwrap();
        assert_eq!(part_2(&answers), 2947);
    }
}
