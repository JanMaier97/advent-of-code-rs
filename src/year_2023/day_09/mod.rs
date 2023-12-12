use itertools::Itertools;

use crate::{print_challenge_header, MyResult};

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challenge_header(8);
    println!(
        "The sum of extrapolated values is {}",
        solve_part_one(INPUT)
    );
    println!(
        "The sum of inverted extrapolated values is {}",
        solve_part_two(INPUT)
    );

    Ok(())
}

fn solve_part_one(input: &str) -> i32 {
    let histories = parse_input(input);
    extrapolate_and_sum(&histories)
}

fn extrapolate_and_sum(histories: &[Vec<i32>]) -> i32 {
    let mut total_sum = 0;
    for history in histories {
        let mut extrapolated_sum = 0;
        let interpolated_values = interpolate(history);

        for value in interpolated_values {
            extrapolated_sum = extrapolated_sum + value;
        }

        total_sum += extrapolated_sum
    }

    total_sum
}

fn solve_part_two(input: &str) -> i32 {
    let histories = parse_input(input);

    let inverted_histories = histories
        .into_iter()
        .map(|h| h.into_iter().rev().collect_vec())
        .collect_vec();

    extrapolate_and_sum(&inverted_histories)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn interpolate(history: &[i32]) -> Vec<i32> {
    let mut interpolated_histories: Vec<Vec<i32>> = Vec::new();

    let mut previous_history = history;
    loop {
        let mut current_history = Vec::new();
        for window in previous_history.windows(2) {
            current_history.push(window[1] - window[0]);
        }

        if current_history.iter().all(|value| *value == 0) {
            break;
        }

        interpolated_histories.push(current_history);
        previous_history = &interpolated_histories.last().unwrap();
    }

    println!("{}", history.iter().map(|v| v.to_string()).join(" "));
    interpolated_histories
        .iter()
        .for_each(|h| println!("{}", h.iter().map(|v| v.to_string()).join(" ")));

    let last_interpolated_values = interpolated_histories
        .iter()
        .map(|history| *history.last().unwrap());

    let mut last_values = Vec::from_iter(last_interpolated_values.rev());
    last_values.push(*history.last().unwrap());

    last_values
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day_09::{solve_part_two, INPUT};

    use super::solve_part_one;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn solve_example_part_one_correctly() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert_eq!(result, 114);
    }

    #[test]
    fn solve_real_part_one_correctly() {
        let result = solve_part_one(INPUT);
        assert_eq!(result, 1916822650);
    }

    #[test]
    fn solve_example_part_two_correctly() {
        let result = solve_part_two(EXAMPLE_INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn solve_real_part_two_correctly() {
        let result = solve_part_two(INPUT);
        assert_eq!(result, 966);
    }
}
