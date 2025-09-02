use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, Clone)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct MapBounds {
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct ParsedInput {
    coordinates: HashMap<char, Vec<Coordinates>>,
    bounds: MapBounds,
}

fn parse_input(input: &str) -> ParsedInput {
    let mut coordinates = HashMap::<char, Vec<Coordinates>>::new();
    let lines: Vec<&str> = input.trim().lines().collect();
    
    let height = lines.len();
    let width = lines.get(0).map_or(0, |line| line.len());
    
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                coordinates.entry(c).or_insert_with(Vec::new).push(Coordinates { x, y });
            }
        }
    }

    ParsedInput {
        coordinates,
        bounds: MapBounds { width, height },
    }
}

/// Returns all combinations of `r` elements from the input array
/// If r > array.len(), returns an empty vector
fn combinations<T: Clone>(array: &[T], r: usize) -> Vec<Vec<T>> {
    if r == 0 {
        return vec![vec![]];
    }
    
    if r > array.len() {
        return vec![];
    }
    
    if r == array.len() {
        return vec![array.to_vec()];
    }
    
    let mut result = Vec::new();
    
    // Generate combinations recursively
    for i in 0..=array.len() - r {
        let first = &array[i];
        let remaining = &array[i + 1..];
        
        for mut combo in combinations(remaining, r - 1) {
            combo.insert(0, first.clone());
            result.push(combo);
        }
    }
    
    result
}

fn calc_antinodes(a1: Coordinates, a2: Coordinates) -> (Coordinates, Coordinates) {
    let (x1, y1, x2, y2) = (a1.x as i32, a1.y as i32, a2.x as i32, a2.y as i32);
    let (dx, dy) = (x2 - x1, y2 - y1);
    (
        Coordinates { x: (x1 - dx) as usize, y: (y1 - dy) as usize },
        Coordinates { x: (x2 + dx) as usize, y: (y2 + dy) as usize },
    )
}

fn calc_all_antinodes(a1: Coordinates, a2: Coordinates, bounds: &MapBounds) -> Vec<Coordinates> {
    let mut antinodes = Vec::new();
    let (x1, y1, x2, y2) = (a1.x as i32, a1.y as i32, a2.x as i32, a2.y as i32);
    let (dx, dy) = (x2 - x1, y2 - y1);
    
    // Generate antinodes in the negative direction (from a1 backwards)
    let mut i = 1;
    loop {
        let x = x1 - i * dx;
        let y = y1 - i * dy;
        
        if x < 0 || y < 0 || x >= bounds.width as i32 || y >= bounds.height as i32 {
            break;
        }
        
        antinodes.push(Coordinates { x: x as usize, y: y as usize });
        i += 1;
    }
    
    // Generate antinodes in the positive direction (from a2 forwards)
    let mut i = 1;
    loop {
        let x = x2 + i * dx;
        let y = y2 + i * dy;
        
        if x < 0 || y < 0 || x >= bounds.width as i32 || y >= bounds.height as i32 {
            break;
        }
        
        antinodes.push(Coordinates { x: x as usize, y: y as usize });
        i += 1;
    }
    
    antinodes
}

fn is_in_bounds(coordinates: &Coordinates, bounds: &MapBounds) -> bool {
    coordinates.x < bounds.width && coordinates.y < bounds.height
}

pub fn part_one(_input: &str) -> Option<u64> {
    let parsed = parse_input(_input);
    let mut visited = vec![vec![false; parsed.bounds.width]; parsed.bounds.height];
    let mut count = 0;
    
    for (_c, coordinates) in parsed.coordinates.iter() {
        let combinations = combinations(coordinates, 2);

        for combination in combinations {
            let (a1, a2) = calc_antinodes(combination[0].clone(), combination[1].clone());
            if is_in_bounds(&a1, &parsed.bounds) && !visited[a1.y][a1.x] {
                visited[a1.y][a1.x] = true;
                count += 1;
            }
            if is_in_bounds(&a2, &parsed.bounds) && !visited[a2.y][a2.x] {
                visited[a2.y][a2.x] = true;
                count += 1;
            }
        }
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let parsed = parse_input(_input);
    let mut visited = vec![vec![false; parsed.bounds.width]; parsed.bounds.height];
    let mut count = 0;
    
    for (_c, coordinates) in parsed.coordinates.iter() {
        let combinations = combinations(coordinates, 2);

        for combination in combinations {
            // Mark the original antenna positions as antinodes
            if !visited[combination[0].y][combination[0].x] {
                visited[combination[0].y][combination[0].x] = true;
                count += 1;
            }
            if !visited[combination[1].y][combination[1].x] {
                visited[combination[1].y][combination[1].x] = true;
                count += 1;
            }

            // Calculate all antinodes in both directions until out of bounds
            let all_antinodes = calc_all_antinodes(combination[0].clone(), combination[1].clone(), &parsed.bounds);
            for antinode in all_antinodes {
                if !visited[antinode.y][antinode.x] {
                    visited[antinode.y][antinode.x] = true;
                    count += 1;
                }
            }
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
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
