fn calc_joltage(s: &str, n: usize) -> i64 {
    let mut sum = 0;
    for l in s.lines() {
        let digits: Vec<i64> = l.chars().map(|c| c.to_digit(10).unwrap().into()).collect();
        let mut batteries = vec![];
        let mut start = 0;
        for i in 0..n {
            let stop = digits.len() - (n - i - 1);
            // using reduce instead of max_by_key because I need the first max value, not last
            let (steps, max) = digits[start..stop]
                .iter()
                .enumerate()
                .reduce(|acc, val| if acc.1 >= val.1 { acc } else { val })
                .unwrap();
            batteries.push(max);
            start += steps + 1;
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
