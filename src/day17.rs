use std::collections::HashSet;

fn _adjacent(p: &(i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let mut adjacent = vec![];
    for x in p.0 - 1..=p.0 + 1 {
        for y in p.1 - 1..=p.1 + 1 {
            for z in p.2 - 1..=p.2 + 1 {
                let v = (x, y, z);
                if v != *p {
                    adjacent.push(v);
                }
            }
        }
    }
    adjacent
}

fn _day17_part1(input: &str) -> u64 {
    let mut actives: HashSet<(i64, i64, i64)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                actives.insert((x as i64, y as i64, 0));
            }
        }
    }

    for _ in 0..6 {
        let x_min = actives.iter().min_by_key(|x| x.0).unwrap().0 - 1;
        let x_max = actives.iter().max_by_key(|x| x.0).unwrap().0 + 1;
        let y_min = actives.iter().min_by_key(|x| x.1).unwrap().1 - 1;
        let y_max = actives.iter().max_by_key(|x| x.1).unwrap().1 + 1;
        let z_min = actives.iter().min_by_key(|x| x.2).unwrap().2 - 1;
        let z_max = actives.iter().max_by_key(|x| x.2).unwrap().2 + 1;

        let mut new_state: HashSet<(i64, i64, i64)> = HashSet::new();
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    let p = (x, y, z);
                    let nearby_actives = _adjacent(&p).iter()
                        .filter(|x| actives.contains(x))
                        .take(4)
                        .count();
                    if nearby_actives == 3 || (actives.contains(&p) && nearby_actives == 2) {
                        new_state.insert(p);
                    }
                }
            }
        }
        actives = new_state;
    }
    actives.len() as u64
}

fn _adjacent4(p: &(i64, i64, i64, i64)) -> Vec<(i64, i64, i64, i64)> {
    let mut adjacent = vec![];
    for x in p.0 - 1..=p.0 + 1 {
        for y in p.1 - 1..=p.1 + 1 {
            for z in p.2 - 1..=p.2 + 1 {
                for w in p.3 - 1..=p.3 + 1 {
                    let v = (x, y, z, w);
                    if v != *p {
                        adjacent.push(v);
                    }
                }
            }
        }
    }
    adjacent
}

fn _day17_part2(input: &str) -> u64 {
    let mut actives: HashSet<(i64, i64, i64, i64)> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                actives.insert((x as i64, y as i64, 0, 0));
            }
        }
    }

    for _ in 0..6 {
        let x_min = actives.iter().min_by_key(|x| x.0).unwrap().0 - 1;
        let x_max = actives.iter().max_by_key(|x| x.0).unwrap().0 + 1;
        let y_min = actives.iter().min_by_key(|x| x.1).unwrap().1 - 1;
        let y_max = actives.iter().max_by_key(|x| x.1).unwrap().1 + 1;
        let z_min = actives.iter().min_by_key(|x| x.2).unwrap().2 - 1;
        let z_max = actives.iter().max_by_key(|x| x.2).unwrap().2 + 1;
        let w_min = actives.iter().min_by_key(|x| x.3).unwrap().3 - 1;
        let w_max = actives.iter().max_by_key(|x| x.3).unwrap().3 + 1;

        let mut new_state: HashSet<(i64, i64, i64, i64)> = HashSet::new();
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    for w in w_min..=w_max {
                        let p = (x, y, z, w);
                        let nearby_actives = _adjacent4(&p).iter()
                            .filter(|x| actives.contains(x))
                            .take(4)
                            .count();
                        if nearby_actives == 3 || (actives.contains(&p) && nearby_actives == 2) {
                            new_state.insert(p);
                        }
                    }
                }
            }
        }
        actives = new_state;
    }
    actives.len() as u64
}

#[cfg(test)]
mod tests_day17 {
    use std::fs::File;
    use std::io::Read;

    use crate::day17::{_day17_part1, _day17_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(112, _day17_part1(".#.\n..#\n###"));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(848, _day17_part2(".#.\n..#\n###"));
    }

    #[test]
    fn day17() {
        let mut file = File::open("data/day17.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(336, _day17_part1(&buffer));
        assert_eq!(2620, _day17_part2(&buffer));
    }
}
