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
//! - Non-colinear case: direct closed-form solution (no search)
//! - Colinear case: closed-form optimal solution using integer bounds (O(1), no k-loop)
//! - Uses i128 for large number arithmetic to prevent overflow
//!
//! ## Readability & Assumptions
//! - Math is hidden behind small, clearly named helpers with plain-English docs
//! - Code and comments are written for a non-specialist audience
//! - Parsing assumes well-formed AoC input (panics on malformed input)
//!
//! ## Personal Reflection
//! This puzzle was a great learning experience in advanced mathematics. While I couldn't solve it
//! independently due to my limited math education, the collaborative problem-solving process
//! helped me understand concepts like linear algebra, number theory, and algorithmic thinking
//! that I hadn't encountered before.

advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy, PartialEq)]
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

/// Parse a coordinate like "X+94, Y+34" or "X=8400, Y=5400".
/// Assumes well-formed input per AoC (panics on malformed input).
fn parse_coord(s: &str, sep: char, delim: char) -> Coord {
    let (x_part, y_part) = s.split_once(delim).unwrap();
    let x = x_part.split_once(sep).unwrap().1.parse::<i128>().unwrap();
    let y = y_part.split_once(sep).unwrap().1.parse::<i128>().unwrap();
    Coord { x, y }
}

/// Parse the full input into machines (assumes well-formed AoC input).
fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|machine_str| {
            let mut lines = machine_str.lines();
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

/// Solve one machine.
///
/// Plain-English summary:
/// - If the two moves point in different directions (not colinear), there is at most one way
///   to land exactly on the prize. We compute that directly.
/// - If the two moves are along the same line (colinear), there are many ways to combine them.
///   We pick the cheapest combination that meets the constraints.
fn solve_machine(machine: &Machine, config: SolverConfig) -> Option<u64> {
    let Machine {
        a: Coord { x: ax, y: ay },
        b: Coord { x: bx, y: by },
        prize: Coord { x: px, y: py },
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

/// Non-colinear case: direct closed-form solution.
///
/// Idea: two move vectors form a 2×2 system with a unique real solution. We accept it if and
/// only if it yields nonnegative integers (and respects the optional press limit).
fn solve_non_colinear(
    ax: i128,
    ay: i128,
    bx: i128,
    by: i128,
    px: i128,
    py: i128,
    config: SolverConfig,
) -> Option<u64> {
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

/// Colinear case: integer combinations along one line.
///
/// Idea: reduce to 1D equation ax*nA + bx*nB = px, find one integer solution with extended GCD,
/// then slide along all solutions to satisfy constraints and minimize cost in O(1).
fn solve_colinear(
    ax: i128,
    ay: i128,
    bx: i128,
    _by: i128,
    px: i128,
    py: i128,
    config: SolverConfig,
) -> Option<u64> {
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

    // Adjust to find non-negative solutions in closed form (no k-loop)
    // General solution: n_a = n_a0 + k*t, n_b = n_b0 - k*s
    let t = bx / gcd; // step for n_a when k increases by 1
    let s = ax / gcd; // step for n_b when k increases by 1

    // Helper closures for integer ceil/floor division with i128
    let ceil_div = |a: i128, b: i128| -> i128 {
        if b == 0 {
            return 0;
        }
        if (a ^ b) >= 0 {
            // same sign
            (a + (b.abs() - 1)) / b
        } else {
            a / b
        }
    };
    let floor_div = |a: i128, b: i128| -> i128 {
        if b == 0 {
            return 0;
        }
        if (a ^ b) >= 0 {
            // same sign
            a / b
        } else {
            (a - (b.abs() - 1)) / b
        }
    };

    // Feasibility ranges from nonnegativity
    // n_a = n_a0 + k*t >= 0  =>  k >= ceil_div(-n_a0, t)
    // n_b = n_b0 - k*s >= 0  =>  k <= floor_div(n_b0, s)
    let mut k_min = ceil_div(-n_a, t);
    let mut k_max = floor_div(n_b, s);

    // Add press-limit constraints if configured
    if let Some(max) = config.max_presses {
        // n_a <= max  =>  n_a0 + k*t <= max  =>  k <= floor_div(max - n_a0, t)
        // n_b <= max  =>  n_b0 - k*s <= max  =>  k >= ceil_div(n_b0 - max, s)
        k_max = k_max.min(floor_div(max - n_a, t));
        k_min = k_min.max(ceil_div(n_b - max, s));
    }

    if k_min > k_max {
        return None;
    }

    // Cost as a function of k: C(k) = c_a*(n_a0 + k*t) + c_b*(n_b0 - k*s)
    // => C(k) = C0 + k*(c_a*t - c_b*s). Monotonic in k.
    let slope = config.cost_a * t - config.cost_b * s;
    let k_star = if slope < 0 {
        k_max
    } else if slope > 0 {
        k_min
    } else {
        k_min
    }; // flat: any feasible; pick k_min

    let best_a = n_a + k_star * t;
    let best_b = n_b - k_star * s;
    if best_a < 0 || best_b < 0 {
        return None;
    }
    if let Some(max) = config.max_presses {
        if best_a > max || best_b > max {
            return None;
        }
    }

    let cost = config.cost_a * best_a + config.cost_b * best_b;
    Some(cost as u64)
}

/// Solves Part 1: Find minimum tokens with 100-press limit per machine
pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse_input(input);
    let config = SolverConfig::part_one();
    let total: u64 = machines
        .iter()
        .filter_map(|machine| solve_machine(machine, config))
        .sum();
    Some(total)
}

/// Solves Part 2: Find minimum tokens with unlimited presses and offset applied
pub fn part_two(input: &str) -> Option<u64> {
    let mut machines = parse_input(input);

    // Add 10000000000000 to both X and Y coordinates of every prize
    const OFFSET: i128 = 10000000000000;
    for machine in &mut machines {
        machine.prize.x += OFFSET;
        machine.prize.y += OFFSET;
    }

    let config = SolverConfig::part_two();
    let total: u64 = machines
        .iter()
        .filter_map(|machine| solve_machine(machine, config))
        .sum();
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
        assert_eq!(result, Some(875318608908)); // Only machines 2 and 4 are solvable in Part 2
    }
}
