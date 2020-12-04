use std::collections::HashMap;
use regex::Regex;
use std::fs::read_to_string;

type Passport<'a> = HashMap<&'a str, &'a str>;

lazy_static! {
    static ref RE_PASSPORT: Regex = Regex::new(r"(?P<field>[a-z]+):(?P<value>\S+)").unwrap();
    static ref RE_HCL: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
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

fn contains_fields(passport: &&Passport) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter()
        .all(|key| (**passport).contains_key(key))
}

fn fields_are_valid(passport: &&Passport) -> bool {
    fn int(value: &str) -> i32 { value.parse().unwrap_or(0) }
    fn height(value: &str) -> i32 { value.get(..value.len() - 2).unwrap_or_default().parse().unwrap_or(0) }

    contains_fields(passport) &&
        (**passport).iter().all(|(&key, &value)|
            match key {
                "byr" => (1920..=2002).contains(&int(value)),
                "iyr" => (2010..=2020).contains(&int(value)),
                "eyr" => (2020..=2030).contains(&int(value)),
                "hgt" if value.ends_with("cm") => (150..=193).contains(&height(value)),
                "hgt" if value.ends_with("in") => (59..=76).contains(&height(value)),
                "hcl" => RE_HCL.is_match(value),
                "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
                "pid" => RE_PID.is_match(value),
                "cid" => true,
                _ => false
            }
        )
}


pub fn solve() {
    let input = read_to_string("../inputs/4.txt").unwrap();
    let passports = parse(&input);
    print!("Day 4 part 1: {}\n", part_1(&passports));
    print!("Day 4 part 2: {}\n", part_2(&passports));
}

fn part_1(passports: &[Passport]) -> usize {
    passports.iter().filter(contains_fields).count()
}

fn part_2(passports: &[Passport]) -> usize {
    passports.iter().filter(fields_are_valid).count()
}


#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use crate::day4::parse;
    use crate::day4::part_1;
    use crate::day4::part_2;
    use crate::day4::fields_are_valid;

    #[test]
    fn test_parser_basic() {
        let data = "ecl:gry pid:860033327\n\neyr:2020";
        let passports = parse(&data);
        assert_eq!(passports[0]["ecl"], "gry");
        assert_eq!(passports[0]["pid"], "860033327");
        assert_eq!(passports[1]["eyr"], "2020");
    }

    #[test]
    fn test_part1_with_example() {
        let data = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in";
        let passports = parse(&data);
        let result = part_1(&passports);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_with_real_data() {
        let data = read_to_string("../inputs/4.txt").unwrap();
        let passports = parse(&data);
        let result = part_1(&passports);
        assert_eq!(result, 247);
    }

    #[test]
    fn test_fields_are_valid_with_invalid_examples() {
        let data = "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007";
        let passports = parse(&data);
        assert_eq!(fields_are_valid(&&passports[0]), false);
        assert_eq!(fields_are_valid(&&passports[1]), false);
        assert_eq!(fields_are_valid(&&passports[2]), false);
        assert_eq!(fields_are_valid(&&passports[3]), false);
    }

    #[test]
    fn test_fields_are_valid_with_valid_examples() {
        let data = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let passports = parse(&data);
        print!("\n{:?}\n", passports[0]);
        assert_eq!(fields_are_valid(&&passports[0]), true);
        assert_eq!(fields_are_valid(&&passports[1]), true);
        assert_eq!(fields_are_valid(&&passports[2]), true);
        assert_eq!(fields_are_valid(&&passports[3]), true);
    }

    #[test]
    fn test_part2_with_real_data() {
        let data = read_to_string("../inputs/4.txt").unwrap();
        let passports = parse(&data);
        let result = part_2(&passports);
        assert_eq!(result, 145);
    }
}
