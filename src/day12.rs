use crate::util::Vector2;

fn _rotate(v: &Vector2, degrees: i64) -> Vector2 {
    Vector2::new(
        v.0 * (degrees as f64 * std::f64::consts::PI / 180_f64).cos() as i64 - v.1 * (degrees as f64 * std::f64::consts::PI / 180_f64).sin() as i64,
        v.0 * (degrees as f64 * std::f64::consts::PI / 180_f64).sin() as i64 + v.1 * (degrees as f64 * std::f64::consts::PI / 180_f64).cos() as i64,
    )
}

fn _part1(commands: &Vec<&str>) -> Vector2 {
    let mut position = Vector2::new(0, 0);
    let mut heading = Vector2::new(1, 0);
    for command in commands {
        let action = command.get(0..1).unwrap();
        let v: i64 = command.get(1..command.len()).unwrap().parse().unwrap();
        match action {
            "N" => position += Vector2(0, v),
            "S" => position -= Vector2(0, v),
            "E" => position += Vector2(v, 0),
            "W" => position -= Vector2(v, 0),
            "F" => position += heading * v,
            "R" => heading = _rotate(&heading, -v),
            "L" => heading = _rotate(&heading, v),
            _ => panic!(),
        }
    }
    position
}

fn _part2(commands: &Vec<&str>) -> Vector2 {
    let mut position = Vector2::new(0, 0);
    let mut waypoint = Vector2(10, 1);
    for command in commands {
        let action = command.get(0..1).unwrap();
        let v: i64 = command.get(1..command.len()).unwrap().parse().unwrap();
        match action {
            "N" => waypoint += Vector2(0, v),
            "S" => waypoint -= Vector2(0, v),
            "E" => waypoint += Vector2(v, 0),
            "W" => waypoint -= Vector2(v, 0),
            "F" => position += waypoint * v,
            "R" => waypoint = _rotate(&waypoint, -v),
            "L" => waypoint = _rotate(&waypoint, v),
            _ => panic!(),
        }
    }
    position
}

fn _day12(input: &str, f: &dyn Fn(&Vec<&str>) -> Vector2) -> i64 {
    let commands: Vec<&str> = input.lines().collect();

    let end_position = f(&commands);

    end_position.manhattan_distance(&Vector2::ZERO)
}

#[cfg(test)]
mod tests_day12 {
    use std::fs::File;
    use std::io::Read;

    use crate::day12::{_day12, _part1, _part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(25, _day12("F10\nN3\nF7\nR90\nF11", &_part1));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(286, _day12("F10\nN3\nF7\nR90\nF11", &_part2));
    }

    #[test]
    fn day12() {
        let mut file = File::open("data/day12.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(879, _day12(&buffer, &_part1));
        assert_eq!(18107, _day12(&buffer, &_part2));
    }
}
