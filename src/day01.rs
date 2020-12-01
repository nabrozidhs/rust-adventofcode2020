fn _day01(input: &Vec<u64>, match_three: bool) -> u64 {
    let mut copy = input.clone();
    copy.sort();
    for i in 0..copy.len() {
        let operand1 = copy[i];
        if match_three {
            for j in i..copy.len() {
                let operand2 = copy[j];
                if operand1 + operand2 > 2020 {
                    break;
                }
                match copy[j + 1..copy.len()].binary_search(&(2020 - operand1 - operand2)) {
                    Ok(k) => return operand1 * operand2 * copy[j + k + 1],
                    Err(_) => continue,
                }
            }
        } else {
            match copy[i + 1..copy.len()].binary_search(&(2020 - operand1)) {
                Ok(j) => return operand1 * copy[i + j + 1],
                Err(_) => continue,
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests_day01 {
    use std::fs::File;
    use std::io::Read;

    use crate::day01::_day01;

    #[test]
    fn part1_test_input() {
        assert_eq!(514579, _day01(&vec![1721, 979, 366, 299, 675, 1456], false));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(241861950, _day01(&vec![1721, 979, 366, 299, 675, 1456], true));
    }

    #[test]
    fn day01() {
        let mut file = File::open("data/day01.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<u64> = buffer.lines()
            .map(|line| line.parse::<u64>().unwrap())
            .collect();

        assert_eq!(1020036, _day01(&input, false));
        assert_eq!(286977330, _day01(&input, true));
    }
}
