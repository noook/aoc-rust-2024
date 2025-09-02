advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect()
}

fn create_blocks(input: &[u64]) -> Vec<Option<u64>> {
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

    blocks
}

fn calculate_checksum(blocks: &[Option<u64>]) -> u64 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &block)| block.map(|id| pos as u64 * id))
        .sum()
}

pub fn part_one(_input: &str) -> Option<u64> {
    let input = parse_input(_input);
    let mut blocks = create_blocks(&input);

    let mut i = 0;
    let mut j = blocks.len() - 1;

    // Compact the disk by moving blocks one at a time
    while i < j {
        if blocks[i].is_some() {
            i += 1;
        } else if blocks[j].is_none() {
            j -= 1;
        } else {
            blocks[i] = blocks[j];
            blocks[j] = None;
            i += 1;
            j -= 1;
        }
    }

    Some(calculate_checksum(&blocks))
}

pub fn part_two(_input: &str) -> Option<u64> {
    let input = parse_input(_input);
    let mut blocks = create_blocks(&input);

    let mut j = blocks.len() - 1;

    // Process files from right to left
    while j > 0 {
        // Skip empty space
        if blocks[j].is_none() {
            j -= 1;
            continue;
        }

        // Find the size of the current file
        let file_id = blocks[j].unwrap();
        let mut size = 0;
        while j > 0 && blocks[j] == Some(file_id) {
            size += 1;
            if j == 0 { break; }
            j -= 1;
        }
        // j is now pointing to the position before the file starts
        let file_start = j + 1;

        // Find leftmost free space that can fit this file
        let mut found_space = false;
        let mut free_start = 0;
        let mut i = 0;
        while i < file_start {
            if blocks[i].is_none() {
                let space_start = i;
                let mut free_size = 0;
                while i < file_start && blocks[i].is_none() {
                    free_size += 1;
                    i += 1;
                }
                if free_size >= size {
                    free_start = space_start;
                    found_space = true;
                    break;
                }
            } else {
                i += 1;
            }
        }

        // Move the file if we found space
        if found_space {
            for k in 0..size {
                blocks[free_start + k] = Some(file_id);
                blocks[file_start + k] = None;
            }
        }
    }

    Some(calculate_checksum(&blocks))
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
        assert_eq!(result, Some(2858));
    }
}
