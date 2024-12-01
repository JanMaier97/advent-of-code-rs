use crate::{print_challenge_header, MyResult};

mod part_1;

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challenge_header(1);

    let part_one_result = part_1::solve(INPUT)?;
    println!("Result for part one is {}", part_one_result);

    let part_two_result = solve_part_two(INPUT)?;
    println!("Result for part two is {}", part_two_result);

    Ok(())
}



fn solve_part_two(_input: &str) -> MyResult<usize> {
    unimplemented!();
}

