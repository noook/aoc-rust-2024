advent_of_code::solution!(7);

type Equation = (u64, Vec<u64>);

fn parse_input(input: &str) -> impl Iterator<Item = Equation> + '_ {
    input.trim().lines().map(|line| {
        let (target, numbers) = line.split_once(": ").expect("Invalid format");
        let target = target.parse().expect("Invalid target");
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse().expect("Invalid number"))
            .collect();
        (target, numbers)
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    const fn apply(self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
            Self::Concat => {
                let digits = Self::count_digits(b);
                a * 10_u64.pow(digits) + b
            }
        }
    }

    const fn count_digits(mut n: u64) -> u32 {
        if n == 0 {
            return 1;
        }
        let mut count = 0;
        while n > 0 {
            n /= 10;
            count += 1;
        }
        count
    }
}

fn can_achieve_target(target: u64, numbers: &[u64], operators: &[Operator]) -> bool {
    fn search(target: u64, numbers: &[u64], operators: &[Operator], idx: usize, acc: u64) -> bool {
        match idx {
            // If we've used all the numbers, check if we've reached the target
            i if i == numbers.len() => acc == target,
            // Operations can only increase the value, so if we already exceed the target, we can stop
            _ if acc > target => false,
            // Try all the operators on the next number
            i => operators
                .iter()
                .any(|&op| search(target, numbers, operators, i + 1, op.apply(acc, numbers[i]))),
        }
    }

    !numbers.is_empty() && search(target, numbers, operators, 1, numbers[0])
}

fn solve(input: &str, operators: &[Operator]) -> u64 {
    parse_input(input)
        .filter(|(target, numbers)| can_achieve_target(*target, numbers, operators))
        .map(|(target, _)| target)
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    const OPERATORS: &[Operator] = &[Operator::Add, Operator::Multiply];
    Some(solve(input, OPERATORS))
}

pub fn part_two(input: &str) -> Option<u64> {
    const OPERATORS: &[Operator] = &[Operator::Add, Operator::Multiply, Operator::Concat];
    Some(solve(input, OPERATORS))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operator_apply() {
        use Operator::*;

        assert_eq!(Add.apply(10, 19), 29);
        assert_eq!(Multiply.apply(10, 19), 190);
        assert_eq!(Concat.apply(12, 345), 12345);
        assert_eq!(Concat.apply(15, 6), 156);
        assert_eq!(Concat.apply(123, 0), 1230);
    }

    #[test]
    fn test_count_digits() {
        assert_eq!(Operator::count_digits(0), 1);
        assert_eq!(Operator::count_digits(5), 1);
        assert_eq!(Operator::count_digits(42), 2);
        assert_eq!(Operator::count_digits(999), 3);
        assert_eq!(Operator::count_digits(1000), 4);
    }

    #[test]
    fn test_can_achieve_target() {
        use Operator::*;

        // 190: 10 19 -> 10 * 19 = 190
        assert!(can_achieve_target(190, &[10, 19], &[Add, Multiply]));

        // 3267: 81 40 27 -> multiple solutions exist
        assert!(can_achieve_target(3267, &[81, 40, 27], &[Add, Multiply]));

        // 156: 15 6 -> requires concatenation
        assert!(!can_achieve_target(156, &[15, 6], &[Add, Multiply]));
        assert!(can_achieve_target(156, &[15, 6], &[Add, Multiply, Concat]));
    }

    #[test]
    fn test_edge_cases() {
        use Operator::*;

        // Empty numbers
        assert!(!can_achieve_target(42, &[], &[Add, Multiply]));

        // Single number
        assert!(can_achieve_target(42, &[42], &[Add, Multiply]));
        assert!(!can_achieve_target(42, &[24], &[Add, Multiply]));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
