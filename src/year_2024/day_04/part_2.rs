use macros::aoc_solver;

use anyhow::Result;

#[aoc_solver(2024, 4, 2, super::INPUT)]
fn solve(input: &str) -> Result<String> {
    let input = super::parse_input(input);
    let mut sum = 0;

    for (row, line) in input.iter().enumerate() {
        for (col, _) in line.iter().enumerate() {
            if is_xmas_cross(row, col, &input) {
                sum += 1;
            }
        }
    }

    Ok(sum.to_string())
}

fn is_xmas_cross(row: usize, col: usize, input: &[Vec<char>]) -> bool {
    if input[row][col] != 'A' {
        return false;
    }

    let texts = get_xmas_positions(row, col, input)
        .iter()
        .map(|positions| {
            positions
                .iter()
                .map(|(r, c)| input[*r][*c])
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    if texts.len() != 2 {
        return false;
    }

    texts.iter().all(|t| t == "MAS" || t == "SAM")
}

fn get_xmas_positions(row: usize, col: usize, input: &[Vec<char>]) -> Vec<Vec<(usize, usize)>> {
    let max_row = input.len();
    let max_col = input[row].len();

    if row < 1 || col < 1 || row + 1 >= max_row || col + 1 >= max_col {
        return Vec::new();
    }

    vec![
        vec![(row - 1, col - 1), (row, col), (row + 1, col + 1)],
        vec![(row - 1, col + 1), (row, col), (row + 1, col - 1)],
    ]
}

#[cfg(test)]
mod tests {

    #[test]
    fn solve_example() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "9");
    }
}
