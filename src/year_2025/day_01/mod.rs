use macros::aoc_solver;

use anyhow::Result;

static INPUT: &str = include_str!("input.txt");

#[aoc_solver(2025, 1, 1, INPUT)]
pub fn solve(input: &str) -> Result<String> {
    let mut current_pos: i32 = 50;
    let mut zero_count = 0;
    for line in input.lines() {
        let (dir, distance) = parse(line)?;
        let dir = if dir == 'L' { -1 } else { 1 };

        current_pos += dir * (distance % 100);

        if current_pos < 0 {
            current_pos += 100;
        }
        if current_pos > 99 {
            current_pos -= 100;
        }

        if current_pos == 0 {
            zero_count += 1;
        }
    }

    return Ok(zero_count.to_string());
}

#[aoc_solver(2025, 1, 2, INPUT)]
pub fn solve_part_2(input: &str) -> Result<String> {
    let mut current_pos: i32 = 50;
    let mut zero_count = 0;
    for line in input.lines() {
        let (dir, distance) = parse(line)?;
        let dir = if dir == 'L' { -1 } else { 1 };
        let prev_pos = current_pos;
        let full_rotations = distance / 100;
        let remainder = distance % 100;

        current_pos += dir * remainder;
        zero_count += full_rotations;

        if current_pos < 0 {
            current_pos += 100;
            if prev_pos != 0 {
                zero_count += 1;
            }
        } else if current_pos > 99 {
            current_pos -= 100;

            if prev_pos != 0 {
                zero_count += 1;
            }
        } else if current_pos == 0 && prev_pos != 0 {
            zero_count += 1;
        }
    }

    return Ok(zero_count.to_string());
}

fn parse(line: &str) -> Result<(char, i32)> {
    let dir = line.chars().next().unwrap();
    let num_str = line.chars().skip(1).collect::<String>();
    let num = num_str.parse::<i32>()?;
    return Ok((dir, num));
}

#[cfg(test)]
mod tests {
    #[test]
    fn solve_example_part_1() {
        let result = super::solve(include_str!("example.txt")).unwrap();
        assert_eq!(result, "3");
    }

    #[test]
    fn solve_example_part_2() {
        let result = super::solve_part_2(include_str!("example.txt")).unwrap();
        assert_eq!(result, "6");
    }

    #[test]
    fn solve_part_2() {
        let result = super::solve_part_2(super::INPUT).unwrap();
        assert_eq!(result, "5978");
    }

    #[test]
    fn solve_example_part_2_single_line() {
        let result = super::solve_part_2("R1000").unwrap();
        assert_eq!(result, "10");
    }

    #[test]
    fn solve_day_01() {
        let result = super::solve(super::INPUT).unwrap();
        assert_eq!(result, "997");
    }
}
