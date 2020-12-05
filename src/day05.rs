use std::collections::HashSet;
use std::iter::FromIterator;

fn _search(input: &str, range: u64) -> u64 {
    let mut min = 0;
    let mut max = range;

    for c in input.chars() {
        if c == 'F' || c == 'L' {
            max = min + (max - min) / 2;
        } else {
            min = min + (max - min) / 2 + 1;
        }
    }

    min
}

fn _day05(input: &str) -> (u64, u64) {
    let ids: HashSet<u64> = HashSet::from_iter(input.split("\n").into_iter()
        .map(|line| {
            _search(&line[0..7], 127) * 8 + _search(&line[7..line.len()], 7)
        })
    );

    let seat = **HashSet::from_iter(*ids.iter().min().unwrap()..=*ids.iter().max().unwrap())
        .difference(&ids)
        .collect::<Vec<&u64>>()
        .first()
        .unwrap();

    (*ids.iter().max().unwrap(), seat)
}

#[cfg(test)]
mod tests_day05 {
    use std::fs::File;
    use std::io::Read;

    use crate::day05::_day05;

    #[test]
    fn part1_test_input() {
        assert_eq!(
            820,
            _day05("BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL").0,
        );
    }

    #[test]
    fn day05() {
        let mut file = File::open("data/day05.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let (part1, part2) = _day05(&buffer);
        assert_eq!(864, part1);
        assert_eq!(739, part2);
    }
}
