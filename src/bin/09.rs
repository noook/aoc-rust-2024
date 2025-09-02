advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq)]
struct FileSpan {
    id: u32,
    start: usize,
    size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct FreeSpan {
    start: usize,
    size: usize,
}

#[derive(Debug)]
struct Disk {
    files: Vec<FileSpan>,
    free_spaces: Vec<FreeSpan>,
    total_size: usize,
}

impl Disk {
    fn from_input(input: &str) -> Self {
        let lengths: Vec<usize> = input
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as usize))
            .collect();

        let mut files = Vec::new();
        let mut free_spaces = Vec::new();
        let mut position = 0;
        let mut file_id = 0;

        for (i, &length) in lengths.iter().enumerate() {
            if length == 0 {
                continue;
            }

            if i % 2 == 0 {
                // File
                files.push(FileSpan {
                    id: file_id,
                    start: position,
                    size: length,
                });
                file_id += 1;
            } else {
                // Free space
                free_spaces.push(FreeSpan {
                    start: position,
                    size: length,
                });
            }
            position += length;
        }

        Disk {
            files,
            free_spaces,
            total_size: position,
        }
    }

    fn calculate_checksum(&self) -> u64 {
        self.files
            .iter()
            .map(|file| {
                (file.start..file.start + file.size)
                    .map(|pos| pos as u64 * file.id as u64)
                    .sum::<u64>()
            })
            .sum()
    }

    fn compact_blocks(&mut self) {
        let mut blocks = self.to_blocks();
        let mut left = 0;
        let mut right = blocks.len() - 1;

        while left < right {
            // Skip non-empty blocks on the left
            while left < right && blocks[left].is_some() {
                left += 1;
            }
            // Skip empty blocks on the right
            while left < right && blocks[right].is_none() {
                right -= 1;
            }
            // Move block if we found a gap and a block
            if left < right {
                blocks[left] = blocks[right];
                blocks[right] = None;
                left += 1;
                right -= 1;
            }
        }

        self.from_blocks(&blocks);
    }

    fn compact_files(&mut self) {
        // Process files in descending order of ID
        self.files.sort_by_key(|f| std::cmp::Reverse(f.id));

        let mut file_idx = 0;
        while file_idx < self.files.len() {
            let file = self.files[file_idx];

            // Find the leftmost free space that can fit this file
            if let Some(space_idx) = self.find_suitable_space(file.size, file.start) {
                let space = self.free_spaces[space_idx];

                // Move the file
                self.files[file_idx].start = space.start;

                // Update free space efficiently
                if space.size == file.size {
                    // Exact fit - remove the free space
                    self.free_spaces.remove(space_idx);
                } else {
                    // Partial fit - shrink the free space
                    self.free_spaces[space_idx] = FreeSpan {
                        start: space.start + file.size,
                        size: space.size - file.size,
                    };
                }

                // Note: We don't need to add free space back since we only care about final positions
                // and we process each file exactly once in decreasing ID order
            }
            file_idx += 1;
        }
    }

    fn find_suitable_space(
        &self,
        needed_size: usize,
        before_position: usize,
    ) -> Option<usize> {
        // Since free_spaces is sorted, we can use early termination
        for (idx, space) in self.free_spaces.iter().enumerate() {
            if space.start >= before_position {
                break; // No point checking further
            }
            if space.size >= needed_size {
                return Some(idx);
            }
        }
        None
    }

    fn to_blocks(&self) -> Vec<Option<u32>> {
        let mut blocks = vec![None; self.total_size];
        for file in &self.files {
            for pos in file.start..file.start + file.size {
                blocks[pos] = Some(file.id);
            }
        }
        blocks
    }

    fn from_blocks(&mut self, blocks: &[Option<u32>]) {
        self.files.clear();
        self.free_spaces.clear();

        let mut i = 0;
        while i < blocks.len() {
            if let Some(file_id) = blocks[i] {
                let start = i;
                let mut size = 0;
                while i < blocks.len() && blocks[i] == Some(file_id) {
                    size += 1;
                    i += 1;
                }
                self.files.push(FileSpan {
                    id: file_id,
                    start,
                    size,
                });
            } else {
                let start = i;
                let mut size = 0;
                while i < blocks.len() && blocks[i].is_none() {
                    size += 1;
                    i += 1;
                }
                if size > 0 {
                    self.free_spaces.push(FreeSpan { start, size });
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut disk = Disk::from_input(input);
    disk.compact_blocks();
    Some(disk.calculate_checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut disk = Disk::from_input(input);
    disk.compact_files();
    Some(disk.calculate_checksum())
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

    #[test]
    fn test_disk_creation() {
        let disk = Disk::from_input("2333133121414131402");
        assert_eq!(disk.files.len(), 10);
        assert_eq!(disk.total_size, 42);
    }

    #[test]
    fn test_checksum_calculation() {
        let disk = Disk::from_input("12345");
        // Input "12345" creates: file 0 (size 1), space (size 2), file 1 (size 3), space (size 4), file 2 (size 5)
        // Layout: 0..111....22222
        // Positions: file 0 at pos 0, file 1 at pos 3-5, file 2 at pos 10-14
        let expected_checksum =
            0 * 0 + (3 * 1 + 4 * 1 + 5 * 1) + (10 * 2 + 11 * 2 + 12 * 2 + 13 * 2 + 14 * 2);
        assert_eq!(disk.calculate_checksum(), expected_checksum);
    }
}
