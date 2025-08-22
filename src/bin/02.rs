use core::fmt;
use std::fmt::Debug;

use regex::Regex;

advent_of_code::solution!(2);

struct PasswordLine {
    min: usize,
    max: usize,
    char: char,
    password: String,
}

impl Debug for PasswordLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\nPasswordLine {{ min: {}, max: {}, char: {}, password: {} }}",
            self.min, self.max, self.char, self.password
        )
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = PasswordLine> + '_ {
    let re =
        Regex::new(r"(?P<min>\d+)-(?P<max>\d+)\s(?P<char>[a-z]):\s(?P<password>[a-z]+)").unwrap();
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(move |line| {
            let caps = re.captures(line).unwrap();
            PasswordLine {
                min: caps["min"].parse().unwrap(),
                max: caps["max"].parse().unwrap(),
                char: caps["char"].parse().unwrap(),
                password: caps["password"].to_string(),
            }
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = parse_input(input);

    let count = lines
        .filter(|line| {
            let char_count: usize = line.password.chars().filter(|c| *c == line.char).count();
            (line.min..=line.max).contains(&char_count)
        })
        .count() as u64;

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = parse_input(input);
    let count = lines
        .filter(|line| {
            let chars: Vec<char> = line.password.chars().collect();
            let a = chars.get(line.min - 1);
            let b = chars.get(line.max - 1);
            match (a, b) {
                (Some(&c1), Some(&c2)) => (c1 == line.char) ^ (c2 == line.char),
                _ => false,
            }
        })
        .count() as u64;

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
