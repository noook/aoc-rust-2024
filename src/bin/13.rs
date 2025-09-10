//! # Day 13: Claw Contraption
//! 
//! ## Problem Summary
//! This puzzle involves claw machines with two buttons (A and B) that move the claw by fixed vectors.
//! The goal is to find the minimum token cost to position the claw exactly on each prize.
//! 
//! ## Mathematical Approach
//! This problem required advanced mathematical concepts that were beyond my initial knowledge:
//! 
//! ### Linear Algebra (Cramer's Rule)
//! - For non-colinear moves (det ≠ 0): Use Cramer's rule to solve the 2×2 linear system
//! - Formula: det = ax × by - ay × bx
//! - Solutions: nA = (px × by - py × bx) ÷ det, nB = (ax × py - ay × px) ÷ det
//! 
//! ### Number Theory (Extended GCD)
//! - For colinear moves (det = 0): Reduce to 1D and use Extended GCD
//! - Find one solution to ax × nA + bx × nB = px
//! - Parameterize all solutions and find the minimum cost
//! 
//! ## Learning Journey
//! - **Initial approach**: Attempted brute force and basic linear algebra
//! - **Challenge**: The mathematical theory required (determinants, GCD, linear Diophantine equations) 
//!   was beyond my high school math background
//! - **Collaboration**: Worked with AI assistant to understand the mathematical foundations
//! - **Key insights learned**:
//!   - How determinants determine solvability of linear systems
//!   - Extended GCD algorithm for integer solutions
//!   - Parameterization of solutions in linear Diophantine equations
//! 
//! ## Implementation Notes
//! - Part 1: Limited to 100 button presses per machine
//! - Part 2: Unlimited presses, with 10^13 offset added to prize coordinates
//! - Uses i128 for large number arithmetic to prevent overflow
//! 
//! ## Personal Reflection
//! This puzzle was a great learning experience in advanced mathematics. While I couldn't solve it
//! independently due to my limited math education, the collaborative problem-solving process
//! helped me understand concepts like linear algebra, number theory, and algorithmic thinking
//! that I hadn't encountered before.

advent_of_code::solution!(13);

#[derive(Debug, PartialEq)]
struct Coord {
    x: i128,
    y: i128,
}

#[derive(Debug)]
struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

