use itertools::Itertools;
use num::{integer::Roots, Float};

use crate::{print_challenge_header, run, MyResult};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

pub fn solve() -> MyResult<()> {
    print_challenge_header(6);

    println!(
        "The product of all win possibilities is {}",
        solve_part_one(INPUT)
    );

    Ok(())
}

fn solve_part_one(input: &str) -> u32 {
    let races = parse_input(input);

    let res = races
        .iter()
        .map(|r| compute_min_and_max_button_duration(r))
        .map(|(min, max)| max - min + 1)
        .product();

    res
}

fn solve_part_two(input: &str) -> u32 {
    unimplemented!()
}

fn compute_min_and_max_button_duration(race: &Race) -> (u32, u32) {
    let time = race.time as f32;
    let distance = race.distance as f32;

    let offset = (((time * time) / 4.) - distance).sqrt();
    let prefix = time / 2.;

    let min = prefix - offset + 1.;
    let max = prefix + offset - 1.;

    (min.floor() as u32, max.ceil() as u32)
}

fn parse_input(input: &str) -> Vec<Race> {
    let x = input
        .lines()
        .into_iter()
        .map(|l| {
            l.split(' ')
                .skip(1)
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<u32>())
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
    use crate::year_2023::day_06::INPUT;

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
}