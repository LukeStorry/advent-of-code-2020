use crate::utils;

pub fn solve() {
  let input = utils::get_input(X);
  print!("Day X part 1: {}\n", part_1(&input));
  print!("Day X part 2: {}\n", part_2(&input));
}

fn part_1(entries: &Vec<i32>) -> i32 {
  0
}

fn part_2(entries: &Vec<i32>) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use super::part_1;
  use super::part_2;

  #[test]
  fn example_1() {
    let data = vec![];
    assert_eq!(part_1(&data), 0)
  }

  #[test]
  fn example_2() {
    let data = vec![];
    assert_eq!(part_2(&data), 0)
  }
}
