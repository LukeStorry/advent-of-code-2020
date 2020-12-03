use crate::utils;

pub fn solve() {
  let input = utils::get_input_chars(3);
  print!("Day 2 part 1: {}\n", part_1(&input));
  print!("Day 2 part 2: {}\n", part_2(&input));
}

fn count_trees_on_route(map: &Vec<String>, dx: usize, dy: usize) -> u32 {
  let mut trees = 0;
  let (mut x, mut y) = (0, 0);
  while y < map.len() {
    if is_tree(map, x, y) {
      trees += 1
    }
    x += dx;
    y += dy;
  }
  trees
}

fn is_tree(map: &Vec<String>, x: usize, y: usize) -> bool {
  let line = &map[y];
  let c = line.chars().nth(x % line.len()).unwrap();
  c == '#'
}

fn part_1(map: &Vec<String>) -> u32 {
  count_trees_on_route(map, 3, 1)
}

fn part_2(map: &Vec<String>) -> u64 {
  let slope1 = count_trees_on_route(map, 1, 1);
  let slope2 = count_trees_on_route(map, 3, 1);
  let slope3 = count_trees_on_route(map, 5, 1);
  let slope4 = count_trees_on_route(map, 7, 1);
  let slope5 = count_trees_on_route(map, 1, 2);
  u64::from(slope1 * slope2) * u64::from(slope3 * slope4 * slope5)
}

#[cfg(test)]
mod tests {
  use super::part_1;
  use super::part_2;

  #[test]
  fn test_part1() {
    let test_map:Vec<String> = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n".lines().map(|line| line.to_string()).collect();
    let result = part_1(&test_map);
    assert_eq!(result, 7);
  }

  #[test]
  fn test_part2() {
    let test_map:Vec<String> = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n".lines().map(|line| line.to_string()).collect();
    let result = part_2(&test_map);
    assert_eq!(result, 336);
  }
}
