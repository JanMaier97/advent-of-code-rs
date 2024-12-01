use crate::{print_challenge_header, MyResult};

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

struct ParsedLocation {
    list1: Vec<u32>,
    list2: Vec<u32>
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(1);

    let part_one_result = part_1::solve(INPUT)?;
    println!("Result for part one is {}", part_one_result);

    let part_two_result = part_2::solve(INPUT)?;
    println!("Result for part two is {}", part_two_result);

    Ok(())
}

fn parse_input(input: &str) -> MyResult<ParsedLocation> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for (idx, line) in input.lines().enumerate() {
        let split = line
            .split("   ")
            .map(|elem| elem.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        if split.len() != 2 {
            return Err(format!("Invalid input at line {}", idx + 1).into());
        }

        list1.push(split[0]);
        list2.push(split[1]);
    }
    
    Ok(ParsedLocation {list1, list2})
}