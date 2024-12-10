use macros::aoc_solver;

use crate::MyResult;

use super::BlockType;


#[aoc_solver(2024, 9, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let mut blocks = parse_input(input)?;
    defragment_disk(&mut blocks);
    let sum: usize = blocks
        .iter()
        .enumerate()
        .map(|(idx, block)| idx * match *block {
            BlockType::File(file_id) => file_id,
            BlockType::Space => 0,
        })
        .sum();

    let sum = u64::try_from(sum)?;

    Ok(sum)
}

fn defragment_disk(blocks: &mut Vec<BlockType>) {
    let mut right_idx = blocks.len()-1;

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
            },
        };
    }
}

fn next_file_index_from_behind(blocks: &Vec<BlockType>, current_idx: usize) -> Option<usize> {
    for idx in (0..=current_idx).rev(){
        if let BlockType::File(_) = blocks[idx] {
            return Some(idx);
        }
    }

    None
}

fn parse_input(input: &str) -> MyResult<Vec<BlockType>> {
    if input.lines().count() != 1 {
        return Err("Input must be only 1 line".into());
    }

    let mut blocks = Vec::new();

    for (idx, c) in input.chars().enumerate() {
        let size = c.to_digit(10).ok_or("Not a number")?;
        let block_type: BlockType; 
        if idx % 2 == 0 {
            block_type = BlockType::File(idx/2);
        } else {
            block_type = BlockType::Space;
        }

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
        assert_eq!(result, 1928);
    }
}
