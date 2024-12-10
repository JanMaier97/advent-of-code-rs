use crate::MyResult;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");


type BlockSize = u32;
type FileId = usize;

struct DiskMap {
    files: Vec<BlockSize>,
    free_spaces: Vec<BlockSize>,
}

#[derive(Clone, Copy, PartialEq)]
enum BlockType {
    File(FileId),
    Space,
}

struct Block {
    r#type: BlockType,
    size: BlockSize
}

fn parse_input(input: &str) -> MyResult<Vec<Block>> {
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

        blocks.push(Block { r#type: block_type, size});
    }

    Ok(blocks)
}