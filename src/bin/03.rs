use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug, Clone)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut instructions = Vec::new();

    for cap in re.captures_iter(input) {
        let full_match = cap.get(0).unwrap().as_str();

        let instruction = match full_match {
            "do()" => Instruction::Do,
            "don't()" => Instruction::Dont,
            _ if full_match.starts_with("mul(") => {
                let x: u64 = cap.get(1).unwrap().as_str().parse().unwrap();
                let y: u64 = cap.get(2).unwrap().as_str().parse().unwrap();
                Instruction::Mul(x, y)
            }
            _ => continue, // Skip invalid matches (shouldn't happen with our regex)
        };

        instructions.push(instruction);
    }

    instructions
}

pub fn part_one(input: &str) -> Option<u64> {
    let instructions = parse_instructions(input);

    let sum = instructions
        .into_iter()
        .filter_map(|inst| match inst {
            Instruction::Mul(x, y) => Some(x * y),
            _ => None,
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let instructions = parse_instructions(input);

    let mut sum = 0;
    let mut active = true;

    for instruction in instructions {
        match instruction {
            Instruction::Do => active = true,
            Instruction::Dont => active = false,
            Instruction::Mul(x, y) => {
                if active {
                    sum += x * y;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
