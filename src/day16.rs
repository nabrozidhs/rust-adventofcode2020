use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

use regex::{Captures, Regex};
use itertools::Itertools;
use std::iter::FromIterator;

lazy_static! {
    static ref RE_NUM: Regex = Regex::new(r"\d+").unwrap();
}

fn _part1(_: &Vec<u64>, all_tickets: &Vec<Vec<u64>>, fields: &HashMap<&str, Vec<RangeInclusive<u64>>>) -> u64 {
    all_tickets.iter()
        .flat_map(|ticket|
            ticket.iter()
                .filter(|e| !fields.values().flat_map(|e| e).any(|r| r.contains(e)))
        )
        .sum()
}

fn _part2(my_ticket: &Vec<u64>, all_tickets: &Vec<Vec<u64>>, fields: &HashMap<&str, Vec<RangeInclusive<u64>>>) -> u64 {
    let mut keys: Vec<&str> = vec![];
    for &k in fields.keys() {
        keys.push(k);
    }

    let valid_tickets: Vec<&Vec<u64>> = all_tickets.iter()
        .filter(|&ticket| ticket.iter().all(|e| fields.values().flat_map(|v| v).any(|r| r.contains(e))))
        .collect();

    let mut search_options: Vec<HashSet<&str>> = vec![];
    for i in 0..valid_tickets.first().unwrap().len() {
        let k: Vec<&str> = keys.iter()
            .filter(|&k| {
                let r = fields.get(k).unwrap();
                valid_tickets.iter().all(|ticket| r.iter().any(|e| e.contains(&ticket[i])))
            })
            .cloned()
            .collect::<Vec<&str>>();
        search_options.push(HashSet::from_iter(k.into_iter()));
    }

    while search_options.iter().any(|e| e.len() > 1) {
        for i in 0..search_options.len() {
            if search_options[i].len() == 1 {
                let k = search_options[i].iter().next().unwrap().clone();
                for (j, option) in search_options.iter_mut().enumerate() {
                    if i != j {
                        option.remove(k);
                    }
                }
            }
        }
    }

    search_options.iter().enumerate()
        .filter(|(_, k)| k.iter().next().unwrap().starts_with("departure"))
        .map(|(i, _)| my_ticket[i])
        .fold1(|acc, x| acc * x)
        .unwrap()
}

fn _day16(input: &str, f: &dyn Fn(&Vec<u64>, &Vec<Vec<u64>>, &HashMap<&str, Vec<RangeInclusive<u64>>>) -> u64) -> u64 {
    let sections: Vec<&str> = input.split("\n\n").collect();

    let mut fields: HashMap<&str, Vec<RangeInclusive<u64>>> = HashMap::new();
    for line in sections[0].lines() {
        let name = line.split(":").collect::<Vec<&str>>()[0];
        if !fields.contains_key(name) {
            fields.insert(name, vec![]);
        }
        let captures: Vec<Captures> = RE_NUM.captures_iter(line).collect();
        let mut i = 0;
        while i < captures.len() {
            fields.get_mut(name).unwrap().push(
                captures.get(i).unwrap()[0].parse().unwrap()..=captures.get(i + 1).unwrap()[0].parse().unwrap()
            );
            i += 2;
        }
    }

    let my_ticket: Vec<u64> = sections[1].lines().skip(1).next().unwrap()
        .split(",")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut all_tickets: Vec<Vec<u64>> = vec![my_ticket.clone()];
    sections[2].lines().skip(1)
        .map(|x| x.split(",").map(|x| x.parse::<u64>().unwrap()).collect())
        .for_each(|e| all_tickets.push(e));

    f(&my_ticket, &all_tickets, &fields)
}

#[cfg(test)]
mod tests_day16 {
    use std::fs::File;
    use std::io::Read;

    use crate::day16::{_day16, _part1, _part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(71, _day16("class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12", &_part1));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(132, _day16("departure class: 0-1 or 4-19\ndeparture row: 0-5 or 8-19\nseat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9", &_part2));
    }

    #[test]
    fn day16() {
        let mut file = File::open("data/day16.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(21980, _day16(&buffer, &_part1));
        assert_eq!(1439429522627, _day16(&buffer, &_part2));
    }
}
