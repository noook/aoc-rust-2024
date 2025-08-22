advent_of_code::solution!(3);

pub fn parse_input(input: &str) -> Vec<&str> {
    input.trim().split("\n").collect()
}

struct Slope {
    right: usize,
    down: usize,
}

fn run_slope(map: &[&str], slope: &Slope) -> u64 {
    let mut count: u64 = 0;

    for (i, j) in (0..).step_by(slope.right).zip((0..map.len()).step_by(slope.down)) {
        let char = map[j].chars().nth(i % map[j].len()).unwrap();
        if char == '#' {
            count += 1;
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = parse_input(input);
    let count: u64 = run_slope(&lines, &Slope { right: 3, down: 1 });

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let slopes: [Slope; 5] = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    let map = parse_input(input);

    let result: u64 = slopes.iter().fold(1, |acc, slope| {
        let count: u64 = run_slope(&map, slope);
        count * acc
    });

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<u64> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result: Option<u64> = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(336));
    }
}
