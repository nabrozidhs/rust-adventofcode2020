use std::collections::HashMap;
use itertools::Itertools;

fn _find(pos: usize, unparsed_rules: &HashMap<usize, &str>, memo: &mut HashMap<usize, Vec<String>>) -> Vec<String> {
    let p = memo.get(&pos);
    if p.is_some() {
        return p.unwrap().clone();
    }

    let v = unparsed_rules[&pos];
    if v.starts_with("\"") {
        return vec![v[1..v.len() - 1].to_string()];
    }

    let result: Vec<String> = v.split(" | ")
        .flat_map(|x|
            x.split_whitespace()
                .map(|e| _find(e.parse::<usize>().unwrap(), unparsed_rules, memo))
                .fold1(|acc, v|
                    acc.iter()
                        // .filter(|x| x.len() < 88)
                        .flat_map(|x|
                            v.iter()
                                .map(|y| format!("{}{}", x, y))
                                .collect::<Vec<String>>()
                        )
                        .collect()
                )
                .unwrap()
        )
        .collect();

    memo.insert(pos, result.clone());

    result
}

fn _day19(input: &str, _: bool) -> usize {
    let v: Vec<&str> = input.split("\n\n").collect();
    let messages = v[1];
    let mut unparsed_rules: HashMap<usize, &str> = HashMap::new();
    for line in v[0].lines() {
        let s: Vec<&str> = line.split(": ").collect();
        unparsed_rules.insert(s[0].parse().unwrap(), s[1]);
    }

    let v = _find(0, &unparsed_rules, &mut HashMap::new());

    messages.lines()
        .filter(|line| v.contains(&line.to_string()))
        .count()
}

#[cfg(test)]
mod tests_day19 {
    use std::fs::File;
    use std::io::Read;

    use crate::day19::_day19;

    #[test]
    fn part1_test_input() {
        assert_eq!(
            2,
            _day19(
                "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb",
                false,
            ),
        );
    }

    #[test]
    fn part2_test_input() {
        // assert_eq!(
        //     12,
        //     _day19(
        //         "42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        //         true,
        //     ),
        // );
    }

    #[test]
    fn day19() {
        let mut file = File::open("data/day19.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(195, _day19(&buffer, false));
        // assert_eq!(290726428573651, _day19(&buffer, true));
    }
}
