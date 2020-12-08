use std::collections::{HashSet, HashMap};
use regex::Regex;
use itertools::any;

lazy_static! {
    static ref RE_KEY_BAG: Regex = Regex::new(r"(\w+ \w+ bag)").unwrap();
    static ref RE_VALUE_BAG: Regex = Regex::new(r"(\d+) (\w+ \w+ bag)").unwrap();
}

fn _find_parents(bag: &String, bags: &HashMap<String, Vec<(u64, String)>>) -> Vec<String> {
    bags.iter()
        .filter(|(_, v)| any(*v, |x| x.1 == *bag))
        .map(|(k, _)| k.to_string())
        .collect()
}

fn _part1(bag: &String, bags: &HashMap<String, Vec<(u64, String)>>) -> u64 {
    let mut found: HashSet<String> = HashSet::new();

    let mut parents = _find_parents(bag, bags);
    while !parents.is_empty() {
        let c = parents.remove(0);
        if found.contains(&c) {
            continue;
        }
        found.insert(c.to_string());
        for v in _find_parents(&c, &bags) {
            parents.push(v);
        }
    }

    found.len() as u64
}

fn _part2(bag: &String, bags: &HashMap<String, Vec<(u64, String)>>) -> u64 {
    match bags.get(bag) {
        Some(value) => value.iter().map(|e| e.0 + e.0 * _part2(&e.1, bags)).sum(),
        None => 0,
    }
}

fn _day07(input: &str, f: &dyn Fn(&String, &HashMap<String, Vec<(u64, String)>>) -> u64) -> u64 {
    let mut bags: HashMap<String, Vec<(u64, String)>> = HashMap::new();
    for line in input.lines() {
        let key = RE_KEY_BAG.captures(&line).unwrap()[1].to_string();
        let value: Vec<(u64, String)> = RE_VALUE_BAG.captures_iter(&line)
            .map(|c| (c[1].parse().unwrap(), c[2].to_string()))
            .collect();
        bags.insert(key, value);
    }

    f(&"shiny gold bag".to_string(), &bags)
}

#[cfg(test)]
mod tests_day07 {
    use std::fs::File;
    use std::io::Read;

    use crate::day07::{_day07, _part1, _part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(
            4,
            _day07(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.\ndark orange bags contain 3 bright white bags, 4 muted yellow bags.\nbright white bags contain 1 shiny gold bag.\nmuted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\nshiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\ndark olive bags contain 3 faded blue bags, 4 dotted black bags.\nvibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\nfaded blue bags contain no other bags.\ndotted black bags contain no other bags.",
                &_part1,
            ),
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            126,
            _day07(
                "shiny gold bags contain 2 dark red bags.\ndark red bags contain 2 dark orange bags.\ndark orange bags contain 2 dark yellow bags.\ndark yellow bags contain 2 dark green bags.\ndark green bags contain 2 dark blue bags.\ndark blue bags contain 2 dark violet bags.\ndark violet bags contain no other bags.",
                &_part2,
            ),
        );
    }

    #[test]
    fn day07() {
        let mut file = File::open("data/day07.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(172, _day07(&buffer, &_part1));
        assert_eq!(39645, _day07(&buffer, &_part2));
    }
}
