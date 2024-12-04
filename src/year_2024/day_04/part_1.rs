use linkme::distributed_slice;
use num::range_step;

use crate::{MyResult, SolverMetadata, SOLVERS};

#[distributed_slice(SOLVERS)]
static SOLVER: SolverMetadata<'static> = SolverMetadata {
    year: 2024,
    day: 4,
    part: 1,
    func: solve,
    input: super::INPUT,
};

fn solve(input: &str) -> MyResult<u32> {
    let input = super::parse_input(input);

    let mut sum = 0;
    for (row, line) in input.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char == 'X' {
                sum += count2(row, col, &input);
            }
        }
    }

    Ok(sum)
}

fn count2(row: usize, col: usize, input: &[Vec<char>]) -> u32 {
    let max_row = input.len();
    let max_col = input[row].len();
    let positions = generate_positions(row, col, max_row, max_col);

    let snippets = positions
        .iter()
        .map(|char_positions| {
            char_positions
                .iter()
                .map(|(r, c)| input[*r][*c])
                .collect::<String>()
        })
        .collect::<Vec<_>>();

    snippets
        .iter()
        .filter(|txt| *txt == "XMAS" || *txt == "SAMX")
        .count() as u32
}

fn generate_positions(
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
) -> Vec<Vec<(usize, usize)>> {
    let row = row as i32;
    let col = col as i32;
    let max_row = max_row as i32;
    let max_col = max_col as i32;

    let mut positions = generate_straight_positions(row, col);
    positions.extend_from_slice(&generate_diagonals(row, col));

    positions
        .iter()
        .filter(|positions| {
            positions
                .iter()
                .all(|(r, c)| *r >= 0 && *c >= 0 && *r < max_row && *c < max_col)
        })
        .map(|positions| {
            positions
                .iter()
                .map(|(r, c)| (*r as usize, *c as usize))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn generate_straight_positions(row: i32, col: i32) -> Vec<Vec<(i32, i32)>> {
    let mut positions = vec![(col..col + 4).map(|c| (row, c)).collect::<Vec<_>>()];
    positions.push((row..row + 4).map(|r| (r, col)).collect::<Vec<_>>());
    positions.push(
        range_step(col, col - 4, -1)
            .map(|c| (row, c))
            .collect::<Vec<_>>(),
    );
    positions.push(
        range_step(row, row - 4, -1)
            .map(|r| (r, col))
            .collect::<Vec<_>>(),
    );

    positions
}

fn generate_diagonals(row: i32, col: i32) -> Vec<Vec<(i32, i32)>> {
    let row_directions = [1, -1];
    let col_directions = [1, -1];
    let mut diagonals = Vec::new();

    for row_dir in row_directions {
        for col_dir in col_directions {
            let row_range = range_step(row, row + 4 * row_dir, row_dir);
            let col_range = range_step(col, col + 4 * col_dir, col_dir);
            let positions = row_range.zip(col_range).collect::<Vec<_>>();
            diagonals.push(positions);
        }
    }

    diagonals
}

#[cfg(test)]
mod tests {
    const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn solve_example() {
        let result = super::solve(EXAMPLE).unwrap();
        assert_eq!(result, 18);
    }

    #[test]
    fn generate_diagonals_correctly() {
        let result = super::generate_diagonals(0, 0);
        let expected = vec![
            vec![(0, 0), (1, 1), (2, 2), (3, 3)],
            vec![(0, 0), (1, -1), (2, -2), (3, -3)],
            vec![(0, 0), (-1, 1), (-2, 2), (-3, 3)],
            vec![(0, 0), (-1, -1), (-2, -2), (-3, -3)],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn generate_straights_correctly() {
        let result = super::generate_straight_positions(0, 0);
        let expected = vec![
            vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            vec![(0, 0), (0, -1), (0, -2), (0, -3)],
            vec![(0, 0), (-1, 0), (-2, 0), (-3, 0)],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn generates_positions_correclty() {
        let result = super::generate_positions(9, 9, 10, 10);
        let expected = vec![
            vec![(9, 9), (9, 8), (9, 7), (9, 6)],
            vec![(9, 9), (8, 9), (7, 9), (6, 9)],
            vec![(9, 9), (8, 8), (7, 7), (6, 6)],
        ];
        assert_eq!(result, expected);
    }
}
