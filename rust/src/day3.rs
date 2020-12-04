use std::fs::read_to_string;

pub fn solve() {
    let input = read_to_string("../inputs/3.txt")
        .unwrap()
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect();
    print!("Day 3 part 1: {}\n", part_1(&input));
    print!("Day 3 part 2: {}\n", part_2(&input));
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
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| u64::from(count_trees_on_route(map, *dx, *dy)))
        .product()
}

#[cfg(test)]
mod tests {
    use super::part_1;
    use super::part_2;

    #[test]
    fn test_part1() {
        let test_map: Vec<String> = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n".lines().map(|line| line.to_string()).collect();
        let result = part_1(&test_map);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part2() {
        let test_map: Vec<String> = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n".lines().map(|line| line.to_string()).collect();
        let result = part_2(&test_map);
        assert_eq!(result, 336);
    }
}
