use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use regex::Regex;

lazy_static! {
    static ref RE_HCL: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    static ref RE_ECL: Regex = Regex::new(r"^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$").unwrap();
    static ref RE_PID: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}
fn _part1(_: &HashMap<&str, &str>) -> bool { true }

fn _part2(m: &HashMap<&str, &str>) -> bool {
    let hgt = m["hgt"];
    (1920..=2002).contains(&m["byr"].parse::<i64>().unwrap()) &&
        (2010..=2020).contains(&m["iyr"].parse::<i64>().unwrap()) &&
        (2020..=2030).contains(&m["eyr"].parse::<i64>().unwrap()) &&
        RE_HCL.is_match(m["hcl"]) &&
        RE_ECL.is_match(m["ecl"]) &&
        RE_PID.is_match(m["pid"]) &&
        ((hgt.ends_with("cm") && (150..=193).contains(&hgt[0..hgt.len() - 2].parse::<i64>().unwrap())) ||
            (hgt.ends_with("in") && (59..=76).contains(&hgt[0..hgt.len() - 2].parse::<i64>().unwrap())))
}

fn _day04(input: &str, f: &dyn Fn(&HashMap<&str, &str>) -> bool) -> usize {
    let required_items: HashSet<&str> =
        HashSet::from_iter(vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned());
    input.split("\n\n").into_iter()
        .map(|line| {
            let mut m: HashMap<&str, &str> = HashMap::new();
            for item in line.split_ascii_whitespace() {
                let s = item.split(":").collect::<Vec<&str>>();
                m.insert(s[0], s[1]);
            }
            m
        })
        .filter(|items|
            required_items.is_subset(&HashSet::from_iter(items.keys().into_iter().cloned())) &&
                f(items)
        )
        .count()
}

#[cfg(test)]
mod tests_day04 {
    use std::fs::File;
    use std::io::Read;

    use crate::day04::_day04;
    use crate::day04::_part1;
    use crate::day04::_part2;

    #[test]
    fn part1_test_input() {
        assert_eq!(
            2,
            _day04(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm\n\niyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929\n\nhcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm\n\nhcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in",
                &_part1,
            ),
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            0,
            _day04(
                "eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n\niyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946\n\nhcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n\nhgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007",
                &_part2,
            ),
        );
        assert_eq!(
            4,
            _day04(
                "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f\n\neyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\nhcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2022\n\niyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
                &_part2,
            ),
        );
    }

    #[test]
    fn day04() {
        let mut file = File::open("data/day04.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(228, _day04(&buffer, &_part1));
        assert_eq!(175, _day04(&buffer, &_part2));
    }
}
