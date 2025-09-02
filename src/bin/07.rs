advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input.trim().lines().map(|line| {
        let (num, nums) = line.split_once(": ").unwrap();
        (num.parse().unwrap(), nums.split_whitespace().map(|x| x.parse().unwrap()).collect())
    }).collect()
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => (a.to_string() + &b.to_string()).parse().unwrap(),
        }
    }
}

fn generate_operator_chain(numbers: &[u64], operators: &[Operator]) -> Vec<Vec<Operator>> {
    let n = numbers.len();
    let op_count = n.saturating_sub(1);
    let mut results = Vec::new();
    let mut current = Vec::with_capacity(op_count);

    fn backtrack(
        pos: usize,
        op_count: usize,
        operators: &[Operator],
        current: &mut Vec<Operator>,
        results: &mut Vec<Vec<Operator>>,
    ) {
        if pos == op_count {
            results.push(current.clone());
            return;
        }
        for op in operators {
            current.push(op.clone());
            backtrack(pos + 1, op_count, operators, current, results);
            current.pop();
        }
    }

    if op_count > 0 {
        backtrack(0, op_count, operators, &mut current, &mut results);
    } else {
        results.push(vec![]);
    }

    results
}

fn is_chain_valid(target: u64, numbers: &[u64], chain: &[Operator]) -> bool {
    if numbers.is_empty() {
        return false;
    }
    
    let mut result = numbers[0];
    
    for (i, op) in chain.iter().enumerate() {
        if i + 1 >= numbers.len() {
            break;
        }
        result = op.apply(result, numbers[i + 1]);
    }
    
    result == target
}

pub fn part_one(_input: &str) -> Option<u64> {
    let input = parse_input(_input);
    let mut result = 0;

    for (target, numbers) in input {
        let chains = generate_operator_chain(&numbers, &[Operator::Add, Operator::Multiply]);
        let valid = chains.iter().any(|chain| is_chain_valid(target, &numbers, chain));
        if valid {
            result += target;
        }
    }
    
    Some(result)
}

pub fn part_two(_input: &str) -> Option<u64> {
    let input = parse_input(_input);
    let mut result = 0;

    for (target, numbers) in input {
        let chains = generate_operator_chain(&numbers, &[Operator::Add, Operator::Multiply, Operator::Concat]);
        let valid = chains.iter().any(|chain| is_chain_valid(target, &numbers, chain));
        if valid {
            result += target;
        }
    }
    
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

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
