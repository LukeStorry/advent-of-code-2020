use regex::Regex;
use std::ops::RangeInclusive;
use std::fs::read_to_string;

pub fn solve() {
  let input = read_to_string("../inputs/2.txt").unwrap();
  let rules = parse_input(&input);
  print!("Day 2 part 1: {}\n", part_1(&rules));
  print!("Day 2 part 2: {}\n", part_2(&rules));
}

#[derive(Debug)]
pub struct Rule {
  range: RangeInclusive<usize>,
  character: char,
  password: String,
}

fn parse_input(input: &str) -> Vec<Rule> {
  let re = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
  re.captures_iter(&input)
    .filter_map(|l| parse_rule(l))
    .collect()
}

fn parse_rule(capture: regex::Captures) -> Option<Rule> {
  let lower = capture[1].parse().unwrap();
  let upper = capture[2].parse().unwrap();

  Some(Rule {
    range: (lower..=upper),
    character: capture[3].chars().next().unwrap(),
    password: capture[4].to_string(),
  })
}

fn part_1(rules: &Vec<Rule>) -> usize {
  rules
    .iter()
    .filter(|rule| {
      let count = rule
        .password
        .chars()
        .filter(|c| *c == rule.character)
        .count();
      rule.range.contains(&count)
    })
    .count()
}

fn part_2(rules: &Vec<Rule>) -> usize {
  rules
    .iter()
    .filter(|rule| {
      let low = *rule.range.start() - 1;
      let high = *rule.range.end() - 1;
      let char1 = rule.password.chars().nth(low).unwrap();
      let char2 = rule.password.chars().nth(high).unwrap();

      (char1 == rule.character) ^ (char2 == rule.character)
    })
    .count()
}

#[cfg(test)]
mod tests {
  use super::parse_input;
  use super::part_1;
  use super::part_2;

  static DATA: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

  #[test]
  fn test_parser() {
    let rules = parse_input(&DATA);
    assert_eq!(rules.len(), 3);
    assert_eq!(rules[1].range.start(), &1);
    assert_eq!(rules[1].range.end(), &3);
    assert_eq!(rules[1].character, 'b');
    assert_eq!(rules[1].password, "cdefg");
  }

  #[test]
  fn test_part1() {
    let result = part_1(&parse_input(&DATA));
    assert_eq!(result, 2);
  }

  #[test]
  fn test_part2() {
    let result = part_2(&parse_input(&DATA));
    assert_eq!(result, 1);
  }
}
