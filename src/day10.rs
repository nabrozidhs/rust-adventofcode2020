use itertools::Itertools;

fn _part1(numbers: &Vec<u64>) -> u64 {
    let mut jolt1 = 0;
    let mut jolt3 = 0;
    for i in 1..numbers.len() {
        let diff = numbers[i] - numbers[i - 1];
        if diff == 3 {
            jolt3 += 1;
        } else if diff == 1 {
            jolt1 += 1;
        }
    }

    jolt1 * jolt3
}

fn _part2(numbers: &Vec<u64>) -> u64 {
    let mut distinct = 1;
    let mut ones_diff_in_row: i64 = 1;
    for i in 1..numbers.len() {
        if numbers[i] - numbers[i - 1] == 1 {
            ones_diff_in_row += 1;
        } else {
            distinct *= match ones_diff_in_row {
                1 | 2 => 1,
                3 => 2,
                4 => 4,
                5 => 7,
                _ => panic!("yeah I'm lazy"),
            };
            ones_diff_in_row = 1;
        }
    }

    distinct
}

fn _day10(input: &str, f: &dyn Fn(&Vec<u64>) -> u64) -> u64 {
    let mut numbers: Vec<u64> = input.lines().map(|x| x.parse().unwrap())
        .sorted()
        .collect();
    numbers.insert(0, 0);
    numbers.insert(numbers.len(), numbers.last().unwrap() + 3);

    f(&numbers)
}

#[cfg(test)]
mod tests_day10 {
    use std::fs::File;
    use std::io::Read;

    use crate::day10::{_day10, _part1, _part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(
            35,
            _day10("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4", &_part1),
        );
        assert_eq!(
            220,
            _day10(
                "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3",
                &_part1,
            ),
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            2,
            _day10("1\n2\n5\n6", &_part2),
        );
        assert_eq!(
            8,
            _day10("16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4", &_part2),
        );
        assert_eq!(
            19208,
            _day10(
                "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3",
                &_part2,
            ),
        );
    }

    #[test]
    fn day10() {
        let mut file = File::open("data/day10.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(1984, _day10(&buffer, &_part1));
        assert_eq!(3543369523456, _day10(&buffer, &_part2));
    }
}
