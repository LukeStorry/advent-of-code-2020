use std::collections::HashMap;
use regex::Regex;
use std::fs::read_to_string;

type Passport<'a> = HashMap<&'a str, &'a str>;

lazy_static! {
    static ref RE_PASSPORT :Regex =Regex::new(r"(?P<field>[a-z]+):(?P<value>\S+)").unwrap();
}

fn parse(input: &str) -> Vec<Passport> {
    input.split("\n\n")
        .map(|passport| RE_PASSPORT.captures_iter(passport.as_ref())
            .map(|captures| (
                captures.name("field").unwrap().as_str(),
                captures.name("value").unwrap().as_str()
            )).collect())
        .collect()
}

pub fn solve() {
    let input = read_to_string("../inputs/4.txt").unwrap();
    let passports = parse(&input);
    print!("Day 4 part 1: {}\n", part_1(&passports));
    print!("Day 4 part 2: {}\n", part_2(&passports));
}

fn valid1(passport: &&Passport) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|key| (**passport).contains_key(key))
}

fn valid2(passport: &&Passport) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|key| (**passport).contains_key(key))
}

fn part_1(passports: &Vec<Passport>) -> usize {
    passports.iter().filter(valid1).count()
}

fn part_2(passports: &Vec<Passport>) -> usize {
    passports.iter().filter(valid2).count()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::parse;
    use super::part_1;
    use super::part_2;

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
        let data = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";
        let passports = parse(&data);
        let result = part_1(&passports);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2() {
        let data = read_to_string("../inputs/1.txt").unwrap();
        let passports = parse(&data);
    }
}
