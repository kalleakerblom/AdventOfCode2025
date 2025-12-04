#[derive(Clone, Copy)]
struct Pos(i32, i32);

fn get_neighbors(p: Pos) -> [Pos; 8] {
    let Pos(x, y) = p;
    [
        Pos(x - 1, y - 1),
        Pos(x - 1, y),
        Pos(x - 1, y + 1),
        Pos(x, y - 1),
        Pos(x, y + 1),
        Pos(x + 1, y - 1),
        Pos(x + 1, y),
        Pos(x + 1, y + 1),
    ]
}

struct Map {
    rolls: Vec<Vec<bool>>,
    width: i32,
    height: i32,
}

impl Map {
    fn parse(s: &str) -> Self {
        let rolls: Vec<Vec<bool>> = s.lines().map(|l| l.chars().map(|ch| ch == '@').collect()).collect();
        let width = rolls[0].len() as i32;
        let height = rolls.len() as i32;
        Self { rolls, width, height }
    }
    fn is_roll_at(&self, p: Pos) -> bool {
        if p.0 < 0 || self.width <= p.0 || p.1 < 0 || self.height <= p.1 {
            return false;
        }
        self.rolls[p.1 as usize][p.0 as usize]
    }
    fn remove_roll(&mut self, p: Pos) {
        self.rolls[p.1 as usize][p.0 as usize] = false;
    }
}

enum Part {
    One,
    Two,
}

fn remove_rolls(s: &str, part: Part) -> i64 {
    let mut map = Map::parse(s);
    let mut count = 0;
    // for part 1 we always break loop early
    loop {
        let mut roll_removed = false;
        for y in 0..map.height {
            for x in 0..map.width {
                let pos = Pos(x, y);
                if map.is_roll_at(pos) {
                    let neighbor_rolls = get_neighbors(pos).iter().filter(|p| map.is_roll_at(**p)).count();
                    if neighbor_rolls < 4 {
                        count += 1;
                        if matches!(part, Part::Two) {
                            map.remove_roll(pos);
                            roll_removed = true;
                        }
                    }
                }
            }
        }
        if !roll_removed {
            break;
        }
    }
    count
}

pub fn part_1(input: &str) -> i64 {
    remove_rolls(input, Part::One)
}
pub fn part_2(input: &str) -> i64 {
    remove_rolls(input, Part::Two)
}

#[cfg(test)]
mod tests {
    use crate::day04::*;
    use std::fs;
    #[test]
    fn example04_part1() {
        let input = fs::read_to_string("input/example04").unwrap();
        assert_eq!(part_1(&input), 13);
    }
    #[test]
    fn day04_part1() {
        let input = fs::read_to_string("input/day04").unwrap();
        assert_eq!(part_1(&input), 1389);
    }
    #[test]
    fn example04_part2() {
        let input = fs::read_to_string("input/example04").unwrap();
        assert_eq!(part_2(&input), 43);
    }
    #[test]
    fn day04_part2() {
        let input = fs::read_to_string("input/day04").unwrap();
        assert_eq!(part_2(&input), 9000);
    }
}
