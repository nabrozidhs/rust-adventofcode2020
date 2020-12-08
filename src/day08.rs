#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Clone,Copy,PartialEq)]
enum Command {
    NOP,
    ACC,
    JMP,
}

fn _run_program(program: &Vec<(Command, i64)>) -> (bool, i64) {
    let mut acc = 0;
    let mut idx: i64 = 0;
    let mut visited: HashSet<i64> = HashSet::new();
    while idx < program.len() as i64 && !visited.contains(&idx) {
        visited.insert(idx);
        let (command, inc) = program.get(idx as usize).unwrap();
        match command {
            Command::NOP => {}
            Command::ACC => acc += inc,
            Command::JMP => idx += inc - 1,
        }
        idx += 1;
    }

    (idx >= program.len() as i64, acc)
}

fn _day08(input: &str) -> (i64, i64) {
    let mut program: Vec<(Command, i64)> = input.lines().map(|x| {
        let s: Vec<&str> = x.split(" ").collect();
        let command = match s[0] {
            "nop" => Command::NOP,
            "acc" => Command::ACC,
            "jmp" => Command::JMP,
            _ => panic!(),
        };
        (command, s[1].parse::<i64>().unwrap())
    })
        .collect();

    let mut part2 = 0;
    for i in 0..program.len() {
        let (c, v) = program.get(i).unwrap().clone();
        if c == Command::NOP {
            program[i] = (Command::JMP, v);
        } else if c == Command::JMP {
            program[i] = (Command::NOP, v);
        } else {
            continue;
        }

        let result = _run_program(&program);
        program[i] = (c, v);
        if result.0 {
            part2 = result.1;
            break;
        }
    }

    (_run_program(&program).1, part2)
}

#[cfg(test)]
mod tests_day08 {
    use std::fs::File;
    use std::io::Read;

    use crate::day08::_day08;

    #[test]
    fn part1_test_input() {
        assert_eq!(
            5,
            _day08("nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6").0,
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            8,
            _day08("nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6").1,
        );
    }

    #[test]
    fn day08() {
        let mut file = File::open("data/day08.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let (part1, part2) = _day08(&buffer);
        assert_eq!(1521, part1);
        assert_eq!(1016, part2);
    }
}
