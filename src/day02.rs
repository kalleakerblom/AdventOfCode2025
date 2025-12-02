use std::ops::RangeInclusive;

fn is_repeating(n: i64) -> bool {
    let n: String = n.to_string();
    if n.len() % 2 == 0 && n[..n.len() / 2] == n[n.len() / 2..] {
        return true;
    }
    false
}

fn is_repeating_at_least_twice(n: i64) -> bool {
    let n: Vec<char> = n.to_string().chars().collect();
    let len = n.len();
    for i in 2..=len {
        if len % i == 0 {
            let mut chunks = n.chunks_exact(len / i);
            let first = chunks.next().unwrap();
            if chunks.all(|c| c == first) {
                return true;
            }
        }
    }
    false
}

fn to_range(s: &str) -> RangeInclusive<i64> {
    let (a, b) = s.split_once('-').unwrap();
    let a: i64 = a.parse().unwrap();
    let b: i64 = b.parse().unwrap();
    a..=b
}

pub fn part_1(input: &str) -> i64 {
    input.split(',').flat_map(to_range).filter(|n| is_repeating(*n)).sum()
}
pub fn part_2(input: &str) -> i64 {
    input
        .split(',')
        .flat_map(to_range)
        .filter(|n| is_repeating_at_least_twice(*n))
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day02::*;
    use std::fs;
    #[test]
    fn example02_part1() {
        let input = fs::read_to_string("input/example02").unwrap();
        assert_eq!(part_1(&input), 1227775554);
    }
    #[test]
    fn day02_part1() {
        let input = fs::read_to_string("input/day02").unwrap();
        assert_eq!(part_1(&input), 55916882972);
    }
    #[test]
    fn example02_part2() {
        let input = fs::read_to_string("input/example02").unwrap();
        assert_eq!(part_2(&input), 4174379265);
    }
    #[test]
    fn day02_part2() {
        let input = fs::read_to_string("input/day02").unwrap();
        assert_eq!(part_2(&input), 76169125915);
    }
}
