use std::fmt::{Debug};

advent_of_code::solution!(4);

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

fn parse_input(input: &str) -> impl Iterator<Item = Passport> + '_ {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.replace("\n", " "))
        .map(|s| {
            let mut passport = Passport {
                byr: None,
                iyr: None,
                eyr: None,
                hgt: None,
                hcl: None,
                ecl: None,
                pid: None,
                cid: None,
            };
            for field in s.split_whitespace() {
                if let Some((key, value)) = field.split_once(':') {
                    match key {
                        "byr" => passport.byr = Some(value.to_string()),
                        "iyr" => passport.iyr = Some(value.to_string()),
                        "eyr" => passport.eyr = Some(value.to_string()),
                        "hgt" => passport.hgt = Some(value.to_string()),
                        "hcl" => passport.hcl = Some(value.to_string()),
                        "ecl" => passport.ecl = Some(value.to_string()),
                        "pid" => passport.pid = Some(value.to_string()),
                        "cid" => passport.cid = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
            passport
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<Passport> = parse_input(input).filter(|passport| {
        match passport {
            Passport {
                byr: Some(_),
                iyr: Some(_),
                eyr: Some(_),
                hgt: Some(_),
                hcl: Some(_),
                ecl: Some(_),
                pid: Some(_),
                cid: _,
            } => true,
            _ => false,
        }
    }).collect();
    Some(lines.len() as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
