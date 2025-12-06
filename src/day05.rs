use std::{cmp, ops::RangeInclusive};
// Part 1
fn to_range(s: &str) -> RangeInclusive<i64> {
    let (a, b) = s.split_once('-').unwrap();
    let a: i64 = a.parse().unwrap();
    let b: i64 = b.parse().unwrap();
    a..=b
}

fn count_fresh(ranges: &str, ids: &str) -> usize {
    let ranges: Vec<_> = ranges.lines().map(to_range).collect();
    let ids = ids.lines().map(|id| id.trim().parse::<i64>().unwrap());
    ids.filter(|id| ranges.iter().any(|range| range.contains(id))).count()
}

// Part 2
fn range_count(r: &Option<RangeInclusive<i64>>) -> i64 {
    let Some(r) = r else {
        return 0;
    };
    r.end() - r.start() + 1
}

fn merge_ranges(r1: &RangeInclusive<i64>, r2: &RangeInclusive<i64>) -> RangeInclusive<i64> {
    let start = cmp::min(r1.start(), r2.start());
    let end = cmp::max(r1.end(), r2.end());
    *start..=*end
}

fn count_fresh_ranges(ranges: &str) -> i64 {
    let mut ranges: Vec<Option<_>> = ranges.lines().map(|l| Some(to_range(l))).collect();
    let len = ranges.len();

    let mut merged_ranges = true;
    while merged_ranges {
        merged_ranges = false;
        for i in 0..len {
            let ri = ranges[i].clone();
            if ri.is_none() {
                continue;
            }
            let ri = ri.unwrap();
            for j in i + 1..len {
                let rj = &ranges[j];
                if rj.is_none() {
                    continue;
                }
                let rj = rj.clone().unwrap();
                if ri.contains(rj.start()) || ri.contains(rj.end()) || rj.contains(ri.start()) || rj.contains(ri.end())
                {
                    let merged = merge_ranges(&ri, &rj);
                    merged_ranges = true;
                    ranges[i] = Some(merged);
                    ranges[j] = None;
                    break;
                }
            }
        }
    }
    ranges.iter().map(range_count).sum()
}

pub fn part_1(input: &str) -> i64 {
    let (ranges, ids) = input.split_once("\r\n\r\n").unwrap();
    count_fresh(ranges, ids) as i64
}
pub fn part_2(input: &str) -> i64 {
    let (ranges, _) = input.split_once("\r\n\r\n").unwrap();
    count_fresh_ranges(ranges)
}

#[cfg(test)]
mod tests {
    use crate::day05::*;
    use std::fs;
    #[test]
    fn example05_part1() {
        let input = fs::read_to_string("input/example05").unwrap();
        assert_eq!(part_1(&input), 3);
    }
    #[test]
    fn day05_part1() {
        let input = fs::read_to_string("input/day05").unwrap();
        assert_eq!(part_1(&input), 652);
    }
    #[test]
    fn example05_part2() {
        let input = fs::read_to_string("input/example05").unwrap();
        assert_eq!(part_2(&input), 14);
    }
    #[test]
    fn day05_part2() {
        let input = fs::read_to_string("input/day05").unwrap();
        assert_eq!(part_2(&input), 341753674214273);
    }
}
