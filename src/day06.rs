use std::collections::HashSet;
use std::iter::FromIterator;

use itertools::Itertools;

fn _part1(group: &str) -> HashSet<char> {
    HashSet::from_iter(group.split_whitespace().map(|x| x.chars()).flatten().into_iter())
}

fn _part2(group: &str) -> HashSet<char> {
    group.split_whitespace()
        .map(|line| HashSet::from_iter(line.chars().into_iter()))
        .fold1(|acc, next| HashSet::from_iter(acc.intersection(&next).into_iter().cloned()))
        .unwrap()
}

fn _day06(input: &str, f: &dyn Fn(&str) -> HashSet<char>) -> usize {
    input.split("\n\n").map(|group| f(group).len()).sum()
}

#[cfg(test)]
mod tests_day06 {
    use std::fs::File;
    use std::io::Read;

    use crate::day06::_day06;
    use crate::day06::_part1;
    use crate::day06::_part2;

    #[test]
    fn part1_test_input() {
        assert_eq!(
            11,
            _day06("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb", &_part1),
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            6,
            _day06("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb", &_part2),
        );
    }

    #[test]
    fn day06() {
        let mut file = File::open("data/day06.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(6530, _day06(&buffer, &_part1));
        assert_eq!(3323, _day06(&buffer, &_part2));
    }
}
