use std::collections::HashSet;
use std::fmt;

advent_of_code::solution!(6);

fn parse_input(input: &str) -> MapState {
    let mut map = Vec::new();
    let mut position = Position { x: 0, y: 0 };
    let direction = Direction::Up;

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            if c == '^' {
                position = Position { x, y };
                row.push('X');
            } else {
                row.push(c);
            }
        }
        map.push(row);
    }

    MapState {
        map: Map(map),
        position,
        direction,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Precomputed direction deltas for faster access - matches enum order: Up, Down, Left, Right
const DELTAS: [(i8, i8); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)]; // Up, Down, Left, Right
const TURN_RIGHT: [Direction; 4] = [
    Direction::Right,
    Direction::Left,
    Direction::Up,
    Direction::Down,
];

impl Direction {
    #[inline(always)]
    fn delta(self) -> (i8, i8) {
        DELTAS[self as usize]
    }

    #[inline(always)]
    fn turn_right(self) -> Self {
        TURN_RIGHT[self as usize]
    }

    #[inline(always)]
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone)]
struct Map(Vec<Vec<char>>);

struct MapState {
    map: Map,
    position: Position,
    direction: Direction,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            for &c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl std::ops::Deref for Map {
    type Target = Vec<Vec<char>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn patrol(state: &MapState) -> u64 {
    let mut count = 1u64;
    let mut visited = vec![vec![false; state.map[0].len()]; state.map.len()];
    visited[state.position.y][state.position.x] = true;

    let mut position = state.position;
    let mut direction = state.direction;

    let height = state.map.len();
    let width = state.map[0].len();

    loop {
        let (dx, dy) = direction.delta();
        let nx = position.x as isize + dx as isize;
        let ny = position.y as isize + dy as isize;

        if nx < 0 || ny < 0 || nx as usize >= width || ny as usize >= height {
            break;
        }

        let next_x = nx as usize;
        let next_y = ny as usize;

        if state.map[next_y][next_x] == '#' {
            direction = direction.turn_right();
            continue;
        }

        position = Position {
            x: next_x,
            y: next_y,
        };

        if !visited[position.y][position.x] {
            visited[position.y][position.x] = true;
            count += 1;
        }
    }

    count
}

// Get all positions visited during the original patrol (for filtering candidates)
fn get_patrol_path(state: &MapState) -> HashSet<Position> {
    let mut visited = HashSet::new();
    visited.insert(state.position);

    let mut position = state.position;
    let mut direction = state.direction;

    let height = state.map.len();
    let width = state.map[0].len();

    loop {
        let (dx, dy) = direction.delta();
        let nx = position.x as isize + dx as isize;
        let ny = position.y as isize + dy as isize;

        if nx < 0 || ny < 0 || nx as usize >= width || ny as usize >= height {
            break;
        }

        let next_x = nx as usize;
        let next_y = ny as usize;

        if state.map[next_y][next_x] == '#' {
            direction = direction.turn_right();
            continue;
        }

        position = Position {
            x: next_x,
            y: next_y,
        };
        visited.insert(position);
    }

    visited
}

// Check if placing an obstacle creates a loop using optimized flat array
fn creates_loop(state: &MapState, obstacle_x: usize, obstacle_y: usize) -> bool {
    let height = state.map.len();
    let width = state.map[0].len();

    // Flat array: visited[y * width * 4 + x * 4 + direction] for better cache locality
    let mut visited = vec![false; height * width * 4];

    let mut position = state.position;
    let mut direction = state.direction;
    let mut steps = 0;

    loop {
        // Early termination - if we've taken too many steps, assume loop
        steps += 1;
        if steps > height * width * 4 {
            return true;
        }

        // Calculate flat index: y * width * 4 + x * 4 + direction
        let idx = position.y * width * 4 + position.x * 4 + direction.index();

        // Check if we've seen this state before (LOOP!)
        if visited[idx] {
            return true; // Loop detected!
        }
        visited[idx] = true;

        // Calculate next position
        let (dx, dy) = direction.delta();
        let nx = position.x as isize + dx as isize;
        let ny = position.y as isize + dy as isize;

        // Check bounds - if out of bounds, no loop
        if nx < 0 || ny < 0 || nx as usize >= width || ny as usize >= height {
            return false; // Guard exits, no loop
        }

        let next_x = nx as usize;
        let next_y = ny as usize;

        // Check for obstacles (including our new one)
        if state.map[next_y][next_x] == '#' || (next_x == obstacle_x && next_y == obstacle_y) {
            direction = direction.turn_right();
        } else {
            position.x = next_x;
            position.y = next_y;
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let state = parse_input(input);
    let count = patrol(&state);
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let state = parse_input(input);
    let patrol_positions = get_patrol_path(&state);
    let mut count = 0;

    // Only test positions the guard actually visits (excluding start)
    for &pos in &patrol_positions {
        if pos == state.position {
            continue; // Skip starting position
        }

        // Check if placing obstacle here creates a loop
        if creates_loop(&state, pos.x, pos.y) {
            count += 1;
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
