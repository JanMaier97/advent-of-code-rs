use itertools::Itertools;
use num::{integer::Roots, Float};

use crate::{print_challenge_header, run, MyResult};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(6);

    println!(
        "The product of all win possibilities is {}",
        solve_part_one(INPUT)
    );

    println!(
        "The product of win possibilities for the actual race is {}",
        solve_part_two(INPUT)
    );

    Ok(())
}

fn solve_part_one(input: &str) -> u64 {
    let races = parse_input(input);

    let res = races
        .iter()
        .map(|r| compute_min_and_max_button_duration(r))
        .map(|(min, max)| max - min + 1)
        .product();

    res
}

fn solve_part_two(input: &str) -> u64 {
    let race = parse_input_as_single_race(input);
    println!("time: {}, distance: {}", race.time, race.distance);

    let (min, max) = compute_min_and_max_button_duration(&race);

    println!("min: {}, max: {}", min, max);

    max - min + 1
}

fn parse_input_as_single_race(input: &str) -> Race {
    let x = input
        .lines()
        .into_iter()
        .map(|l| l.split(' ').skip(1).join("").parse::<u64>().unwrap())
        .collect_vec();

    assert_eq!(x.len(), 2);

    Race {
        time: x[0],
        distance: x[1],
    }
}

fn compute_min_and_max_button_duration(race: &Race) -> (u64, u64) {
    let time = race.time as f64;
    let distance = race.distance as f64;

    let offset = (((time * time) / 4.) - distance).sqrt();
    let prefix = time / 2.;

    let min = prefix - offset + 1.;
    let max = prefix + offset - 1.;

    (min.floor() as u64, max.ceil() as u64)
}

fn parse_input(input: &str) -> Vec<Race> {
    let x = input
        .lines()
        .into_iter()
        .map(|l| {
            l.split(' ')
                .skip(1)
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u64>())
                .flatten()
                .collect_vec()
        })
        .collect_vec();

    assert_eq!(x.len(), 2);

    x[0].iter()
        .zip(&x[1])
        .map(|(&time, &distance)| Race { time, distance })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day_06::{solve_part_two, INPUT};

    use super::solve_part_one;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn example_input_part_one_solved_correctly() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert_eq!(result, 288);
    }

    #[test]
    fn real_input_part_one_solved_correctly() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 1413720);
    }

    #[test]
    fn example_input_part_two_solved_correctly() {
        let result = solve_part_two(EXAMPLE_INPUT);
        assert_eq!(result, 71503);
    }

    #[test]
    fn real_input_part_two_solved_correctly() {
        let result = solve_part_two(INPUT);
        assert_eq!(result, 30565288);
    }
}
