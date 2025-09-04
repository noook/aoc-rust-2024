use std::collections::HashSet;

advent_of_code::solution!(10);

type Grid = Vec<Vec<u8>>;
type Position = (usize, usize);

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const PEAK_HEIGHT: u8 = 9;
const TRAILHEAD_HEIGHT: u8 = 0;

fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit") as u8)
                .collect()
        })
        .collect()
}

/// Check if a position is within grid bounds
#[inline]
fn is_valid_position(grid: &Grid, row: isize, col: isize) -> bool {
    row >= 0 && col >= 0 && (row as usize) < grid.len() && (col as usize) < grid[0].len()
}

/// Get valid neighbors with the expected height
fn get_valid_neighbors(grid: &Grid, pos: Position, expected_height: u8) -> impl Iterator<Item = Position> + '_ {
    let (row, col) = pos;
    
    DIRECTIONS
        .iter()
        .filter_map(move |&(dr, dc)| {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;
            
            if is_valid_position(grid, new_row, new_col) {
                let new_pos = (new_row as usize, new_col as usize);
                if grid[new_pos.0][new_pos.1] == expected_height {
                    Some(new_pos)
                } else {
                    None
                }
            } else {
                None
            }
        })
}

/// Count all distinct trails from a position to height 9
fn count_trails(grid: &Grid, pos: Position) -> u64 {
    let current_height = grid[pos.0][pos.1];
    
    if current_height == PEAK_HEIGHT {
        return 1;
    }
    
    // Sum trails from all valid next positions
    get_valid_neighbors(grid, pos, current_height + 1)
        .map(|next_pos| count_trails(grid, next_pos))
        .sum()
}

/// Find all reachable positions with height 9 from a starting position
fn find_reachable_peaks(grid: &Grid, pos: Position) -> HashSet<Position> {
    let current_height = grid[pos.0][pos.1];
    
    if current_height == PEAK_HEIGHT {
        return HashSet::from([pos]);
    }
    
    // Collect all reachable peaks from valid next positions
    get_valid_neighbors(grid, pos, current_height + 1)
        .flat_map(|next_pos| find_reachable_peaks(grid, next_pos))
        .collect()
}

/// Find all trailheads (positions with height 0) in the grid (optimized version)
fn find_trailheads(grid: &Grid) -> impl Iterator<Item = Position> + '_ {
    grid.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(col, &height)| {
                    (height == TRAILHEAD_HEIGHT).then_some((row, col))
                })
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    
    let total_score = find_trailheads(&grid)
        .map(|trailhead| find_reachable_peaks(&grid, trailhead).len() as u64)
        .sum();
    
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    
    let total_rating = find_trailheads(&grid)
        .map(|trailhead| count_trails(&grid, trailhead))
        .sum();
    
    Some(total_rating)
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
