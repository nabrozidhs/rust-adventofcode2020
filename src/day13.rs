use itertools::Itertools;

fn _part1(start_timestamp: u64, buses: &Vec<u64>) -> u64 {
    let next_stops: Vec<(u64, u64)> = buses.iter()
        .filter(|&&x| x > 0)
        .map(|&bus| {
            let next_stop: u64;
            if start_timestamp % bus == 0 {
                next_stop = start_timestamp
            } else {
                next_stop = ((start_timestamp / bus) + 1) * bus
            }
            (bus, next_stop)
        }).collect();

    let b = next_stops.iter().min_by_key(|(_, next_stop)| next_stop).unwrap();
    b.0 * (b.1 - start_timestamp)
}

fn _part2(_: u64, buses: &Vec<u64>) -> u64 {
    let filtered_buses: Vec<(usize, &u64)> = buses.iter().enumerate()
        .filter(|(_, &v)| v != 0)
        .sorted_by_key(|(_, &v)| v)
        .rev()
        .collect();

    let a = filtered_buses[0];
    let b = filtered_buses[1];
    let cycle = a.1 * b.1;
    let diff = a.0 as i64 - b.0 as i64;
    let mut n: u64 = 0;
    'outer: for j in 1..i64::max_value() {
        let av = *a.1 as i64 * j;
        let start = av * (*b.1 as i64 / av);
        for k in start..i64::max_value() {
            let d = av - *b.1 as i64 * k;
            if d == diff {
                n = *a.1 * j as u64 - a.0 as u64;
                break 'outer;
            } else if (d < 0 && diff > 0) || (d < diff && diff < 0) {
                break;
            }
        }
    }

    let mut t: u64 = n;
    loop {
        if filtered_buses.iter().skip(2).all(|(index, &v)| (t + *index as u64) % v == 0) {
            return t;
        }
        t += cycle;
    }
}

fn _day13(input: &str, f: &dyn Fn(u64, &Vec<u64>) -> u64) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let start_timestamp: u64 = lines[0].parse().unwrap();
    let buses: Vec<u64> = lines[1].split(",")
        .map(|x| x.parse().unwrap_or(0))
        .collect();

    f(start_timestamp, &buses)
}

#[cfg(test)]
mod tests_day13 {
    use std::fs::File;
    use std::io::Read;

    use crate::day13::{_day13, _part1, _part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(295, _day13("939\n7,13,x,x,59,x,31,19", &_part1));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(6, _day13("0\n3,x,4", &_part2));
        assert_eq!(1068781, _day13("0\n7,13,x,x,59,x,31,19", &_part2));
        assert_eq!(3417, _day13("0\n17,x,13,19", &_part2));
        assert_eq!(754018, _day13("0\n67,7,59,61", &_part2));
        assert_eq!(779210, _day13("0\n67,x,7,59,61", &_part2));
        assert_eq!(1261476, _day13("0\n67,7,x,59,61", &_part2));
        assert_eq!(1202161486, _day13("0\n1789,37,47,1889", &_part2));
    }

    #[test]
    fn day13() {
        let mut file = File::open("data/day13.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(3464, _day13(&buffer, &_part1));
        // assert_eq!(760171380521445, _day13(&buffer, &_part2));
    }
}
