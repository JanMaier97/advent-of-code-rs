use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challange_header(4);

    println!(
        "Number of pairs where one fully contains the other: {}",
        calculate_solution_part_one(INPUT)?
    );
    println!(
        "Number of pairs that overlap: {}",
        calculate_solution_part_two(INPUT)?
    );

    Ok(())
}

fn calculate_solution_part_one(input: &str) -> MyResult<u32> {
    let mut count = 0;
    for line in input.lines() {
        let (range_1, range_2) = parse_range_pair(&line)?;

        if (range_1.0 >= range_2.0 && range_1.1 <= range_2.1)
            || (range_2.0 >= range_1.0 && range_2.1 <= range_1.1)
        {
            count += 1;
        }
    }

    Ok(count)
}

fn calculate_solution_part_two(input: &str) -> MyResult<u32> {
    let mut count = 0;
    for line in input.lines() {
        let (range_1, range_2) = parse_range_pair(&line)?;

        if range_1.0 < range_2.0 && range_1.1 < range_2.0 {
            continue;
        }

        if range_2.0 < range_1.0 && range_2.1 < range_1.0 {
            continue;
        }

        count += 1;
    }

    Ok(count)
}

fn parse_range_pair(input: &str) -> MyResult<((u32, u32), (u32, u32))> {
    let Some((range_1, range_2)) = input.split_once(',') else {
            return Err("Invalid format".into());
        };

    let range_1 = parse_range(range_1)?;
    let range_2 = parse_range(range_2)?;

    Ok((range_1, range_2))
}

fn parse_range(range: &str) -> MyResult<(u32, u32)> {
    let Some((lower_bound, upper_bound)) = range.split_once('-') else {
            return Err("Invalid range".into());
        };

    Ok((lower_bound.parse()?, upper_bound.parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn calculate_solution_part_one_example() {
        let result = calculate_solution_part_one(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn calculate_solution_part_one_real() {
        let result = calculate_solution_part_one(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 644);
    }

    #[test]
    fn calculate_solution_part_two_solution() {
        let result = calculate_solution_part_two(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 4);
    }
}
