use regex::Regex;
use std::ops::RangeInclusive;

struct _Line {
    range: RangeInclusive<usize>,
    character: char,
    password: String,
}

fn _part1(line: &_Line) -> bool {
    line.range.contains(&line.password.chars().filter(|x| *x == line.character).count())
}

fn _part2(line: &_Line) -> bool {
    let string_as_bytes = line.password.as_bytes();
    [line.range.start() - 1, line.range.end() - 1].iter()
        .filter(|x| string_as_bytes[**x] as char == line.character)
        .count() == 1
}

fn _day02(input: &str, f: &dyn Fn(&_Line) -> bool) -> usize {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
    input.lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            _Line {
                range: cap[1].parse().unwrap()..=cap[2].parse().unwrap(),
                character: cap[3].parse().unwrap(),
                password: cap[4].parse().unwrap(),
            }
        })
        .filter(|line| f(line))
        .count()
}

#[cfg(test)]
mod tests_day02 {
    use std::fs::File;
    use std::io::Read;

    use crate::day02::_day02;
    use crate::day02::_part1;
    use crate::day02::_part2;

    #[test]
    fn part1_test_input() {
        assert_eq!(2, _day02("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc", &_part1));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(1, _day02("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc", &_part2));
    }

    #[test]
    fn day02() {
        let mut file = File::open("data/day02.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(603, _day02(&buffer, &_part1));
        assert_eq!(404, _day02(&buffer, &_part2));
    }
}
