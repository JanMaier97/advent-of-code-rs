use itertools::Itertools;
use macros::aoc_solver;

use crate::MyResult;

type BlockSize = u32;
type FileId = usize;

struct DiskMap {
    files: Vec<BlockSize>,
    free_spaces: Vec<BlockSize>,
}

#[aoc_solver(2024, 9, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let disk_map = parse_input(input)?;
    let proper_disk = defragment_disk(&disk_map);
    let sum: usize = proper_disk
        .iter()
        .enumerate()
        .map(|(idx, file_id)| idx * file_id)
        .sum();

    let sum = u64::try_from(sum)?;

    Ok(sum)
}

fn defragment_disk(map: &DiskMap) -> Vec<FileId> {
    let mut blocks = Vec::new();
    let mut spaces = map.free_spaces.iter().cloned().rev().collect_vec();
    let mut files = map.files.clone();
    let mut right_file: FileId = files.len() - 1;

    for left_file in 0..files.len() {
        let size = files[left_file];

        for _ in 0..size {
            blocks.push(left_file);
        }

        let Some(mut free_blocks) = spaces.pop() else {
            continue;
        };

        'inner: loop {
            let size = files[right_file];

            if right_file <= left_file {
                return blocks;
            }

            for _ in 0..free_blocks.min(size) {
                blocks.push(right_file);
            }

            if free_blocks > size {
                free_blocks -= size;
                right_file -= 1;
                continue;
            }

            if free_blocks == size {
                right_file -= 1;
            }

            if size > free_blocks {
                files[right_file] = size - free_blocks;
            }

            break 'inner;
        }
    }

    blocks
}

fn parse_input(input: &str) -> MyResult<DiskMap> {
    if input.lines().count() != 1 {
        return Err("Input must be only 1 line".into());
    }

    let mut files = Vec::new();
    let mut free_spaces = Vec::new();

    for (idx, c) in input.chars().enumerate() {
        let size = c.to_digit(10).ok_or("Not a number")?;
        if idx % 2 == 0 {
            files.push(size);
        } else {
            free_spaces.push(size);
        }
    }

    let map = DiskMap {
        files: files,
        free_spaces,
    };

    Ok(map)
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, 1928);
    }
}
