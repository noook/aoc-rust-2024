advent_of_code::solution!(12);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

struct Region {
    kind: char,
    perimeter: u64,
    area: u64,
    points: Vec<(usize, usize)>,
}

fn explore_region(grid: &mut Vec<Vec<char>>, region: &mut Region, start: (i32, i32)) {
    let row = start.0 as usize;
    let col = start.1 as usize;
    
    // Check if current cell matches the region kind
    if grid[row][col] != region.kind {
        return;
    }
    
    // Mark this cell as visited and count it in the area
    grid[row][col] = '_';
    region.points.push((row, col));
    region.area += 1;

    for direction in DIRECTIONS {
        let next = (start.0 + direction.0, start.1 + direction.1);
        
        // Check bounds
        if next.0 < 0 || next.0 >= grid.len() as i32 || next.1 < 0 || next.1 >= grid[0].len() as i32 {
            region.perimeter += 1;
            continue;
        }
        
        let next_row = next.0 as usize;
        let next_col = next.1 as usize;
        let next_cell = grid[next_row][next_col];
        
        if next_cell == region.kind {
            explore_region(grid, region, next);
        } else if next_cell != '_' {
            // Different kind (not visited), adds to perimeter
            region.perimeter += 1;
        }
    }
}

fn count_sides(region: &Region) -> u64 {
    use std::collections::HashSet;
    let points: HashSet<_> = region.points.iter().cloned().collect();
    
    let mut corners = 0;
    
    for &(r, c) in &region.points {
        // Check all 4 possible corner configurations for this cell
        let patterns = [
            // Top-left corner
            [(0, 0), (-1, 0), (0, -1), (-1, -1)],
            // Top-right corner  
            [(0, 0), (-1, 0), (0, 1), (-1, 1)],
            // Bottom-left corner
            [(0, 0), (1, 0), (0, -1), (1, -1)],
            // Bottom-right corner
            [(0, 0), (1, 0), (0, 1), (1, 1)],
        ];
        
        for pattern in patterns {
            let cells: Vec<bool> = pattern.iter()
                .map(|(dr, dc)| {
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nc >= 0 {
                        points.contains(&(nr as usize, nc as usize))
                    } else {
                        false
                    }
                })
                .collect();
            
            // Corner cases:
            // 1. Outer corner: current cell is in region, but neither adjacent is
            // 2. Inner corner: current cell and both adjacent are in region, but diagonal is not
            if (cells[0] && !cells[1] && !cells[2]) ||  // Outer corner
               (cells[0] && cells[1] && cells[2] && !cells[3]) {  // Inner corner
                corners += 1;
            }
        }
    }
    
    corners
}

fn build_regions(grid: &mut Vec<Vec<char>>) -> Vec<Region> {
    let mut regions = Vec::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            let cell = grid[row][col];
            
            // Skip if already visited or is a separator
            if cell == '.' || cell == '_' {
                continue;
            }
            
            // Found a new region, explore it
            let mut region = Region {
                kind: cell,
                perimeter: 0,
                area: 0,
                points: Vec::new(),
            };
            
            explore_region(grid, &mut region, (row as i32, col as i32));
            
            for &(r, c) in &region.points {
                grid[r][c] = '.';
            }
            
            regions.push(region);
        }
    }

    regions
}

pub fn part_one(_input: &str) -> Option<u64> {
    let mut grid = parse_input(_input);
    
    let regions = build_regions(&mut grid);
    let total_price = regions.iter().map(|region| region.area * region.perimeter).sum();

    Some(total_price)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let mut grid = parse_input(_input);
    
    let regions = build_regions(&mut grid);
    let total_price = regions.iter().map(|region| region.area * count_sides(region)).sum();

    Some(total_price)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
