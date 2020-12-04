use std::collections::HashMap;
use regex::Regex;

type Passport<'a> = HashMap<&'a str, &'a str>;

lazy_static! {
    static ref RE_PASSPORT :Regex =Regex::new(r"(?P<field>[a-z]+):(?P<value>\S+)").unwrap();
}

fn parse(input: &str) -> Vec<Passport> {
    input.split("\n\n")
        .map(|passport| RE_PASSPORT.captures_iter(passport.as_ref())
            .map(|captures|
                (
                    captures.name("field").unwrap().as_str(),
                    captures.name("value").unwrap().as_str()
                )
            )
            .collect())
        .collect()
}

pub fn solve() {
    // let input = read_to_string("../inputs/3.txt")
    //     .unwrap()
    //     .lines()
    //     .map(|line| line.trim())
    //     .filter(|line| !line.is_empty())
    //     .map(|line| line.to_string())
    //     .collect();
    // print!("Day 2 part 1: {}\n", part_1(&input));
    // print!("Day 2 part 2: {}\n", part_2(&input));
}

fn part_1(passports: &Vec<Passport>) -> usize {
    0
}

fn part_2(passports: &Vec<Passport>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    // use super::part_1;
    // use super::part_2;
    use super::parse;

    #[test]
    fn test_parser() {
        let data = "ecl:gry pid:860033327\n\neyr:2020";
        let passports = parse(&data);
        assert_eq!(passports[0]["ecl"], "gry");
        assert_eq!(passports[0]["pid"], "860033327");
        assert_eq!(passports[1]["eyr"], "2020");
    }

    #[test]
    fn test_part1() {
        // let data = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";
    }

    #[test]
    fn test_part2() {}
}
