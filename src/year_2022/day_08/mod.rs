use itertools::Itertools;

use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");

struct Grid<T> {
    pub columns: usize,
    pub rows: usize,
    pub items: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    fn new(items: Vec<Vec<T>>) -> MyResult<Self> {
        let rows = items.len();
        let columns = items.first().map_or(0, |line| line.len());

        for line in &items {
            if line.len() != columns {
                return Err("Inner lists are not of the same size".into());
            }
        }

        Ok(Grid {
            columns,
            rows,
            items,
        })
    }
}

pub fn solve() -> MyResult<()> {
    print_challange_header(8);

    println!(
        "The number of visible trees is {}",
        solve_first_part(INPUT)?
    );
    println!("The highest scenic score is {}", solve_second_part(INPUT)?);

    Ok(())
}

fn solve_first_part(input: &str) -> MyResult<u32> {
    let grid = load_tree_grid(input)?;
    let mut visible_field_count = 0;

    for (row_idx, row) in grid.items.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            if field_is_visible(&grid, row_idx, col_idx)? {
                visible_field_count += 1;
            }
        }
    }

    Ok(visible_field_count)
}

fn solve_second_part(input: &str) -> MyResult<u32> {
    let grid = load_tree_grid(input)?;
    let mut scores = Vec::new();

    for (row_idx, row) in grid.items.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            scores.push(compute_view_score_for_tree(&grid, row_idx, col_idx)?);
        }
    }

    Ok(*scores.iter().max().unwrap())
}

fn compute_view_score_for_tree(grid: &Grid<u32>, row_idx: usize, col_idx: usize) -> MyResult<u32> {
    let Some(row) = grid.items.get(row_idx) else {
        return Err("invalid row index".into());
    };

    let Some(tree_size) = row.get(col_idx) else {
        return Err("invalid column index".into());
    };

    // let mut visible_trees = Vec::new();

    // top of field
    let top_count = grid.items[0..row_idx]
        .iter()
        .map(|row| row[col_idx])
        .rev()
        .enumerate()
        .find(|&(_, size)| size >= *tree_size)
        .map_or_else(|| row_idx, |(idx, _)| idx + 1)
        .clamp(0, usize::MAX);

    let left_count = row[0..col_idx]
        .iter()
        .rev()
        .enumerate()
        .find(|&(_, size)| size >= tree_size)
        .map_or_else(|| col_idx, |(idx, _)| idx + 1)
        .clamp(0, usize::MAX);

    let bottom_count = grid.items[row_idx + 1..]
        .iter()
        .map(|row| row[col_idx])
        .enumerate()
        .find(|&(_, size)| size >= *tree_size)
        .map_or_else(|| grid.rows - row_idx - 1, |(idx, _)| idx + 1)
        .clamp(0, usize::MAX);

    let right_count = row[col_idx + 1..]
        .iter()
        .enumerate()
        .find(|&(_, size)| size >= tree_size)
        .map_or_else(|| grid.columns - 1 - col_idx, |(idx, _)| idx + 1)
        .clamp(0, usize::MAX);

    Ok((top_count * left_count * bottom_count * right_count) as u32)
}

fn field_is_visible(grid: &Grid<u32>, row_idx: usize, col_idx: usize) -> MyResult<bool> {
    let Some(row) = grid.items.get(row_idx) else {
        return Err("invalid row index".into());
    };

    let Some(tree_size) = row.get(col_idx) else {
        return Err("invalid column index".into());
    };

    // fields on the edge are always visible
    if row_idx == 0 || row_idx == grid.rows - 1 || col_idx == 0 || col_idx == grid.columns - 1 {
        return Ok(true);
    }

    let is_visible = row[0..col_idx].iter().all(|c| c < tree_size)
        || row[col_idx + 1..].iter().all(|c| c < tree_size)
        || grid.items[0..row_idx]
            .iter()
            .all(|row| row[col_idx] < *tree_size)
        || grid.items[row_idx + 1..]
            .iter()
            .all(|row| row[col_idx] < *tree_size);

    Ok(is_visible)
}

fn load_tree_grid(input: &str) -> MyResult<Grid<u32>> {
    let mut parsed_content = Vec::new();
    for line in input.lines() {
        let parsed_line = line
            .chars()
            .map(|c| c.to_string().parse::<u32>())
            .try_collect()?;
        parsed_content.push(parsed_line);
    }

    Grid::new(parsed_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn solve_first_part_example() {
        let result = solve_first_part(EXAMPLE_INPUT);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 21);
    }

    #[test]
    fn solve_first_part_real() {
        let result = solve_first_part(INPUT);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1713);
    }

    #[test]
    fn solve_second_part_example() {
        let result = solve_second_part(EXAMPLE_INPUT);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn solve_second_part_real() {
        let result = solve_second_part(INPUT);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 268464);
    }
}
