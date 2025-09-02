use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Bounds {
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct Grid {
    antennas: HashMap<char, Vec<Point>>,
    bounds: Bounds,
}

fn parse_input(input: &str) -> Grid {
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
    let lines: Vec<&str> = input.trim().lines().collect();

    let height = lines.len();
    let width = lines.first().map_or(0, |line| line.len());

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_default().push(Point {
                    x: x as i32,
                    y: y as i32,
                });
            }
        }
    }

    Grid {
        antennas,
        bounds: Bounds { width, height },
    }
}

/// Generate all pairs from a slice
fn pairs<T: Copy>(items: &[T]) -> impl Iterator<Item = (T, T)> + '_ {
    items
        .iter()
        .enumerate()
        .flat_map(move |(i, &a)| items[i + 1..].iter().map(move |&b| (a, b)))
}

impl Point {
    fn is_in_bounds(self, bounds: Bounds) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < bounds.width as i32 && self.y < bounds.height as i32
    }
}

/// Calculate antinodes for a pair of antennas
/// - `min_step`: `0` to include antennas, `1` to exclude them
/// - `max_steps`: `Some(n)` to limit steps, `None` for unlimited
fn calc_antinodes(
    a1: Point,
    a2: Point,
    bounds: Bounds,
    min_step: i32,
    max_steps: Option<usize>,
) -> Vec<Point> {
    let mut antinodes = Vec::new();
    let dx = a2.x - a1.x;
    let dy = a2.y - a1.y;

    // Generate antinodes in both directions
    for (start, dir) in [(a1, -1), (a2, 1)] {
        let mut i = min_step;
        loop {
            let point = Point {
                x: start.x + dir * i * dx,
                y: start.y + dir * i * dy,
            };

            if !point.is_in_bounds(bounds) {
                break;
            }

            antinodes.push(point);

            // Check if we've reached the step limit
            if let Some(limit) = max_steps {
                if i >= limit as i32 {
                    break;
                }
            }

            i += 1;
        }
    }

    antinodes
}

/// Count unique antinode positions in the grid
fn count_antinodes(grid: &Grid, min_step: i32, max_steps: Option<usize>) -> usize {
    let mut antinodes = HashSet::new();

    for antennas in grid.antennas.values() {
        for (a1, a2) in pairs(antennas) {
            for antinode in calc_antinodes(a1, a2, grid.bounds, min_step, max_steps) {
                antinodes.insert(antinode);
            }
        }
    }

    antinodes.len()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    Some(count_antinodes(&grid, 1, Some(1)) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    Some(count_antinodes(&grid, 0, None) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
