advent_of_code::solution!(4);

const MAS: &[char] = &['M', 'A', 'S'];

type Grid = Vec<Vec<char>>;

fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn check_sequence_tail(
    grid: &Grid,
    start_row: usize,
    start_col: usize,
    direction: Direction,
    tail: &[char],
) -> bool {
    let mut pos = (start_row, start_col);
    
    for &expected_char in tail {
        // Move to next position
        pos = match direction.apply(pos.0, pos.1) {
            Some(new_pos) => new_pos,
            None => return false,
        };
        
        // Check bounds and character
        match grid.get(pos.0).and_then(|row| row.get(pos.1)) {
            Some(&ch) if ch == expected_char => continue,
            _ => return false,
        }
    }
    
    true
}

#[derive(Debug, Clone, Copy)]
struct Direction(i32, i32);

impl Direction {
    const ALL: [Direction; 8] = [
        Direction(0, 1),   // right
        Direction(1, 0),   // down
        Direction(0, -1),  // left
        Direction(-1, 0),  // up
        Direction(1, 1),   // down-right
        Direction(-1, -1), // up-left
        Direction(1, -1),  // down-left
        Direction(-1, 1),  // up-right
    ];
    
    // Diagonal directions for X-MAS pattern
    const UP_LEFT: Direction = Direction(-1, -1);
    const UP_RIGHT: Direction = Direction(-1, 1);
    const DOWN_LEFT: Direction = Direction(1, -1);
    const DOWN_RIGHT: Direction = Direction(1, 1);
    
    fn apply(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        let new_row = row as i32 + self.0;
        let new_col = col as i32 + self.1;
        
        if new_row >= 0 && new_col >= 0 {
            Some((new_row as usize, new_col as usize))
        } else {
            None
        }
    }
    
    fn get_char_at(&self, grid: &Grid, row: usize, col: usize) -> Option<char> {
        let (new_row, new_col) = self.apply(row, col)?;
        grid.get(new_row)?.get(new_col).copied()
    }
}

fn check_x_mas(grid: &Grid, row: usize, col: usize) -> bool {
    if grid[row][col] != 'A' {
        return false;
    }
    
    // Check both diagonals for MAS pattern (M and S can be in either direction)
    let slash_diagonal = (
        Direction::UP_RIGHT.get_char_at(grid, row, col),
        Direction::DOWN_LEFT.get_char_at(grid, row, col),
    );
    
    let backslash_diagonal = (
        Direction::UP_LEFT.get_char_at(grid, row, col),
        Direction::DOWN_RIGHT.get_char_at(grid, row, col),
    );
    
    matches!(slash_diagonal, (Some('M'), Some('S')) | (Some('S'), Some('M')))
        && matches!(backslash_diagonal, (Some('M'), Some('S')) | (Some('S'), Some('M')))
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut count = 0;

    for (row, row_data) in grid.iter().enumerate() {
        for (col, &ch) in row_data.iter().enumerate() {
            if ch == 'X' {
                count += Direction::ALL
                    .iter()
                    .filter(|&&dir| check_sequence_tail(&grid, row, col, dir, MAS))
                    .count();
            }
        }
    }

    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    
    let count = grid
        .iter()
        .enumerate()
        .flat_map(|(row, row_data)| {
            row_data
                .iter()
                .enumerate()
                .map(move |(col, _)| (row, col))
        })
        .filter(|&(row, col)| check_x_mas(&grid, row, col))
        .count() as u64;

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
