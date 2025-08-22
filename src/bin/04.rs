use std::collections::HashMap;

advent_of_code::solution!(4);

/// A passport represented as a collection of field-value pairs (zero-copy)
type Passport<'a> = HashMap<&'a str, &'a str>;

/// The required fields for a valid passport (cid is optional)
const REQUIRED_FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

/// Parse input into an iterator of passports
fn parse_input(input: &str) -> impl Iterator<Item = Passport> + '_ {
    input
        .trim()
        .split("\n\n")
        .map(parse_passport_entry)
}

/// Parse a single passport entry from a multi-line string (zero-copy)
fn parse_passport_entry(entry: &str) -> Passport {
    entry
        .split_whitespace()
        .filter_map(|field| field.split_once(':'))
        .collect()
}

/// Check if a passport has all required fields (Part 1)
/// Optimized: Uses short-circuiting - stops at first missing field
fn has_required_fields(passport: &Passport<'_>) -> bool {
    REQUIRED_FIELDS.iter().all(|&field| passport.contains_key(field))
}

/// Validate a year field within the given range
fn validate_year(value: &str, min: u32, max: u32) -> bool {
    value
        .parse::<u32>()
        .map_or(false, |year| (min..=max).contains(&year))
}

/// Validate height field (must be valid number with cm or in suffix)
fn validate_height(value: &str) -> bool {
    if let Some(cm_str) = value.strip_suffix("cm") {
        cm_str
            .parse::<u32>()
            .map_or(false, |height| (150..=193).contains(&height))
    } else if let Some(in_str) = value.strip_suffix("in") {
        in_str
            .parse::<u32>()
            .map_or(false, |height| (59..=76).contains(&height))
    } else {
        false
    }
}

/// Validate hair color (# followed by 6 hex digits)
/// Optimized: Use bytes instead of chars for better performance
fn validate_hair_color(value: &str) -> bool {
    value.len() == 7 
        && value.starts_with('#')
        && value.as_bytes()[1..].iter().all(|&b| b.is_ascii_hexdigit())
}

/// Validate eye color (must be one of the valid colors)
/// Optimized: Early exit on length check
fn validate_eye_color(value: &str) -> bool {
    value.len() == 3 && matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

/// Validate passport ID (exactly 9 digits)
/// Optimized: Use bytes for faster digit checking
fn validate_passport_id(value: &str) -> bool {
    value.len() == 9 && value.as_bytes().iter().all(|&b| b.is_ascii_digit())
}

/// Check if a passport is fully valid (Part 2)
/// Optimized: Check required fields first, then validate in order of likely failure
fn is_fully_valid(passport: &Passport<'_>) -> bool {
    // Early return if missing required fields
    if !has_required_fields(passport) {
        return false;
    }
    
    // Validate in order of complexity (simple checks first for early exit)
    validate_field(passport, "ecl", validate_eye_color)        // Simple match
        && validate_field(passport, "pid", validate_passport_id) // Length + digit check
        && validate_field(passport, "hcl", validate_hair_color) // Length + hex check
        && validate_field(passport, "byr", |v| validate_year(v, 1920, 2002)) // Parse + range
        && validate_field(passport, "iyr", |v| validate_year(v, 2010, 2020)) // Parse + range
        && validate_field(passport, "eyr", |v| validate_year(v, 2020, 2030)) // Parse + range
        && validate_field(passport, "hgt", validate_height)     // Most complex validation
}

/// Helper function to validate a specific field
fn validate_field<F>(passport: &Passport<'_>, field: &str, validator: F) -> bool
where
    F: Fn(&str) -> bool,
{
    passport
        .get(field)
        .map_or(false, |value| validator(value))
}

pub fn part_one(input: &str) -> Option<u64> {
    let count = parse_input(input)
        .filter(has_required_fields)
        .count();
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let count = parse_input(input)
        .filter(is_fully_valid)
        .count();
    Some(count as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(4));
    }
}
