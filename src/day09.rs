use itertools::Itertools;

fn _day09(input: &str, length: usize) -> (i64, i64) {
    let numbers: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();

    let mut part1 = 0;
    for i in length..numbers.len() {
        part1 = numbers[i];
        if !numbers[i - length..i].iter().permutations(2).any(|x| x[0] + x[1] == part1) {
            break;
        }
    }

    let mut part2 = 0;
    'outer: for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let s: i64 = numbers[i..j].iter().sum();
            if s == part1 {
                part2 = numbers[i..j].iter().min().unwrap() + numbers[i..j].iter().max().unwrap();
                break 'outer;
            } else if s > part1 {
                break;
            }
        }
    }

    (part1, part2)
}

#[cfg(test)]
mod tests_day09 {
    use std::fs::File;
    use std::io::Read;

    use crate::day09::_day09;

    #[test]
    fn part1_test_input() {
        assert_eq!(
            127,
            _day09(
                "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576",
                5,
            ).0,
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            62,
            _day09(
                "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576",
                5,
            ).1,
        );
    }

    #[test]
    fn day09() {
        let mut file = File::open("data/day09.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let (part1, part2) = _day09(&buffer, 25);
        assert_eq!(57195069, part1);
        assert_eq!(7409241, part2);
    }
}
