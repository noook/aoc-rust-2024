use std::collections::HashSet;

advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

type Direction = (i32, i32);

const DIRECTIONS: [Direction; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn find_trail_count(input: &[Vec<u8>], row: usize, col: usize) -> u64 {
    let current_value = input[row][col];

    if current_value == 9 {
        return 1;
    }

    let mut trail_count = 0;

    for direction in DIRECTIONS {
        let next_row = row as i32 + direction.0;
        let next_col = col as i32 + direction.1;

        // Check bounds and if the next position has the correct value (current + 1)
        if next_row >= 0
            && next_row < input.len() as i32
            && next_col >= 0
            && next_col < input[0].len() as i32
            && input[next_row as usize][next_col as usize] == current_value + 1
        {
            // Recursively count all trails from the next position
            trail_count += find_trail_count(input, next_row as usize, next_col as usize);
        }
    }

    trail_count
}

fn find_trail(
    input: &[Vec<u8>],
    row: usize,
    col: usize,
    start: u8,
    end: u8,
    increase: i8,
) -> HashSet<(usize, usize)> {
    let current_value = input[row][col];

    if current_value == end {
        let mut set = HashSet::new();
        set.insert((row, col));
        return set;
    }

    let mut reachable_ends = HashSet::new();

    for direction in DIRECTIONS {
        let next_row = row as i32 + direction.0;
        let next_col = col as i32 + direction.1;

        // Check bounds and if the next position has the correct value
        if next_row >= 0
            && next_row < input.len() as i32
            && next_col >= 0
            && next_col < input[0].len() as i32
            && input[next_row as usize][next_col as usize] as i8 == current_value as i8 + increase
        {
            let ends_from_next = find_trail(
                input,
                next_row as usize,
                next_col as usize,
                start,
                end,
                increase,
            );

            reachable_ends.extend(ends_from_next);
        }
    }

    reachable_ends
}

pub fn part_one(_input: &str) -> Option<u64> {
    let input = parse_input(_input);
    let mut result = 0;

    for (row, line) in input.iter().enumerate() {
        for (col, &value) in line.iter().enumerate() {
            if value == 0 {
                let reachable_nines = find_trail(&input, row, col, 0, 9, 1);
                result += reachable_nines.len();
            }
        }
    }
    Some(result as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let input = parse_input(_input);
    let mut result = 0;

    for (row, line) in input.iter().enumerate() {
        for (col, &value) in line.iter().enumerate() {
            if value == 0 {
                let trail_count = find_trail_count(&input, row, col);
                result += trail_count;
            }
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
