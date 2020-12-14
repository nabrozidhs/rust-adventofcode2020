use std::collections::HashMap;

fn _u64_to_binary(value: u64) -> Vec<char> {
    let mut binary: Vec<char> = vec![];

    let mut v = value;
    for _ in 0..36 {
        if v % 2 == 1 {
            binary.push('1');
        } else {
            binary.push('0');
        }
        v /= 2;
    }
    binary.reverse();

    binary
}

fn _binary_to_u64(binary: &Vec<char>) -> u64 {
    let mut v = 0;

    for (i, &c) in binary.iter().rev().enumerate() {
        if c == '1' {
            v += 2_u64.pow(i as u32);
        }
    }

    v
}

fn _part1(lines: &Vec<&str>) -> HashMap<usize, u64> {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut mask: Vec<char> = vec![];
    for line in lines {
        let split: Vec<&str> = line.split(" = ").collect();
        if line.starts_with("mask") {
            mask = split[1].chars().collect();
        } else {
            let address: usize = split[0].get(4..split[0].len() - 1).unwrap().parse().unwrap();
            let value: Vec<char> = _u64_to_binary(split[1].parse().unwrap());

            mem.insert(
                address,
                _binary_to_u64(
                    &mask.iter().zip(value)
                        .map(|(&a, b)| if a == 'X' { b } else { a })
                        .collect()
                ),
            );
        }
    }
    mem
}

fn _addresses(address: &Vec<char>) -> Vec<usize> {
    if address.is_empty() {
        return vec![0];
    }

    let next = _addresses(&address.clone().into_iter().skip(1).collect());
    match address[0] {
        '1' => next.iter().map(|x| 1 + 2 * x).collect(),
        '0' => next.iter().map(|x| 2 * x).collect(),
        'X' => {
            let mut v = vec![];
            next.iter().map(|x| 2 * x).for_each(|e| v.push(e));
            next.iter().map(|x| 1 + 2 * x).for_each(|e| v.push(e));
            v
        }
        _ => panic!(),
    }
}

fn _part2(lines: &Vec<&str>) -> HashMap<usize, u64> {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut mask: Vec<char> = vec![];
    for line in lines {
        let split: Vec<&str> = line.split(" = ").collect();
        if line.starts_with("mask") {
            mask = split[1].chars().collect();
        } else {
            let address: Vec<char> = _u64_to_binary(split[0].get(4..split[0].len() - 1).unwrap().parse().unwrap());
            let value: u64 = split[1].parse().unwrap();

            let addresses = _addresses(
                &mask.iter().zip(address)
                    .map(|(&a, b)| if a == '1' || a == 'X' { a } else { b })
                    .collect()
            );

            addresses.iter().for_each(|&a| { mem.insert(a, value); });
        }
    }
    mem
}

fn _day14(input: &str, f: &dyn Fn(&Vec<&str>) -> HashMap<usize, u64>) -> u64 {
    let lines: Vec<&str> = input.lines().collect();

    f(&lines).values().sum()
}

#[cfg(test)]
mod tests_day14 {
    use std::fs::File;
    use std::io::Read;

    use crate::day14::{_day14, _part1, _part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(165, _day14("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0", &_part1));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(208, _day14("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1", &_part2));
    }

    #[test]
    fn day14() {
        let mut file = File::open("data/day14.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(11501064782628, _day14(&buffer, &_part1));
        assert_eq!(5142195937660, _day14(&buffer, &_part2));
    }
}
