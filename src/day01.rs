enum Part {
    One,
    Two,
}

fn count_point_at_zero(s: &str, part: Part) -> i32 {
    let mut pos: i32 = 50;
    let mut count = 0;
    for l in s.lines() {
        let v: Vec<&str> = l.split_inclusive(['L', 'R']).collect();
        let mut rotation: i32 = v[1].parse().unwrap();
        if v[0] == "L" {
            rotation = -rotation;
        }
        let before = pos;
        pos = (pos + rotation).rem_euclid(100);
        let passes_zero = before != 0 && (pos - before).signum() != rotation.signum();
        if pos == 0 || (matches!(part, Part::Two) && passes_zero) {
            count += 1;
        }
        let full_turns = rotation.abs() / 100;
        if matches!(part, Part::Two) {
            count += full_turns;
        }
    }
    count
}

pub fn part_1(input: &str) -> i32 {
    count_point_at_zero(input, Part::One)
}
pub fn part_2(input: &str) -> i32 {
    count_point_at_zero(input, Part::Two)
}

#[cfg(test)]
mod tests {
    use crate::day01::*;
    use std::fs;
    #[test]
    fn example01_part1() {
        let input = fs::read_to_string("input/example01").unwrap();
        assert_eq!(part_1(&input), 3);
    }
    #[test]
    fn day01_part1() {
        let input = fs::read_to_string("input/day01").unwrap();
        assert_eq!(part_1(&input), 1165);
    }
    #[test]
    fn example01_part2() {
        let input = fs::read_to_string("input/example01").unwrap();
        assert_eq!(part_2(&input), 6);
    }
    #[test]
    fn day01_part2() {
        let input = fs::read_to_string("input/day01").unwrap();
        assert_eq!(part_2(&input), 6496);
    }
}
