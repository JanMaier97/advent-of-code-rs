mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
