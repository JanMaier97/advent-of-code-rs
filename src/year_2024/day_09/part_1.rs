use macros::aoc_solver;

use anyhow::{anyhow, bail, Result};

use super::BlockType;

#[aoc_solver(2024, 9, 1, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let mut blocks = parse_input(input)?;
    defragment_disk(&mut blocks);
    let sum: usize = blocks
        .iter()
        .enumerate()
        .map(|(idx, block)| {
            idx * match *block {
                BlockType::File(file_id) => file_id,
                BlockType::Space => 0,
            }
        })
        .sum();

    Ok(sum.to_string())
}

fn defragment_disk(blocks: &mut [BlockType]) {
    let mut right_idx = blocks.len() - 1;

    for left_idx in 0..blocks.len() {
        match blocks[left_idx] {
            BlockType::File(_) => continue,
            BlockType::Space => {
                let Some(idx) = next_file_index_from_behind(blocks, right_idx) else {
                    continue;
                };

                right_idx = idx;

                if left_idx >= right_idx {
                    break;
                }

                blocks[left_idx] = blocks[right_idx];
                blocks[right_idx] = BlockType::Space;
            }
        };
    }
}

fn next_file_index_from_behind(blocks: &[BlockType], current_idx: usize) -> Option<usize> {
    for idx in (0..=current_idx).rev() {
        if let BlockType::File(_) = blocks[idx] {
            return Some(idx);
        }
    }

    None
}

fn parse_input(input: &str) -> Result<Vec<BlockType>> {
    if input.lines().count() != 1 {
        bail!("Input must be only 1 line");
    }

    let mut blocks = Vec::new();

    for (idx, c) in input.chars().enumerate() {
        let size = c.to_digit(10).ok_or(anyhow!("Not a number"))?;
        let block_type = if idx % 2 == 0 {
            BlockType::File(idx / 2)
        } else {
            BlockType::Space
        };

        for _ in 0..size {
            blocks.push(block_type);
        }
    }

    Ok(blocks)
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "1928");
    }
}
