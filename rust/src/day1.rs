use crate::utils;

fn find_corresponding(entries: &Vec<i32>) -> i32 {
  for entry in entries {
    let corresponding_entry = 2020 - entry;
    if entries.contains(&corresponding_entry) {
      return entry * corresponding_entry;
    }
  }
  0
}

pub fn part1() {
  let input = utils::get_input(1);
  print!("{}", find_corresponding(&input))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example_1() {
    let data = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(find_corresponding(&data), 514579)
  }
}
