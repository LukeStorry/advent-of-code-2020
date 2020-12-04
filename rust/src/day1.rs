use std::fs::read_to_string;

pub fn solve() {
  let input = read_to_string("../inputs/1.txt")
      .unwrap()
      .lines()
      .map(|l| l.trim())
      .filter(|l| !l.is_empty())
      .map(|l| l.parse().unwrap())
      .collect();
  print!("Day 1 part 1: {}\n", part_1(&input));
  print!("Day 1 part 2: {}\n", part_2(&input));
}

fn part_1(entries: &Vec<i32>) -> i32 {
  for x in entries {
    let y = 2020 - x;
    if entries.contains(&y) {
      return x * y;
    }
  }
  0
}

fn part_2(entries: &Vec<i32>) -> i32 {
  for x in entries {
    for y in entries {
      let z = 2020 - x - y;
      if z < 0 {
        continue;
      };
      if entries.contains(&z) {
        return x * y * z;
      }
    }
  }
  0
}

#[cfg(test)]
mod tests {
  use super::part_1;
  use super::part_2;

  #[test]
  fn example_1() {
    let data = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(part_1(&data), 514579)
  }

  #[test]
  fn example_2() {
    let data = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(part_2(&data), 241861950)
  }
}
