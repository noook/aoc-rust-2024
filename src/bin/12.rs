advent_of_code::solution!(12);

use std::collections::HashSet;

type Position = (usize, usize);
type Grid = Vec<Vec<char>>;
type Direction = (i32, i32);

const DIRECTIONS: [Direction; 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn parse_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Debug, Clone)]
struct Region {
    #[allow(dead_code)]
    plant_type: char,
    positions: HashSet<Position>,
}

impl Region {
    fn new(plant_type: char) -> Self {
        Self {
            plant_type,
            positions: HashSet::new(),
        }
    }
    
    fn area(&self) -> usize {
        self.positions.len()
    }
    
    fn perimeter(&self) -> usize {
        self.positions
            .iter()
            .map(|&(r, c)| {
                DIRECTIONS
                    .iter()
                    .filter(|&&(dr, dc)| {
                        let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                        nr < 0 
                            || nc < 0 
                            || !self.positions.contains(&(nr as usize, nc as usize))
                    })
                    .count()
            })
            .sum()
    }
    
    fn sides(&self) -> usize {
        self.positions
            .iter()
            .map(|&pos| self.count_corners_at(pos))
            .sum()
    }
    
    fn count_corners_at(&self, (r, c): Position) -> usize {
        const CORNER_PATTERNS: [[(i32, i32); 4]; 4] = [
            [(0, 0), (-1, 0), (0, -1), (-1, -1)], // Top-left
            [(0, 0), (-1, 0), (0, 1), (-1, 1)],   // Top-right  
            [(0, 0), (1, 0), (0, -1), (1, -1)],   // Bottom-left
            [(0, 0), (1, 0), (0, 1), (1, 1)],     // Bottom-right
        ];
        
        CORNER_PATTERNS
            .iter()
            .filter(|pattern| {
                let cells: Vec<bool> = pattern
                    .iter()
                    .map(|&(dr, dc)| {
                        let (nr, nc) = (r as i32 + dr, c as i32 + dc);
                        nr >= 0 
                            && nc >= 0 
                            && self.positions.contains(&(nr as usize, nc as usize))
                    })
                    .collect();
                
                // Outer corner: current cell in region, neither adjacent is
                // Inner corner: current and both adjacent in region, diagonal is not
                (cells[0] && !cells[1] && !cells[2]) 
                    || (cells[0] && cells[1] && cells[2] && !cells[3])
            })
            .count()
    }
}

fn flood_fill(grid: &Grid, start: Position, plant_type: char) -> Region {
    let mut region = Region::new(plant_type);
    let mut stack = vec![start];
    let mut visited = HashSet::new();
    
    let (rows, cols) = (grid.len(), grid[0].len());
    
    while let Some((r, c)) = stack.pop() {
        if visited.contains(&(r, c)) || grid[r][c] != plant_type {
            continue;
        }
        
        visited.insert((r, c));
        region.positions.insert((r, c));
        
        // Add valid neighbors to stack
        for &(dr, dc) in &DIRECTIONS {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            if nr >= 0 && nc >= 0 {
                let (nr, nc) = (nr as usize, nc as usize);
                if nr < rows && nc < cols && !visited.contains(&(nr, nc)) {
                    stack.push((nr, nc));
                }
            }
        }
    }
    
    region
}

fn find_all_regions(grid: &Grid) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut visited = HashSet::new();
    
    for (r, row) in grid.iter().enumerate() {
        for (c, &plant_type) in row.iter().enumerate() {
            let pos = (r, c);
            
            if !visited.contains(&pos) {
                let region = flood_fill(grid, pos, plant_type);
                
                // Mark all positions in this region as visited
                for &position in &region.positions {
                    visited.insert(position);
                }
                
                regions.push(region);
            }
        }
    }
    
    regions
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let regions = find_all_regions(&grid);
    
    let total_price = regions
        .iter()
        .map(|region| region.area() as u64 * region.perimeter() as u64)
        .sum();
    
    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let regions = find_all_regions(&grid);
    
    let total_price = regions
        .iter()
        .map(|region| region.area() as u64 * region.sides() as u64)
        .sum();
    
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
