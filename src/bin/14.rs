advent_of_code::solution!(14);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

#[derive(Clone, Copy)]
struct MapConfig {
    width: u64,
    height: u64,
}

// Configuration: large map by default, small map for tests
fn get_map_config() -> MapConfig {
    #[cfg(test)]
    {
        // Test configuration - small map for examples
        MapConfig { width: 11, height: 7 }
    }
    
    #[cfg(not(test))]
    {
        // Default production configuration - large map for real puzzle
        MapConfig { width: 101, height: 103 }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (position_str, velocity_str) = line.split_once(' ').unwrap();
            let position = position_str.strip_prefix("p=").unwrap().split_once(',').unwrap();
            let velocity = velocity_str.strip_prefix("v=").unwrap().split_once(',').unwrap();

            Robot {
                position: Coord {
                    x: position.0.parse().unwrap(),
                    y: position.1.parse().unwrap(),
                },
                velocity: Coord {
                    x: velocity.0.parse().unwrap(),
                    y: velocity.1.parse().unwrap(),
                },
            }
        })
        .collect()
}

fn safe_index(start: i64, velocity: i64, time: u64, bound: u64) -> i64 {
    let bound = bound as i64;
    let time = time as i64;
    ((start + velocity * time) % bound + bound) % bound
}

fn predict_position(robot: &Robot, time: u64, config: &MapConfig) -> Coord {
    Coord {
        x: safe_index(robot.position.x, robot.velocity.x, time, config.width),
        y: safe_index(robot.position.y, robot.velocity.y, time, config.height),
    }
}

fn quandrant_predict_position(coords: &Vec<Coord>, config: &MapConfig) -> [u64; 4] {
    let mut quadrants: [u64; 4] = [0, 0, 0, 0];
    let width = config.width as i64;
    let height = config.height as i64;
    
    // Middle lines - robots on these don't count
    let middle_x = width / 2;
    let middle_y = height / 2;

    for coord in coords {
        // Skip robots that are exactly in the middle (horizontally or vertically)
        if coord.x == middle_x || coord.y == middle_y {
            continue;
        }
        
        let quadrant = match (coord.x, coord.y) {
            (x, y) if x < middle_x && y < middle_y => 0, // Top-left
            (x, y) if x > middle_x && y < middle_y => 1, // Top-right
            (x, y) if x < middle_x && y > middle_y => 2, // Bottom-left
            (x, y) if x > middle_x && y > middle_y => 3, // Bottom-right
            _ => panic!("Invalid coordinate: {:?}", coord),
        };
        quadrants[quadrant] += 1;
    }
    quadrants
}

fn count_unique_positions(coords: &[Coord]) -> usize {
    use std::collections::HashSet;
    
    let mut unique_positions = HashSet::new();
    for coord in coords {
        unique_positions.insert((coord.x, coord.y));
    }
    unique_positions.len()
}

/// Print a visual representation of the robot positions on the map
#[allow(dead_code)]
fn print_map(coords: &[Coord], config: &MapConfig) {
    use std::collections::HashMap;
    
    // Count robots at each position
    let mut counts: HashMap<(i64, i64), usize> = HashMap::new();
    for coord in coords {
        let pos = (coord.x, coord.y);
        *counts.entry(pos).or_insert(0) += 1;
    }
    
    // Print the grid
    for y in 0..config.height {
        for x in 0..config.width {
            let count = counts.get(&(x as i64, y as i64));
            if let Some(&n) = count {
                if n > 9 {
                    print!("*"); // More than 9 robots
                } else {
                    print!("{}", n);
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    let config = get_map_config();
    let robots = parse_input(input);

    // Simulate for 100 seconds
    let positions: Vec<Coord> = robots
        .iter()
        .map(|robot| predict_position(robot, 100, &config))
        .collect();

    let quadrant_counts = quandrant_predict_position(&positions, &config);
    
    // Calculate safety factor (product of quadrant counts)
    Some(quadrant_counts.iter().product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let config = get_map_config();
    let robots = parse_input(input);

    // Find the first time when all robots are in unique positions (Christmas tree pattern)
    for time in 1.. {
        let positions: Vec<Coord> = robots
            .iter()
            .map(|robot| predict_position(robot, time, &config))
            .collect();

        // Check if all robots are in unique positions (no overlaps)
        if count_unique_positions(&positions) == robots.len() {
            return Some(time);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert!(result.is_some());
    }

    #[test]
    fn test_predict_position() {
        let robot = Robot {
            position: Coord { x: 2, y: 4 },
            velocity: Coord { x: 2, y: -3 },
        };
        let config = MapConfig { width: 11, height: 7 };
        let position = predict_position(&robot, 5, &config);
        assert_eq!(position, Coord { x: 1, y: 3 });
    }

    #[test]
    fn test_safe_index() {
        assert_eq!(safe_index(5, 3, 1, 11), 8);
        assert_eq!(safe_index(5, -3, 1, 11), 2);
        assert_eq!(safe_index(0, -1, 1, 11), 10);
        assert_eq!(safe_index(10, 1, 1, 11), 0);
    }

    #[test]
    fn test_count_unique_positions() {
        let coords = vec![
            Coord { x: 1, y: 1 },
            Coord { x: 1, y: 1 }, // duplicate
            Coord { x: 2, y: 2 },
        ];
        assert_eq!(count_unique_positions(&coords), 2);
    }
}
