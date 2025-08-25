advent_of_code::solution!(2);

pub fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn is_sequence_safe(levels: &[u64]) -> bool {
    if levels.len() <= 1 {
        return true;
    }

    let mut increasing = true;
    let mut decreasing = true;

    for window in levels.windows(2) {
        let diff = window[0].abs_diff(window[1]);

        if !(1..=3).contains(&diff) {
            return false;
        }

        if window[0] > window[1] {
            increasing = false;
        } else if window[0] < window[1] {
            decreasing = false;
        } else {
            return false;
        }

        if !increasing && !decreasing {
            return false;
        }
    }

    true
}

pub fn part_one(_input: &str) -> Option<u64> {
    let input = parse_input(_input);

    let result = input.iter().filter(|line| is_sequence_safe(line)).count();

    Some(result as u64)
}

fn is_safe_with_dampener(levels: &[u64]) -> bool {
    if is_sequence_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i);
        if is_sequence_safe(&modified) {
            return true;
        }
    }

    false
}

pub fn part_two(_input: &str) -> Option<u64> {
    let input = parse_input(_input);

    let result = input
        .iter()
        .filter(|line| is_safe_with_dampener(line))
        .count();

    Some(result as u64)
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
        assert_eq!(result, Some(4));
    }
}
