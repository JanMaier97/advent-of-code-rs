mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

type BlockSize = u32;
type FileId = usize;

#[derive(Clone, Copy, PartialEq, Debug)]
enum BlockType {
    File(FileId),
    Space,
}

#[derive(Clone, Copy, Debug)]
struct Block {
    r#type: BlockType,
    size: BlockSize,
}
