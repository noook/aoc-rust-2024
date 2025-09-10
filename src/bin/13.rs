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

fn parse_coord(s: &str, sep: char, delim: char) -> Coord {
    let (x, y) = s.split_once(delim).unwrap();
    Coord {
        x: x.split_once(sep).unwrap().1.parse().unwrap(),
        y: y.split_once(sep).unwrap().1.parse().unwrap(),
    }
}

fn parse_input(input: &str) -> Vec<Machine> {

    input
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();
            let a = lines.next().unwrap().strip_prefix("Button A: ").unwrap();
            let b = lines.next().unwrap().strip_prefix("Button B: ").unwrap();
            let prize = lines.next().unwrap().strip_prefix("Prize: ").unwrap();

            Machine {
                a: parse_coord(a, '+', ','),
                b: parse_coord(b, '+', ','),
                prize: parse_coord(prize, '=', ','),
            }
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

fn find_min_tokens(machine: &Machine) -> Option<u64> {
    let Machine { 
        a: Coord { x: ax, y: ay }, 
        b: Coord { x: bx, y: by }, 
        prize: Coord { x: px, y: py } 
    } = machine;
    
    let (ax, ay, bx, by, px, py) = (*ax, *ay, *bx, *by, *px, *py);

    let det = ax * by - ay * bx;

    if det != 0 {
        // Non-colinear case: use Cramer's rule
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

        // Part 1 constraint: at most 100 presses each
        if n_a > 100 || n_b > 100 {
            return None;
        }

        let cost = 3_i128 * n_a + 1_i128 * n_b;
        return Some(cost as u64);
    }

    // det == 0: colinear case - use GCD approach
    // Check if prize is on the same line as the moves
    if ax * py - ay * px != 0 {
        return None; // Prize not on the same line
    }

    // Reduce to 1D problem: find n_a, n_b such that ax*n_a + bx*n_b = px
    // and minimize cost = 3*n_a + n_b
    
    if ax == 0 && bx == 0 {
        return if px == 0 { Some(0) } else { None };
    }
    
    if ax == 0 {
        // Only B moves: bx * n_b = px
        if px % bx != 0 {
            return None;
        }
        let n_b = px / bx;
        if n_b < 0 || n_b > 100 {
            return None;
        }
        return Some(n_b as u64);
    }
    
    if bx == 0 {
        // Only A moves: ax * n_a = px
        if px % ax != 0 {
            return None;
        }
        let n_a = px / ax;
        if n_a < 0 || n_a > 100 {
            return None;
        }
        return Some((3 * n_a) as u64);
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
    
    // Find the range of k that gives n_a, n_b >= 0
    let mut best_cost: Option<i128> = None;
    
    // Try k values around the initial solution
    for k in -200..=200 {
        let test_a = n_a + k * step_a;
        let test_b = n_b - k * step_b;
        
        if test_a >= 0 && test_b >= 0 && test_a <= 100 && test_b <= 100 {
            let cost = 3 * test_a + test_b;
            best_cost = match best_cost {
                Some(prev) => Some(prev.min(cost)),
                None => Some(cost),
            };
        }
    }
    
    best_cost.map(|c| c as u64)
}

pub fn part_one(_input: &str) -> Option<u64> {
    let machines = parse_input(_input);
    let total: u64 = machines.iter().filter_map(find_min_tokens).sum();
    Some(total)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let mut machines = parse_input(_input);
    
    // Add 10000000000000 to both X and Y coordinates of every prize
    const OFFSET: i128 = 10000000000000;
    for machine in &mut machines {
        machine.prize.x += OFFSET;
        machine.prize.y += OFFSET;
    }
    
    // Remove the 100-press constraint for Part 2
    let total: u64 = machines.iter().filter_map(|machine| {
        let Machine { 
            a: Coord { x: ax, y: ay }, 
            b: Coord { x: bx, y: by }, 
            prize: Coord { x: px, y: py } 
        } = machine;
        
        let (ax, ay, bx, by, px, py) = (*ax, *ay, *bx, *by, *px, *py);
        
        let det = ax * by - ay * bx;
        
        if det != 0 {
            // Non-colinear case: use Cramer's rule
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
            
            // No 100-press constraint for Part 2
            let cost = 3_i128 * n_a + 1_i128 * n_b;
            return Some(cost as u64);
        }
        
        // det == 0: colinear case - use GCD approach (no 100-press constraint)
        if ax * py - ay * px != 0 {
            return None; // Prize not on the same line
        }
        
        if ax == 0 && bx == 0 {
            return if px == 0 { Some(0) } else { None };
        }
        
        if ax == 0 {
            if px % bx != 0 {
                return None;
            }
            let n_b = px / bx;
            if n_b < 0 {
                return None;
            }
            return Some(n_b as u64);
        }
        
        if bx == 0 {
            if px % ax != 0 {
                return None;
            }
            let n_a = px / ax;
            if n_a < 0 {
                return None;
            }
            return Some((3 * n_a) as u64);
        }
        
        // Both moves: use extended GCD
        let (gcd, x, y) = extended_gcd(ax, bx);
        
        if px % gcd != 0 {
            return None;
        }
        
        let scale = px / gcd;
        let n_a = x * scale;
        let n_b = y * scale;
        
        let step_a = bx / gcd;
        let step_b = ax / gcd;
        
        let mut best_cost: Option<i128> = None;
        
        // Try k values around the initial solution (wider range for Part 2)
        for k in -1000..=1000 {
            let test_a = n_a + k * step_a;
            let test_b = n_b - k * step_b;
            
            if test_a >= 0 && test_b >= 0 {
                let cost = 3 * test_a + test_b;
                best_cost = match best_cost {
                    Some(prev) => Some(prev.min(cost)),
                    None => Some(cost),
                };
            }
        }
        
        best_cost.map(|c| c as u64)
    }).sum();
    
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
