fn _part1(ops: &Vec<char>, stack: &mut Vec<u64>) -> u64 {
    for o in ops {
        let operand1 = stack.remove(0);
        let operand2 = stack.remove(0);
        match o {
            '+' => stack.insert(0, operand1 + operand2),
            '*' => stack.insert(0, operand1 * operand2),
            _ => panic!(),
        }
    }

    stack.pop().unwrap()
}

fn _part2(ops: &Vec<char>, stack: &mut Vec<u64>) -> u64 {
    for (i, _) in ops.iter().enumerate().filter(|(_, &o)| o == '+').rev() {
        let operand1 = stack.remove(i);
        let operand2 = stack.remove(i);
        stack.insert(i, operand1 + operand2);
    }

    stack.iter().product()
}

fn _eval(input: &str, f: &dyn Fn(&Vec<char>, &mut Vec<u64>) -> u64) -> u64 {
    let mut stack: Vec<u64> = vec![];
    let mut ops: Vec<char> = vec![];

    let mut i = 0;
    let chars = input.chars().collect::<Vec<char>>();
    while i < chars.len() {
        let e = chars[i];
        match e {
            '+' | '*' => ops.push(e),
            ' ' => {}
            '(' => {
                let start = i + 1;
                let mut count = 1;
                while count > 0 {
                    i += 1;
                    match chars[i] {
                        ')' => count -= 1,
                        '(' => count += 1,
                        _ => {}
                    }
                }
                stack.push(_eval(&input[start..i], f));
            }
            _ => stack.push(e.to_digit(10).unwrap() as u64),
        }
        i += 1;
    }

    f(&ops, &mut stack)
}

fn _day18(input: &str, f: &dyn Fn(&Vec<char>, &mut Vec<u64>) -> u64) -> u64 {
    input.lines().map(|line| _eval(line, f)).sum()
}

#[cfg(test)]
mod tests_day18 {
    use std::fs::File;
    use std::io::Read;

    use crate::day18::{_day18,_part1, _part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(71, _day18("1 + 2 * 3 + 4 * 5 + 6", &_part1));
        assert_eq!(26, _day18("2 * 3 + (4 * 5)", &_part1));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(231, _day18("1 + 2 * 3 + 4 * 5 + 6", &_part2));
        assert_eq!(51, _day18("1 + (2 * 3) + (4 * (5 + 6))", &_part2));
    }

    #[test]
    fn day18() {
        let mut file = File::open("data/day18.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(11004703763391, _day18(&buffer, &_part1));
        assert_eq!(290726428573651, _day18(&buffer, &_part2));
    }
}
