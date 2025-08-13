advent_of_code::solution!(1);

fn parse_input(input: &str) -> Input {
     input
        .trim()
        .split("\n")
        .map(|line| line.parse::<u64>().unwrap())
        .collect()

}

type Input = Vec<u64>;

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = parse_input(input);

    for i in 0..parsed.len() {
        for j in i + 1..parsed.len() {
            let (a, b) = (parsed[i], parsed[j]);
            if a + b == 2020 {
                return Some(a * b);
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u64> {
    let parsed = parse_input(input);
    // let result = with_size(&parsed, 3, &mut vec![]);
    // println!("RESULT: {:?}", result);

    for i in 0..parsed.len() {
        for j in i + 1..parsed.len() {
            for k in j + 1..parsed.len() {
                let (a, b, c) = (parsed[i], parsed[j], parsed[k]);
                if a + b + c == 2020 {
                    return Some(a * b * c);
                }
            }
        }
    }

    None
}

fn with_size(input: &[u64], size: u64, nums: &mut Input) -> Option<u64> {
    // println!("Slice: {:?}", input);
    // println!("Nums: {:?}", nums);
    for i in 0..input.len() {
        if size - 1 > 0 {
            nums.push(input[i]);
            if let None =  with_size(&input[1..], size - 1, nums) {
                nums.pop();
                continue;

            }
        } else {
            println!("{}", nums.iter().sum::<u64>());
            if nums.iter().sum::<u64>().eq(&2020) {
                return Some(nums.iter().fold(1, |acc, num| acc * num))
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(514579));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(241861950));
    }
}
