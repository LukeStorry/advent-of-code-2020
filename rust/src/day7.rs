use std::collections::HashMap;
use regex::Regex;
use std::fs::read_to_string;


type Contents<'a> = Vec<(u32, &'a str)>;
type BagsMap<'a> = HashMap<&'a str, Contents<'a>>;

fn parse(input: &str) -> BagsMap {
    let re_line = Regex::new(r"(\w+ \w+) bags contain (.*).").unwrap();
    let re_contents = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();

    input.split("\n\n")
        .flat_map(|line| re_line.captures_iter(line.as_ref())
            .map(|line_captures| {
                let colour = line_captures.get(1).unwrap().as_str();
                let contents_str = line_captures.get(2).unwrap().as_str();
                let contents: Contents = re_contents.captures_iter(contents_str)
                    .map(|contents_captures| (
                        contents_captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                        contents_captures.get(2).unwrap().as_str()
                    )).collect();
                (colour, contents)
            }))
        .collect()
}


pub fn solve() {
    let input = read_to_string("../inputs/7.txt").unwrap();
    let bags = parse(&input);
    print!("Day 7 part 1: {}\n", part_1(&bags));
    print!("Day 7 part 2: {}\n", part_2(&bags));
}

fn part_1(bags: &BagsMap) -> u32 {
    fn bag_contains_gold(bags: &BagsMap, bag: &str) -> bool {
        bags[bag].iter().any(|(_, inner_bag)|
            inner_bag == &"shiny gold" || bag_contains_gold(bags, inner_bag)
        )
    }

    bags.keys()
        .map(|bag| bag_contains_gold(bags, bag) as u32)
        .sum()
}


fn part_2(bags: &BagsMap) -> u32 {
    fn bag_must_contain(bags: &BagsMap, bag: &str) -> u32 {
        bags[bag].iter().map(|(count, inner_bag)|
            count + count * bag_must_contain(bags, inner_bag)
        ).sum()
    }

    bag_must_contain(bags, "shiny gold")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let data = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        let bags = parse(&data);
        assert_eq!(bags["light red"], vec!((1, "bright white"), (2, "muted yellow")));
    }

    #[test]
    fn test_part1_with_example() {
        let data = "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.";
        let bags = parse(&data);
        let result = part_1(&bags);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part1_with_real_data() {
        let data = read_to_string("../inputs/7.txt").unwrap();
        let bags = parse(&data);
        let result = part_1(&bags);
        assert_eq!(result, 148);
    }

    #[test]
    fn test_part2_with_example() {
        let data = "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.";
        let bags = parse(&data);
        let result = part_2(&bags);
        assert_eq!(result, 126);
    }

    #[test]
    fn test_part2_with_real_data() {
        let data = read_to_string("../inputs/7.txt").unwrap();
        let bags = parse(&data);
        let result = part_2(&bags);
        assert_eq!(result, 24867);
    }
}
