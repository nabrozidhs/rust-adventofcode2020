use std::collections::HashMap;

fn _day15(input: &Vec<usize>, limit: usize) -> usize {
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, &n) in input.iter().enumerate() {
        map.insert(n, vec![i]);
    }

    let empty: Vec<usize> = vec![];
    let mut next: usize = *input.last().unwrap();
    for i in input.len()..limit {
        let v = map.get(&next).unwrap_or(&empty);
        if v.len() > 1 {
            next = v[v.len() - 1] - v[v.len() - 2];
        } else {
            next = 0;
        }

        if !map.contains_key(&next) {
            map.insert(next, vec![i]);
        } else {
            map.get_mut(&next).unwrap().push(i);
        }
    }
    next
}

#[cfg(test)]
mod tests_day15 {
    use crate::day15::_day15;

    #[test]
    fn part1_test_input() {
        assert_eq!(436, _day15(&vec![0, 3, 6], 2020));
        assert_eq!(1, _day15(&vec![1, 3, 2], 2020));
        assert_eq!(10, _day15(&vec![2, 1, 3], 2020));
        assert_eq!(27, _day15(&vec![1, 2, 3], 2020));
        assert_eq!(78, _day15(&vec![2, 3, 1], 2020));
        assert_eq!(438, _day15(&vec![3, 2, 1], 2020));
        assert_eq!(1836, _day15(&vec![3, 1, 2], 2020));
    }

    // #[test]
    // fn part2_test_input() {
    //     assert_eq!(175594, _day15(&vec![0, 3, 6], 30000000));
    //     assert_eq!(2578, _day15(&vec![1, 3, 2], 30000000));
    //     assert_eq!(3544142, _day15(&vec![2, 1, 3], 30000000));
    //     assert_eq!(261214, _day15(&vec![1, 2, 3], 30000000));
    //     assert_eq!(6895259, _day15(&vec![2, 3, 1], 30000000));
    //     assert_eq!(18, _day15(&vec![3, 2, 1], 30000000));
    //     assert_eq!(362, _day15(&vec![3, 1, 2], 30000000));
    // }

    #[test]
    fn day15() {
        assert_eq!(1696, _day15(&vec![12, 1, 16, 3, 11, 0], 2020));
        // assert_eq!(37385, _day15(&vec![12, 1, 16, 3, 11, 0], 30000000));
    }
}
