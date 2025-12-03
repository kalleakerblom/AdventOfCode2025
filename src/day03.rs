fn calc_joltage(s: &str, n: usize) -> i64 {
    let mut sum = 0;
    for l in s.lines() {
        let digits: Vec<i64> = l.chars().map(|c| c.to_digit(10).unwrap().into()).collect();
        let len = digits.len();
        let mut batteries = vec![];
        let mut rem = 0;
        for i in 0..n {
            let to_leave = n - i - 1;
            // using fold instead of max_by_key because I need the first max value, not last
            let (steps, max) = digits[rem..len - to_leave]
                .iter()
                .enumerate()
                .fold((0, digits[rem]), |acc, (i, d)| if *d > acc.1 { (i, *d) } else { acc });
            batteries.push(max);
            rem += steps + 1;
        }
        let joltage = batteries
            .iter()
            .rev()
            .enumerate()
            .map(|(i, n)| *n * 10_i64.pow(i as u32))
            .sum::<i64>();
        sum += joltage;
    }
    sum
}

pub fn part_1(input: &str) -> i64 {
    calc_joltage(input, 2)
}

pub fn part_2(input: &str) -> i64 {
    calc_joltage(input, 12)
}

#[cfg(test)]
mod tests {
    use crate::day03::*;
    use std::fs;
    #[test]
    fn example03_part1() {
        let input = fs::read_to_string("input/example03").unwrap();
        assert_eq!(part_1(&input), 357);
    }
    #[test]
    fn day03_part1() {
        let input = fs::read_to_string("input/day03").unwrap();
        assert_eq!(part_1(&input), 17158);
    }
    #[test]
    fn example03_part2() {
        let input = fs::read_to_string("input/example03").unwrap();
        assert_eq!(part_2(&input), 3121910778619);
    }
    #[test]
    fn day03_part2() {
        let input = fs::read_to_string("input/day03").unwrap();
        assert_eq!(part_2(&input), 170449335646486);
    }
}
