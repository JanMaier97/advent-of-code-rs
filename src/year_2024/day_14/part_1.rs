use macros::aoc_solver;

use crate::{year_2024::day_14::parse_input, MyResult};

use super::{move_robot, Dimensions, Point};

#[aoc_solver(2024, 14, 1, super::INPUT)]
fn solve(input: &str) -> MyResult<u64> {
    let dim = Dimensions {
        height: 103,
        width: 101,
    };
    solve_with_input(input, dim, 100)
}

fn solve_with_input(input: &str, dim: Dimensions, times: u64) -> MyResult<u64> {
    let robots = parse_input(input)?;
    let points = robots
        .iter()
        .map(|r| move_robot(*r, dim, times))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(count_positions(&points, dim).try_into()?)
}

fn count_positions(poinst: &[Point<u64>], dim: Dimensions) -> usize {
    let middle_x = dim.width / 2;
    let middle_y = dim.height / 2;

    let top_left = poinst
        .iter()
        .filter(|r| r.x < middle_x && r.y < middle_y)
        .count();
    let top_right = poinst
        .iter()
        .filter(|r| r.x > middle_x && r.y < middle_y)
        .count();
    let bottom_right = poinst
        .iter()
        .filter(|r| r.x > middle_x && r.y > middle_y)
        .count();
    let bottom_left = poinst
        .iter()
        .filter(|r| r.x < middle_x && r.y > middle_y)
        .count();

    top_left * top_right * bottom_left * bottom_right
}

#[cfg(test)]
mod tests {
    use crate::year_2024::day_14::Dimensions;

    #[test]
    fn solve_example() {
        let dim = Dimensions {
            height: 7,
            width: 11,
        };
        let result = super::solve_with_input(include_str!("example.txt"), dim, 100).unwrap();
        assert_eq!(result, 12);
    }

    #[test]
    fn solve_for_single_moving_robot() {
        let dim = Dimensions {
            height: 7,
            width: 11,
        };
        let input = "p=0,4 v=3,-3\np=0,0 v=0,0\np=0,6 v=0,0\np=10,0 v=0,0\np=10,6 v=0,0";
        let result = super::solve_with_input(input, dim, 5).unwrap();
        assert_eq!(result, 1);
    }
}
