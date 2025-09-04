advent_of_code::solution!(11);

use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn count_digits(mut n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

/// Split a number with even digits into left and right halves
fn split_number(n: u64, digits: u32) -> (u64, u64) {
    let half = digits / 2;
    let divisor = 10u64.pow(half);
    (n / divisor, n % divisor)
}

fn count_stones_after_blinks(stone: u64, blinks: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    
    if let Some(&result) = memo.get(&(stone, blinks)) {
        return result;
    }
    
    let result = match stone {
        0 => count_stones_after_blinks(1, blinks - 1, memo),
        _ => {
            let digits = count_digits(stone);
            if digits % 2 == 0 {
                let (left, right) = split_number(stone, digits);
                count_stones_after_blinks(left, blinks - 1, memo) + 
                count_stones_after_blinks(right, blinks - 1, memo)
            } else {
                count_stones_after_blinks(stone * 2024, blinks - 1, memo)
            }
        }
    };
    
    memo.insert((stone, blinks), result);
    result
}

fn solve_with_blinks(input: &str, blinks: u32) -> u64 {
    let stones = parse_input(input);
    let mut memo = HashMap::new();
    
    stones.iter()
        .map(|&stone| count_stones_after_blinks(stone, blinks, &mut memo))
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve_with_blinks(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve_with_blinks(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
