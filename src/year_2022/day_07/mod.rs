use once_cell::sync::OnceCell;
use regex::Regex;

use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");

static CD_REGEX: OnceCell<Regex> = OnceCell::new();
static CD_UP_REGEX: OnceCell<Regex> = OnceCell::new();
static LS_REGEX: OnceCell<Regex> = OnceCell::new();
static DIR_REGEX: OnceCell<Regex> = OnceCell::new();
static FILE_REGEX: OnceCell<Regex> = OnceCell::new();

const THRESHOLD: u32 = 100000;
const MAX_DISK_SPACE: u32 = 70000000;
const REQUIRED_DISK_SPACE: u32 = 30000000;

#[derive(Debug)]
enum LineTypes {
    DirLabel(String),
    File((String, u32)),
    Dir((String, u32)),
}

pub fn solve() -> MyResult<()> {
    print_challange_header(7);

    println!(
        "The total size of directories exeeding the threshold is {}",
        solve_part_one(INPUT)?
    );

    println!(
        "The size of the file to delete is {}",
        solve_part_two(INPUT)?
    );

    Ok(())
}

fn solve_part_two(input: &str) -> MyResult<u32> {
    let (_, directories, used_disk_space) = solve_function_thats_way_to_big(input)?;
    let unused_disk_space = MAX_DISK_SPACE - used_disk_space;
    let min_dir_size_to_delete = REQUIRED_DISK_SPACE - unused_disk_space;

    let size = directories
        .into_iter()
        .filter(|&size| size > min_dir_size_to_delete)
        .min()
        .expect("could not find file to delete");

    Ok(size)
}

fn solve_part_one(input: &str) -> MyResult<u32> {
    let size = solve_function_thats_way_to_big(input)?.0;
    Ok(size)
}

fn squash_fs_stack(stack: &mut Vec<LineTypes>) -> (String, u32) {
    let mut dir_size = 0;
    loop {
        let stack_top = stack.pop().expect("Stack is empty");
        match stack_top {
            LineTypes::DirLabel(label) => {
                let dir = LineTypes::Dir((label.clone(), dir_size));
                stack.push(dir);
                return (label, dir_size);
            }
            LineTypes::File((_, size)) => {
                dir_size += size;
            }
            LineTypes::Dir((_, size)) => {
                dir_size += size;
            }
        }
    }
}

fn solve_function_thats_way_to_big(input: &str) -> MyResult<(u32, Vec<u32>, u32)> {
    LS_REGEX.get_or_init(|| Regex::new(r"/^\$ ls$").unwrap());

    let file_regex = FILE_REGEX.get_or_init(|| Regex::new(r"^(\d+) (.+)$").unwrap());
    let _dir_regex = DIR_REGEX.get_or_init(|| Regex::new(r"^dir (.)$").unwrap());
    let cd_regex = CD_REGEX.get_or_init(|| Regex::new(r"^\$ cd (\w+|/)$").unwrap());
    let cd_up_regex = CD_UP_REGEX.get_or_init(|| Regex::new(r"^\$ cd \.\.$").unwrap());

    let mut stack = Vec::new();
    let mut directories = Vec::new();
    let mut summed_dir_size = 0;

    for line in input.lines() {
        if let Some(f) = file_regex.captures(&line) {
            let file_size = f.get(1).unwrap().as_str().parse::<u32>()?;
            let file_name = f.get(2).unwrap().as_str();
            stack.push(LineTypes::File((file_name.to_string(), file_size)));
            continue;
        }

        if let Some(_) = file_regex.captures(&line) {
            // let dir_name = d.get(1).unwrap().as_str();
            // stack.push(LineTypes::DirLabel(dir_name.to_string()));
            continue;
        }

        if let Some(d) = cd_regex.captures(&line) {
            let dir_name = d.get(1).unwrap().as_str();
            stack.push(LineTypes::DirLabel(dir_name.to_string()));
            continue;
        }

        if cd_up_regex.is_match(&line) {
            let (_, dir_size) = squash_fs_stack(&mut stack);
            handle_summed_size(&mut summed_dir_size, dir_size);
            directories.push(dir_size);
        }
    }

    while stack.len() > 1 {
        let (_, dir_size) = squash_fs_stack(&mut stack);
        handle_summed_size(&mut summed_dir_size, dir_size);
        directories.push(dir_size);
    }

    match stack.first().unwrap() {
        LineTypes::Dir((_, size)) => Ok((summed_dir_size, directories, *size)),
        _ => Err("invalid rood dir".into()),
    }
}

fn handle_summed_size(total_sum: &mut u32, size: u32) {
    if size <= THRESHOLD {
        *total_sum += size;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn solve_part_one_example() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 95437);
    }

    #[test]
    fn solve_part_one_real() {
        let result = solve_part_one(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1432936);
    }

    #[test]
    fn solve_part_two_example() {
        let result = solve_part_two(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 24933642);
    }

    #[test]
    fn solve_part_two_real() {
        let result = solve_part_two(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 272298);
    }
}
