use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn parse_input(input: &str) -> Option<(Vec<u64>, Vec<u64>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.trim().lines() {
        let (a, b) = line.split_once("   ")?;
        left.push(a.parse().ok()?);
        right.push(b.parse().ok()?);
    }

    Some((left, right))
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut left, mut right) = parse_input(input)?;

    left.sort_unstable();
    right.sort_unstable();

    Some(
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (left, right) = parse_input(input)?;

    let counts: HashMap<u64, u64> = right.into_iter().fold(HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    Some(
        left.into_iter()
            .map(|x| x * counts.get(&x).unwrap_or(&0))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
