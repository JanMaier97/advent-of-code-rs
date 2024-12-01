use crate::MyResult;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

struct ParsedLocation {
    list1: Vec<u32>,
    list2: Vec<u32>
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