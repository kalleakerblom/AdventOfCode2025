use std::ops::RangeInclusive;

fn is_repeating(n: i64) -> bool {
    let n: String = n.to_string();
    if n.len() % 2 == 0 && n[..n.len() / 2] == n[n.len() / 2..] {
        return true;
    }
    false
}

fn nr_digits(mut n: i64) -> usize {
    if n == 0 {
        return 1;
    }
    let mut res = 0;
    while n != 0 {
        n /= 10;
        res += 1;
    }
    res
}

fn is_repeating_at_least_twice(n: i64) -> bool {
    let nr_digits = nr_digits(n);
    for chunk_size in (1..=nr_digits / 2).rev() {
        if nr_digits % chunk_size == 0 {
            let mut m = n;
            let div = 10_i64.pow(chunk_size as u32);
            let first_chunk = m % div;
            loop {
                if first_chunk != m % div {
                    break; // try next chunk size
                }
                m /= div;
                if m == 0 {
                    return true;
                }
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
