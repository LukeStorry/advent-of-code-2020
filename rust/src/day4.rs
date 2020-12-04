use std::str::FromStr;

#[derive(Debug)]
struct ParseError {
    message: String
}

#[derive(Default, Debug)]
struct Passport<'a> {
    byr: Option<&'a str>,
    iyr: Option<&'a str>,
    eyr: Option<&'a str>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}

impl FromStr for Passport<'static> {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default(); // initialise with all fields as None
        let fields: Vec<(&'static str, &'static str)> = s.split_whitespace()
            .map(|f| f.split(':').take(2).collect::<Vec<&str>>())
            .map(|split| (split[0], split[1]))
            .collect();
        for (key, value) in fields {
            match key {
                "byr" => passport.byr = Some(value),
                "iyr" => passport.iyr = Some(value),
                "eyr" => passport.eyr = Some(value),
                "hgt" => passport.hgt = Some(value),
                "hcl" => passport.hcl = Some(value),
                "ecl" => passport.ecl = Some(value),
                "pid" => passport.pid = Some(value),
                "cid" => passport.cid = Some(value),
                unknown => return Err(ParseError { message: format!("unknown field: {}", unknown) })
            }
        }

        Ok(passport)
    }
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
    use super::Passport;
    use std::str::FromStr;

    #[test]
    fn test_parser() {
        let data = "";
        let passport = Passport::from_str(data);
        print!("{:?}", passport)
    }

    #[test]
    fn test_part1() {}

    #[test]
    fn test_part2() {}
}
