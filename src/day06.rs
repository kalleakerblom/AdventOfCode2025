use std::fs::read;

fn read_columns(s: &str) -> Vec<Vec<&str>> {
    let mut lines = s.lines();
    let first_line = lines.next().unwrap();
    let mut columns: Vec<_> = first_line.split_whitespace().map(|split| vec![split]).collect();
    for line in lines {
        for (i, split) in line.split_whitespace().enumerate() {
            columns[i].push(split);
        }
    }
    columns
}

fn calc_column(col: &[&str]) -> i64 {
    let op = col.last().unwrap();
    let numbers = col[..col.len() - 1].iter().map(|s| s.parse::<i64>().unwrap());
    match *op {
        "+" => numbers.sum(),
        "*" => numbers.product(),
        _ => panic!(),
    }
}

fn read_columns_part2(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

fn calc_columns_part2(row_data: &[Vec<char>]) -> i64 {
    let mut numbers: Vec<i64> = vec![];
    let mut result: i64 = 0;
    let width = row_data[0].len();
    let height = row_data.len();
    for col in (0..width).rev() {
        let mut n = String::new();
        for row in 0..height - 1 {
            let ch = row_data[row][col];
            if ch.is_numeric() {
                n.push(ch);
            }
        }
        if n.is_empty() {
            continue;
        }
        let n: i64 = n.parse().unwrap();
        numbers.push(n);
        let last_in_col = row_data[height - 1][col];
        if last_in_col == '+' {
            result += numbers.iter().sum::<i64>();
            numbers.clear();
        }
        if last_in_col == '*' {
            result += numbers.iter().product::<i64>();
            numbers.clear();
        }
    }
    result
}

pub fn part_1(input: &str) -> i64 {
    let col_data = read_columns(input);
    col_data.iter().map(|c| calc_column(c)).sum()
}
pub fn part_2(input: &str) -> i64 {
    let col_data = read_columns_part2(input);
    calc_columns_part2(&col_data)
}

#[cfg(test)]
mod tests {
    use crate::day06::*;
    use std::fs;
    #[test]
    fn example06_part1() {
        let input = fs::read_to_string("input/example06").unwrap();
        assert_eq!(part_1(&input), 4277556);
    }
    #[test]
    fn day06_part1() {
        let input = fs::read_to_string("input/day06").unwrap();
        assert_eq!(part_1(&input), 6299564383938);
    }
    #[test]
    fn example06_part2() {
        let input = fs::read_to_string("input/example06").unwrap();
        assert_eq!(part_2(&input), 3263827);
    }
    #[test]
    fn day06_part2() {
        let input = fs::read_to_string("input/day06").unwrap();
        assert_eq!(part_2(&input), 11950004808442);
    }
}
