use crate::util::Vector2;

fn _day03(input: &str, slopes: &Vec<Vector2>) -> usize {
    let map: Vec<Vec<bool>> = input.lines()
        .map(|line| line.chars().map(|x| x == '#').collect())
        .collect();

    let width = map[0].len();
    slopes.iter().map(|&slope| {
        let mut trees = 0;
        let mut position = Vector2::ZERO;
        while position.1 < map.len() as i64 - 1 {
            position += slope;
            trees += if map[position.1 as usize][position.0 as usize % width] { 1 } else { 0 };
        }
        trees
    }).fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests_day03 {
    use std::fs::File;
    use std::io::Read;

    use crate::day03::_day03;
    use crate::util::Vector2;

    #[test]
    fn part1_test_input() {
        assert_eq!(
            7,
            _day03("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#",
                   &vec![Vector2(3, 1)],
            ),
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            336,
            _day03("..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#",
                   &vec![Vector2(1, 1), Vector2(3, 1), Vector2(5, 1), Vector2(7, 1), Vector2(1, 2)],
            ),
        );
    }

    #[test]
    fn day03() {
        let mut file = File::open("data/day03.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(292, _day03(&buffer, &vec![Vector2(3, 1)]));
        assert_eq!(9354744432, _day03(&buffer, &vec![Vector2(1, 1), Vector2(3, 1), Vector2(5, 1), Vector2(7, 1), Vector2(1, 2)]));
    }
}
