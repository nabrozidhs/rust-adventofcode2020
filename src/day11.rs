use crate::util::Vector2;

lazy_static! {
    static ref VECTORS: Vec<Vector2> = vec![
        Vector2::new(-1, -1), Vector2::new(0, -1), Vector2::new(1, -1),
        Vector2::new(-1, 0), Vector2::new(1, 0),
        Vector2::new(-1, 1), Vector2::new(0, 1), Vector2::new(1, 1),
    ];
}

fn _adjacent(board: &Vec<Vec<char>>, pos: &Vector2) -> Vec<Vector2> {
    let mut adjacent = vec![];

    for v in VECTORS.iter() {
        let p = *pos + *v;
        if (0..board.len() as i64).contains(&p.1) &&
            (0..board[0].len() as i64).contains(&p.0) {
            adjacent.push(p);
        }
    }

    adjacent
}

fn _part1(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next_state = state.clone();

    for y in 0..state.len() {
        for x in 0..state[y].len() {
            match state[y][x] {
                'L' => {
                    if _adjacent(&state, &Vector2::new(x as i64, y as i64)).iter().all(|pos| state[pos.1 as usize][pos.0 as usize] != '#') {
                        next_state[y][x] = '#';
                    }
                }
                '#' => {
                    if _adjacent(&state, &Vector2::new(x as i64, y as i64)).iter().filter(|pos| state[pos.1 as usize][pos.0 as usize] == '#').count() >= 4 {
                        next_state[y][x] = 'L';
                    }
                }
                '.' => {}
                _ => panic!(),
            }
        }
    }

    next_state
}

fn _visible(board: &Vec<Vec<char>>, pos: &Vector2) -> Vec<Vector2> {
    let mut visible = vec![];

    for v in VECTORS.iter() {
        let mut p = *pos + *v;
        loop {
            if !(0..board.len() as i64).contains(&p.1) ||
                !(0..board[0].len() as i64).contains(&p.0)
            {
                break;
            }

            if board[p.1 as usize][p.0 as usize] != '.' {
                visible.push(p);
                break;
            }

            p = p + *v;
        }
    }

    visible
}

fn _part2(state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut next_state = state.clone();

    for y in 0..state.len() {
        for x in 0..state[y].len() {
            match state[y][x] {
                'L' => {
                    if _visible(&state, &Vector2::new(x as i64, y as i64)).iter().all(|pos| state[pos.1 as usize][pos.0 as usize] != '#') {
                        next_state[y][x] = '#';
                    }
                }
                '#' => {
                    if _visible(&state, &Vector2::new(x as i64, y as i64)).iter().filter(|pos| state[pos.1 as usize][pos.0 as usize] == '#').count() >= 5 {
                        next_state[y][x] = 'L';
                    }
                }
                '.' => {}
                _ => panic!(),
            }
        }
    }

    next_state
}

fn _day11(input: &str, f: &dyn Fn(&Vec<Vec<char>>) -> Vec<Vec<char>>) -> usize {
    let mut state: Vec<Vec<char>> = input.lines().map(|x| x.chars().into_iter().collect())
        .collect();

    loop {
        let next_state = f(&state);

        if next_state == state {
            return state.iter()
                .map(|x| x.iter().filter(|&&x| x == '#').count())
                .sum();
        }
        state = next_state;
    }
}

#[cfg(test)]
mod tests_day11 {
    use std::fs::File;
    use std::io::Read;

    use crate::day11::{_day11, _part1, _part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(
            37,
            _day11(
                "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL",
                &_part1,
            ),
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            26,
            _day11(
                "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL",
                &_part2,
            ),
        );
    }

    #[test]
    fn day11() {
        let mut file = File::open("data/day11.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(2299, _day11(&buffer, &_part1));
        assert_eq!(2047, _day11(&buffer, &_part2));
    }
}
