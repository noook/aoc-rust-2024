use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

#[derive(Debug, Clone)]
struct OrderingRules {
    successors: HashMap<u32, HashSet<u32>>,
}

impl OrderingRules {
    fn new() -> Self {
        Self {
            successors: HashMap::new(),
        }
    }

    fn add_rule(&mut self, before: u32, after: u32) {
        self.successors.entry(before).or_default().insert(after);
    }

    fn is_valid_order(&self, update: &[u32]) -> bool {
        update.iter().enumerate().all(|(i, &page)| {
            // Check that all pages after this one are allowed to come after it
            update[i + 1..]
                .iter()
                .all(|&later_page| !self.violates_rule(later_page, page))
        })
    }

    /// Returns true if placing `first` before `second` violates any rule
    fn violates_rule(&self, first: u32, second: u32) -> bool {
        self.successors
            .get(&first)
            .map_or(false, |successors| successors.contains(&second))
    }

    /// Fixes the order of an invalid update using topological sorting
    fn fix_order(&self, update: &[u32]) -> Vec<u32> {
        let mut pages = update.to_vec();

        // Sort using a comparator based on the ordering rules
        pages.sort_by(|&a, &b| self.compare_pages(a, b));

        pages
    }

    /// Compares two pages according to the ordering rules
    fn compare_pages(&self, a: u32, b: u32) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        // Check if a must come before b
        if self
            .successors
            .get(&a)
            .map_or(false, |successors| successors.contains(&b))
        {
            return Ordering::Less;
        }

        // Check if b must come before a
        if self
            .successors
            .get(&b)
            .map_or(false, |successors| successors.contains(&a))
        {
            return Ordering::Greater;
        }

        // No ordering constraint between these pages
        Ordering::Equal
    }
}

impl Default for OrderingRules {
    fn default() -> Self {
        Self::new()
    }
}

/// Parses the input into ordering rules and update sequences
/// Returns None if parsing fails (instead of panicking)
fn parse_input(input: &str) -> Option<(OrderingRules, Vec<Vec<u32>>)> {
    let (rules_section, updates_section) = input.split_once("\n\n")?;

    // Parse ordering rules
    let mut rules = OrderingRules::new();
    for line in rules_section.lines() {
        let (before_str, after_str) = line.split_once('|')?;
        let before = before_str.parse().ok()?;
        let after = after_str.parse().ok()?;
        rules.add_rule(before, after);
    }

    // Parse updates
    let updates: Option<Vec<Vec<u32>>> = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
        .collect();

    Some((rules, updates?))
}

/// Returns the middle element of a slice
fn middle_element<T>(slice: &[T]) -> &T {
    &slice[slice.len() / 2]
}

pub fn part_one(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input)?;

    let sum = updates
        .iter()
        .filter(|update| rules.is_valid_order(update))
        .map(|update| *middle_element(update) as u64)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, updates) = parse_input(input)?;

    let sum = updates
        .iter()
        .filter(|update| !rules.is_valid_order(update))
        .map(|update| {
            let fixed = rules.fix_order(update);
            *middle_element(&fixed) as u64
        })
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }

    #[test]
    fn test_ordering_rules() {
        let mut rules = OrderingRules::new();
        rules.add_rule(47, 53);
        rules.add_rule(97, 13);

        assert!(rules.is_valid_order(&[47, 53]));
        assert!(!rules.is_valid_order(&[53, 47]));

        let fixed = rules.fix_order(&[53, 47]);
        assert_eq!(fixed, vec![47, 53]);
    }

    #[test]
    fn test_middle_element() {
        assert_eq!(*middle_element(&[1, 2, 3]), 2);
        assert_eq!(*middle_element(&[1, 2, 3, 4, 5]), 3);
        assert_eq!(*middle_element(&[1, 2]), 2);
    }
}
