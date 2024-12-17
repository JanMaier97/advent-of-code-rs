use macros::aoc_solver;

use anyhow::{anyhow, bail, Result};

use super::{Block, BlockType};

#[aoc_solver(2024, 9, 2, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let mut blocks = parse_input(input)?;
    defragment_disk(&mut blocks);

    let sum: usize = blocks
        .iter()
        .flat_map(|block| vec![block.r#type; block.size as usize])
        .enumerate()
        .filter_map(|(idx, block_type)| match block_type {
            BlockType::File(id) => Some((idx, id)),
            BlockType::Space => None,
        })
        .map(|(idx, file_id)| idx * file_id)
        .sum();

    Ok(sum.to_string())
}

fn defragment_disk(blocks: &mut Vec<Block>) {
    for current_file_id in (0..=blocks.len() / 2).rev() {
        let Some((file_idx_to_move, &file_block)) =
            blocks
                .iter()
                .enumerate()
                .rev()
                .find(|(_, block)| match block.r#type {
                    BlockType::File(id) => id == current_file_id,
                    BlockType::Space => false,
                })
        else {
            break;
        };

        let Some((space_idx_to_move, &space_block)) = blocks
            .iter()
            .take(file_idx_to_move)
            .enumerate()
            .find(|(_, block)| match block.r#type {
                BlockType::File(_) => false,
                BlockType::Space => block.size >= file_block.size,
            })
        else {
            continue;
        };

        blocks[space_idx_to_move] = file_block;
        blocks[file_idx_to_move] = Block {
            r#type: BlockType::Space,
            size: file_block.size,
        };

        if space_block.size > file_block.size {
            let new_space = Block {
                r#type: BlockType::Space,
                size: space_block.size - file_block.size,
            };
            blocks.insert(space_idx_to_move + 1, new_space);
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<Block>> {
    if input.lines().count() != 1 {
        bail!("Input must be only 1 line");
    }

    let mut blocks = Vec::new();

    for (idx, c) in input.chars().enumerate() {
        let size = c.to_digit(10).ok_or(anyhow!("Not a number"))?;
        let block_type: BlockType = if idx % 2 == 0 {
            BlockType::File(idx / 2)
        } else {
            BlockType::Space
        };

        blocks.push(Block {
            r#type: block_type,
            size,
        });
    }

    Ok(blocks)
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "2858");
    }
}