/// Parsing error types
#[derive(Debug)]
enum ParseError {
    InvalidCoord(String),
    InvalidMachine(String),
    InvalidNumber(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidCoord(msg) => write!(f, "Invalid coordinate format: {}", msg),
            ParseError::InvalidMachine(msg) => write!(f, "Invalid machine format: {}", msg),
            ParseError::InvalidNumber(msg) => write!(f, "Invalid number: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parses a coordinate string like "X+94, Y+34" or "X=8400, Y=5400"
fn parse_coord(s: &str, sep: char, delim: char) -> Result<Coord, ParseError> {
    let (x_part, y_part) = s.split_once(delim)
        .ok_or_else(|| ParseError::InvalidCoord(format!("Missing delimiter '{}' in: {}", delim, s)))?;
    
    let x = x_part.split_once(sep)
        .ok_or_else(|| ParseError::InvalidCoord(format!("Missing separator '{}' in X part: {}", sep, x_part)))?
        .1
        .parse::<i128>()
        .map_err(|e| ParseError::InvalidNumber(format!("Invalid X coordinate '{}': {}", x_part, e)))?;
    
    let y = y_part.split_once(sep)
        .ok_or_else(|| ParseError::InvalidCoord(format!("Missing separator '{}' in Y part: {}", sep, y_part)))?
        .1
        .parse::<i128>()
        .map_err(|e| ParseError::InvalidNumber(format!("Invalid Y coordinate '{}': {}", y_part, e)))?;
    
    Ok(Coord { x, y })
}

/// Parses the input string into a vector of machines
fn parse_input(input: &str) -> Result<Vec<Machine>, ParseError> {
    input
        .split("\n\n")
        .enumerate()
        .map(|(i, machine_str)| {
            let mut lines = machine_str.lines();
            
            let a_line = lines.next()
                .ok_or_else(|| ParseError::InvalidMachine(format!("Machine {}: Missing Button A line", i + 1)))?;
            let a = a_line.strip_prefix("Button A: ")
                .ok_or_else(|| ParseError::InvalidMachine(format!("Machine {}: Invalid Button A format: {}", i + 1, a_line)))?;
            
            let b_line = lines.next()
                .ok_or_else(|| ParseError::InvalidMachine(format!("Machine {}: Missing Button B line", i + 1)))?;
            let b = b_line.strip_prefix("Button B: ")
                .ok_or_else(|| ParseError::InvalidMachine(format!("Machine {}: Invalid Button B format: {}", i + 1, b_line)))?;
            
            let prize_line = lines.next()
                .ok_or_else(|| ParseError::InvalidMachine(format!("Machine {}: Missing Prize line", i + 1)))?;
            let prize = prize_line.strip_prefix("Prize: ")
                .ok_or_else(|| ParseError::InvalidMachine(format!("Machine {}: Invalid Prize format: {}", i + 1, prize_line)))?;

            Ok(Machine {
                a: parse_coord(a, '+', ',')?,
                b: parse_coord(b, '+', ',')?,
                prize: parse_coord(prize, '=', ',')?,
            })
        })
        .collect()
}

// Extended GCD: returns (gcd, x, y) such that ax + by = gcd
fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = extended_gcd(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (gcd, x, y)
    }
}

/// Configuration for solving claw machines
#[derive(Debug, Clone, Copy)]
struct SolverConfig {
    /// Maximum number of button presses allowed (None = unlimited)
    max_presses: Option<i128>,
    /// Cost per A button press
    cost_a: i128,
    /// Cost per B button press  
    cost_b: i128,
}

impl SolverConfig {
    /// Configuration for Part 1 (limited presses)
    fn part_one() -> Self {
        Self {
            max_presses: Some(100),
            cost_a: 3,
            cost_b: 1,
        }
    }
    
    /// Configuration for Part 2 (unlimited presses)
    fn part_two() -> Self {
        Self {
            max_presses: None,
            cost_a: 3,
            cost_b: 1,
        }
    }
}

/// Solves a single claw machine with the given configuration
fn solve_machine(machine: &Machine, config: SolverConfig) -> Option<u64> {
    let Machine { 
        a: Coord { x: ax, y: ay }, 
        b: Coord { x: bx, y: by }, 
        prize: Coord { x: px, y: py } 
    } = machine;
    
    let (ax, ay, bx, by, px, py) = (*ax, *ay, *bx, *by, *px, *py);

    let det = ax * by - ay * bx;

    if det != 0 {
        // Non-colinear case: use Cramer's rule
        solve_non_colinear(ax, ay, bx, by, px, py, config)
    } else {
        // Colinear case: use GCD approach
        solve_colinear(ax, ay, bx, by, px, py, config)
    }
}

/// Solves the non-colinear case using Cramer's rule
fn solve_non_colinear(ax: i128, ay: i128, bx: i128, by: i128, px: i128, py: i128, config: SolverConfig) -> Option<u64> {
    let det = ax * by - ay * bx;
    let n_a_num = px * by - py * bx;
    let n_b_num = ax * py - ay * px;

    if n_a_num % det != 0 || n_b_num % det != 0 {
        return None;
    }

    let n_a = n_a_num / det;
    let n_b = n_b_num / det;

    if n_a < 0 || n_b < 0 {
        return None;
    }

    // Check press limits if configured
    if let Some(max) = config.max_presses {
        if n_a > max || n_b > max {
            return None;
        }
    }

    let cost = config.cost_a * n_a + config.cost_b * n_b;
    Some(cost as u64)
}

/// Solves the colinear case using Extended GCD
fn solve_colinear(ax: i128, ay: i128, bx: i128, _by: i128, px: i128, py: i128, config: SolverConfig) -> Option<u64> {
    // Check if prize is on the same line as the moves
    if ax * py - ay * px != 0 {
        return None; // Prize not on the same line
    }

    // Reduce to 1D problem: find n_a, n_b such that ax*n_a + bx*n_b = px
    // and minimize cost = cost_a*n_a + cost_b*n_b
    
    if ax == 0 && bx == 0 {
        return if px == 0 { Some(0) } else { None };
    }
    
    if ax == 0 {
        // Only B moves: bx * n_b = px
        if px % bx != 0 {
            return None;
        }
        let n_b = px / bx;
        if n_b < 0 {
            return None;
        }
        if let Some(max) = config.max_presses {
            if n_b > max {
                return None;
            }
        }
        return Some((config.cost_b * n_b) as u64);
    }
    
    if bx == 0 {
        // Only A moves: ax * n_a = px
        if px % ax != 0 {
            return None;
        }
        let n_a = px / ax;
        if n_a < 0 {
            return None;
        }
        if let Some(max) = config.max_presses {
            if n_a > max {
                return None;
            }
        }
        return Some((config.cost_a * n_a) as u64);
    }

    // Both moves: use extended GCD to find one solution
    let (gcd, x, y) = extended_gcd(ax, bx);
    
    if px % gcd != 0 {
        return None; // No integer solution
    }
    
    // One solution: n_a = x * (px/gcd), n_b = y * (px/gcd)
    let scale = px / gcd;
    let n_a = x * scale;
    let n_b = y * scale;
    
    // Adjust to find non-negative solutions
    // General solution: n_a = n_a0 + k*(bx/gcd), n_b = n_b0 - k*(ax/gcd)
    let step_a = bx / gcd;
    let step_b = ax / gcd;
    
    find_optimal_solution(n_a, n_b, step_a, step_b, config)
}

/// Finds the optimal solution by searching the parameter space
fn find_optimal_solution(n_a: i128, n_b: i128, step_a: i128, step_b: i128, config: SolverConfig) -> Option<u64> {
    let mut best_cost: Option<i128> = None;
    
    // Determine search range based on configuration
    let search_range = if config.max_presses.is_some() { 200 } else { 1000 };
    
    for k in -search_range..=search_range {
        let test_a = n_a + k * step_a;
        let test_b = n_b - k * step_b;
        
        if test_a >= 0 && test_b >= 0 {
            // Check press limits if configured
            if let Some(max) = config.max_presses {
                if test_a > max || test_b > max {
                    continue;
                }
            }
            
            let cost = config.cost_a * test_a + config.cost_b * test_b;
            best_cost = match best_cost {
                Some(prev) => Some(prev.min(cost)),
                None => Some(cost),
            };
        }
    }
    
    best_cost.map(|c| c as u64)
}

/// Solves Part 1: Find minimum tokens with 100-press limit per machine
pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input).ok()?;
    let config = SolverConfig::part_one();
    let total: u64 = machines.iter().filter_map(|machine| solve_machine(machine, config)).sum();
    Some(total)
}

/// Solves Part 2: Find minimum tokens with unlimited presses and offset applied
pub fn part_two(input: &str) -> Option<u64> {
    let mut machines = parse_input(input).ok()?;
    
    // Add 10000000000000 to both X and Y coordinates of every prize
    const OFFSET: i128 = 10000000000000;
    for machine in &mut machines {
        machine.prize.x += OFFSET;
        machine.prize.y += OFFSET;
    }
    
    let config = SolverConfig::part_two();
    let total: u64 = machines.iter().filter_map(|machine| solve_machine(machine, config)).sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(200)); // Only machines 2 and 4 are solvable in Part 2
    }
}
