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

fn count_stones_after_blinks(stone: u64, blinks: u32, memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if blinks == 0 {
        return 1;
    }
    
    if let Some(&result) = memo.get(&(stone, blinks)) {
        return result;
    }
    
    let result = match stone {
        0 => count_stones_after_blinks(1, blinks - 1, memo),
        _ if count_digits(stone) % 2 == 0 => {
            let digits = count_digits(stone);
            let half = digits / 2;
            let divisor = 10u64.pow(half);
            let left = stone / divisor;
            let right = stone % divisor;
            count_stones_after_blinks(left, blinks - 1, memo) + 
            count_stones_after_blinks(right, blinks - 1, memo)
        },
        _ => count_stones_after_blinks(stone * 2024, blinks - 1, memo),
    };
    
    memo.insert((stone, blinks), result);
    result
}

pub fn part_one(_input: &str) -> Option<u64> {
    let input = parse_input(_input);
    let mut memo = HashMap::new();
    
    let total = input.iter()
        .map(|&stone| count_stones_after_blinks(stone, 25, &mut memo))
        .sum();
    
    Some(total)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let input = parse_input(_input);
    let mut memo = HashMap::new();
    
    let total = input.iter()
        .map(|&stone| count_stones_after_blinks(stone, 75, &mut memo))
        .sum();
    
    Some(total)
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
