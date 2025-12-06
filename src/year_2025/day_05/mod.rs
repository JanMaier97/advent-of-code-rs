use std::collections::HashSet;

use macros::aoc_solver;

use anyhow::{anyhow, Result};

static INPUT: &str = include_str!("input.txt");

struct Range {
    start: u64,
    end: u64,
}

struct IngredientData {
    fresh_ingredients: Vec<Range>,
    ingredients: HashSet<u64>,
}

#[aoc_solver(2025, 5, 1, INPUT)]
fn solve_part_1(input: &str) -> Result<String> {
    let data = parse_input(input)?;

    let count = data
        .ingredients
        .into_iter()
        .filter(|id| is_fresh(*id, &data.fresh_ingredients))
        .count();

    return Ok(count.to_string());
}

#[aoc_solver(2025, 5, 2, INPUT)]
fn solve_part_2(input: &str) -> Result<String> {
    let data = parse_input(input)?;
    let count = count_ranges(&data.fresh_ingredients);
    Ok(count.to_string())
}

fn count_ranges(ranges: &[Range]) -> u64 {
    let mut count = 0;
    let mut highest_id = 0;

    for range in ranges {
        if range.end <= highest_id {
            continue;
        }

        let start = range.start.max(highest_id + 1);
        count += range.end - start + 1;

        highest_id = range.end;
    }

    return count;
}

fn is_fresh(ingredient: u64, fresh_ingredients: &[Range]) -> bool {
    for range in fresh_ingredients {
        if ingredient < range.start {
            return false;
        }

        if ingredient <= range.end {
            return true;
        }
    }

    return false;
}

fn parse_input(input: &str) -> Result<IngredientData> {
    let mut fresh_ingredients = Vec::new();
    let mut ingredients = HashSet::new();

    let mut lines = input.lines().into_iter();
    loop {
        let line = lines
            .next()
            .ok_or(anyhow!("Input invalid: missing ingredients"))?;
        if line == "" {
            break;
        }
        let mut nums = line.split('-');
        let lower: u64 = nums
            .next()
            .ok_or(anyhow!("Could not parse range: {}", line))?
            .parse::<u64>()?;
        let upper = nums
            .next()
            .ok_or(anyhow!("Could not parse range: {}", line))?
            .parse::<u64>()?;

        fresh_ingredients.push(Range {
            start: lower,
            end: upper,
        });
    }

    fresh_ingredients.sort_by(|a, b| a.start.cmp(&b.start));

    for line in lines {
        let id = line.parse::<u64>()?;
        ingredients.insert(id);
    }

    Ok(IngredientData {
        fresh_ingredients,
        ingredients,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve_part_1(include_str!("example.txt")).unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn solve_example_part_2() {
        let result = super::solve_part_2(include_str!("example.txt")).unwrap();
        assert_eq!(result, "14");
    }

    #[test]
    fn solve_part_1() {
        let result = super::solve_part_1(super::INPUT).unwrap();
        assert_eq!(result, "623");
    }

    #[test]
    fn solve_part_2() {
        let result = super::solve_part_2(super::INPUT).unwrap();
        assert_eq!(result, "353507173555373");
    }
}
