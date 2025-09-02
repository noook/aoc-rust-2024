advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

pub fn part_one(_input: &str) -> Option<u64> {
    let input = parse_input(_input);

    // Create a vector of blocks where each block is either Some(file_id) or None (empty)
    let mut blocks = Vec::new();
    let mut file_id = 0;

    for (i, &len) in input.iter().enumerate() {
        if i % 2 == 0 {
            // File blocks
            for _ in 0..len {
                blocks.push(Some(file_id));
            }
            file_id += 1;
        } else {
            // Empty space
            for _ in 0..len {
                blocks.push(None);
            }
        }
    }

    let mut i = 0;
    let mut j = blocks.len() - 1;

    // Compact the disk and calculate checksum in one pass
    let mut result = 0;
    while i <= j {
        if let Some(id) = blocks[i] {
            // Element is in correct position
            result += i as u64 * id;
            i += 1;
        } else if blocks[j].is_none() {
            // Skip empty space from the right
            j -= 1;
        } else {
            // Use element from j at position i
            let id = blocks[j].unwrap();
            result += i as u64 * id;
            i += 1;
            j -= 1;
        }
    }

    Some(result)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
