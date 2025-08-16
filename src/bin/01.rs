advent_of_code::solution!(1);

fn parse_input(input: &str) -> Input {
     input
        .trim()
        .split("\n")
        .map(|line| line.parse::<u64>().unwrap())
        .collect()

}

type Input = Vec<u64>;

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = parse_input(input);

    for i in 0..parsed.len() {
        for j in i + 1..parsed.len() {
            let (a, b) = (parsed[i], parsed[j]);
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = parse_input(input);

    for i in 0..parsed.len() {
        for j in i + 1..parsed.len() {
            for k in j + 1..parsed.len() {
                let (a, b, c) = (parsed[i], parsed[j], parsed[k]);
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }

    None
}

/// This is the recursive approach that creates n combinations of numbers that sum to the target
/// It's not as fast as 2 or 3 nested loops, but it's more flexible and can be used for any number of numbers
/// Benchmarks comparison:
/// nested loops [2]: 7.7µs
/// recursive approach [2]: 35.9µs
/// nested loops [3]: 106.1µs
/// recursive approach [3]: 757.7µs
#[allow(dead_code)]
fn find_combination_recursive<F>(
    numbers: &[u64],
    target: u64,
    count: usize,
    check_fn: F,
) -> Option<Vec<u64>>
where
    F: Fn(&Vec<u64>) -> u64,
{
    // Validate inputs - count must be at least 2 for the puzzle to make sense
    if count < 2 {
        return None;
    }

    let mut combination = Vec::with_capacity(count);
    find_combination_recursive_helper(numbers, target, count, &check_fn, &mut combination)
}

fn find_combination_recursive_helper<F>(
    numbers: &[u64],
    target: u64,
    remaining_count: usize,
    check_fn: &F,
    current_combination: &mut Vec<u64>,
) -> Option<Vec<u64>>
where
    F: Fn(&Vec<u64>) -> u64,
{
    // Base case: if we have found enough numbers
    if remaining_count == 0 {
        if check_fn(current_combination) == target {
            return Some(current_combination.clone());
        }
        return None;
    }

    // Try each remaining number
    for (i, &num) in numbers.iter().enumerate() {
        current_combination.push(num);
        
        // Recursively try to find the remaining numbers
        if let Some(result) = find_combination_recursive_helper(
            &numbers[i + 1..],
            target,
            remaining_count - 1,
            check_fn,
            current_combination,
        ) {
            return Some(result);
        }
        
        // Backtrack
        current_combination.pop();
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(241861950));
    }
}
