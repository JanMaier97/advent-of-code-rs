use macros::aoc_solver;

use crate::{year_2024::day_14::parse_input, MyResult};

use super::{Dimensions, Point, Robot, Vec2};

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

fn move_robot(robot: Robot, dim: Dimensions, times: u64) -> MyResult<Point<u64>> {
    let normal_vel = normalize_velocity(robot.velocity, dim);
    let next_pos = robot.pos + normal_vel * times;

    Ok(Point {
        x: next_pos.x % dim.width,
        y: next_pos.y % dim.height,
    })
}

fn normalize_velocity(velocity: Vec2<i32>, dim: Dimensions) -> Vec2<u64> {
    Vec2 {
        x: normalize_value(velocity.x, dim.width),
        y: normalize_value(velocity.y, dim.height),
    }
}

fn normalize_value(value: i32, identity: u64) -> u64 {
    if value < 0 {
        let abs: u64 = value.unsigned_abs().into();
        let times = abs / identity;
        let rem = abs % identity;

        if abs >= identity {
            println!(
                "normalized {} to {}",
                value,
                identity * times + identity - rem
            );
        }

        identity * times + identity - rem
    } else {
        value as u64
    }
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
    use crate::year_2024::day_14::{Dimensions, Point, Robot, Vec2};

    use super::move_robot;

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

    #[test]
    fn robots_move_with_wrapping() {
        let robot = Robot {
            pos: Point { x: 2, y: 4 },
            velocity: Vec2 { x: 2, y: -3 },
        };
        let dim = Dimensions {
            height: 7,
            width: 11,
        };

        let pos = move_robot(robot, dim, 1).unwrap();
        assert_eq!(pos, Point { x: 4, y: 1 });

        let pos = move_robot(robot, dim, 2).unwrap();
        assert_eq!(pos, Point { x: 6, y: 5 });

        let pos = move_robot(robot, dim, 3).unwrap();
        assert_eq!(pos, Point { x: 8, y: 2 });

        let pos = move_robot(robot, dim, 4).unwrap();
        assert_eq!(pos, Point { x: 10, y: 6 });

        let pos = move_robot(robot, dim, 5).unwrap();
        assert_eq!(pos, Point { x: 1, y: 3 });
    }
}
