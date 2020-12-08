use std::fs::read_to_string;
use std::num::ParseIntError;
use std::mem::discriminant;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl std::str::FromStr for Instruction {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        match (split[0], split[1]) {
            ("acc", arg) => Ok(Instruction::Acc(arg.parse()?)),
            ("jmp", arg) => Ok(Instruction::Jmp(arg.parse()?)),
            ("nop", arg) => Ok(Instruction::Nop(arg.parse()?)),

            (u, _) => { panic!("Unknown operation {}", u) }
        }
    }
}

type Program = Vec<Instruction>;

#[derive(Debug, Clone)]
struct State {
    program: Program,
    acc: i32,
    ptr: usize,
    visited_ptrs: Vec<bool>,
    hit_inifinite_loop: bool,
    finished: bool,
}

impl State {
    fn new(program: &Program) -> Self {
        let len = program.len();
        let p = program.clone();
        State {
            program: p,
            acc: 0,
            ptr: 0,
            visited_ptrs: vec![false; len],
            hit_inifinite_loop: false,
            finished: false,
        }
    }

    fn step(&self) -> Self {
        // print!("{:?}\n", self);
        let Self { program, acc, ptr, .. } = self;
        let mut visited_ptrs = self.visited_ptrs.clone();
        visited_ptrs[*ptr] = true;

        let (acc, ptr) = match program[*ptr] {
            Instruction::Acc(arg) => (acc + arg, *ptr + 1),
            Instruction::Jmp(arg) => (*acc, (*ptr as i32 + arg) as usize),
            Instruction::Nop(_) => (*acc, *ptr + 1)
        };

        let finished = ptr > program.len() - 1;
        let hit_inifinite_loop = !finished && visited_ptrs[ptr];
        Self { program: self.program.clone(), acc, ptr, visited_ptrs, hit_inifinite_loop, finished }
    }

    fn run_until_completion(&self) -> State {
        let mut state = self.step();
        while !state.hit_inifinite_loop && !state.finished {
            state = state.step();
        }
        state
    }
}


fn parse(input: &str) -> Program {
    input.lines().map(|line| line.parse().unwrap()).collect()
}


pub fn solve() {
    let input = read_to_string("../inputs/8.txt").unwrap();
    let program = parse(&input);
    print!("Day 8 part 1: {}\n", part_1(&program));
    print!("Day 8 part 2: {}\n", part_2(&program));
}

fn part_1(program: &Program) -> i32 {
    State::new(program).run_until_completion().acc
}

fn part_2(program: &Program) -> i32 {
    program.iter()
        .enumerate()
        .map(|(i, _)| {
            let mut possibility = program.clone();
            possibility[i] = match possibility[i] {
                Instruction::Acc(a) => Instruction::Acc(a),
                Instruction::Jmp(a) => Instruction::Nop(a),
                Instruction::Nop(a) => Instruction::Jmp(a),
            };
            State::new(&possibility).run_until_completion()
        })
        .filter(|state| state.finished)
        .nth(0)
        .unwrap()
        .acc
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let data = "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";
        let code = parse(&data);
        assert_eq!(code[0], Instruction::Nop(0));
        assert_eq!(code[4], Instruction::Jmp(-3));
    }

    #[test]
    fn test_parser_real() {
        let data = read_to_string("../inputs/8.txt").unwrap();
        let code = parse(&data);
        assert_eq!(code[0], Instruction::Acc(22));
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
