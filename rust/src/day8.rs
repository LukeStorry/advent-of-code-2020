use std::fs::read_to_string;

type Program<'a> = Vec<(&'a str, i32)>;

fn parse(input: &str) -> Program {
    input.lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .map(|line| (line[0], line[1].parse().unwrap()))
        .collect()
}

pub fn run(program: &Program) -> (bool, i32) {
    let (mut acc, mut ptr) = (0, 0);
    let mut visited_pointers = vec![false; program.len()];
    loop {
        if ptr > program.len() - 1 { return (true, acc); }
        if visited_pointers[ptr] { return (false, acc); }

        visited_pointers[ptr] = true;

        match program[ptr] {
            ("acc", arg) => {
                acc += arg;
                ptr += 1
            }
            ("jmp", arg) => { ptr = (ptr as i32 + arg) as usize }
            ("nop", _) => { ptr += 1 }
            _ => { panic!("Unknown operation") }
        }
    }
}

pub fn solve() {
    let input = read_to_string("../inputs/8.txt").unwrap();
    let bags = parse(&input);
    print!("Day 8 part 1: {}\n", part_1(&bags));
    print!("Day 8 part 2: {}\n", part_2(&bags));
}

fn part_1(program: &Program) -> i32 {
    let (_, acc) = run(program);
    acc
}

fn part_2(program: &Program) -> i32 {
    for p in program.iter()
        .enumerate()
        .filter(|(_, (arg, _))| *arg == "jmp")
        .map(|(i, _)| {
            let mut possibility = program.clone();
            possibility[i].0 = "nop";
            possibility
        })
    {
        let result = run(&p);
        if result.0 { return result.1; }
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let data = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let code = parse(&data);
        assert_eq!(code[0], ("nop", 0));
        assert_eq!(code[4], ("jmp", -3));
    }

    #[test]
    fn test_parser_real() {
        let data = read_to_string("../inputs/8.txt").unwrap();
        let code = parse(&data);
        assert_eq!(code[0], ("acc", 22));
        assert_eq!(code.len(), 611);
    }

    #[test]
    fn test_part1_with_example() {
        let data = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let code = parse(&data);
        let result = part_1(&code);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part1_with_real() {
        let data = read_to_string("../inputs/8.txt").unwrap();
        let code = parse(&data);
        let result = part_1(&code);
        assert_eq!(result, 1782);
    }

    #[test]
    fn test_part2_with_example() {
        let data = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let code = parse(&data);
        let result = part_2(&code);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_part2_with_real() {
        let data = read_to_string("../inputs/8.txt").unwrap();
        let code = parse(&data);
        let result = part_2(&code);
        assert_eq!(result, 797);
    }
}
