use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");

pub fn solve() -> MyResult<()> {
    print_challange_header(2);
    let total_score_by_move = calculate_total_score_by_move(INPUT)?;
    let total_score_by_result = calculate_total_score_by_round_result(INPUT)?;

    println!("The total score is {}", total_score_by_move);
    println!("The total score is {}", total_score_by_result);
    Ok(())
}

#[derive(Clone, Copy)]
enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum RpsOutcome {
    Draw,
    Win,
    Loss,
}

fn calculate_total_score_by_move(input: &str) -> MyResult<u32> {
    let mut total_score = 0;

    for line in input.lines() {
        let Some((input_p1, input_p2)) = line.split_once(' ') else {
            return Err("Line is malformed".into());
        };

        let opponent_move = parse_rps_move(input_p1)?;
        let my_move = parse_rps_move(input_p2)?;

        total_score += get_score_for_move(my_move)
            + get_score_for_round_outcome(compare_rps_moves(my_move, opponent_move));
    }

    Ok(total_score)
}

fn get_score_for_round_outcome(outcome: RpsOutcome) -> u32 {
    match outcome {
        RpsOutcome::Win => 6,
        RpsOutcome::Draw => 3,
        RpsOutcome::Loss => 0,
    }
}

fn get_score_for_move(rps_move: RpsMove) -> u32 {
    match rps_move {
        RpsMove::Rock => 1,
        RpsMove::Paper => 2,
        RpsMove::Scissors => 3,
    }
}

fn compare_rps_moves(my_move: RpsMove, opponent_move: RpsMove) -> RpsOutcome {
    match my_move {
        RpsMove::Rock => match opponent_move {
            RpsMove::Rock => RpsOutcome::Draw,
            RpsMove::Paper => RpsOutcome::Loss,
            RpsMove::Scissors => RpsOutcome::Win,
        },
        RpsMove::Paper => match opponent_move {
            RpsMove::Rock => RpsOutcome::Win,
            RpsMove::Paper => RpsOutcome::Draw,
            RpsMove::Scissors => RpsOutcome::Loss,
        },
        RpsMove::Scissors => match opponent_move {
            RpsMove::Rock => RpsOutcome::Loss,
            RpsMove::Paper => RpsOutcome::Win,
            RpsMove::Scissors => RpsOutcome::Draw,
        },
    }
}

fn calculate_total_score_by_round_result(input: &str) -> MyResult<u32> {
    let mut total_score = 0;
    for line in input.lines() {
        let Some((opponent_move, expected_outcome)) = line.split_once(' ') else {
            return Err(format!("Input is malformed: {}", line).into());
        };

        let opponent_move = parse_rps_move(opponent_move)?;
        let expected_outcome = parse_rps_outcome(expected_outcome)?;
        let my_move = match expected_outcome {
            RpsOutcome::Draw => opponent_move,
            RpsOutcome::Win => match opponent_move {
                RpsMove::Rock => RpsMove::Paper,
                RpsMove::Paper => RpsMove::Scissors,
                RpsMove::Scissors => RpsMove::Rock,
            },
            RpsOutcome::Loss => match opponent_move {
                RpsMove::Rock => RpsMove::Scissors,
                RpsMove::Paper => RpsMove::Rock,
                RpsMove::Scissors => RpsMove::Paper,
            },
        };

        total_score += get_score_for_move(my_move) + get_score_for_round_outcome(expected_outcome);
    }

    Ok(total_score)
}

fn parse_rps_move(raw_move: &str) -> MyResult<RpsMove> {
    match raw_move {
        "A" | "X" => Ok(RpsMove::Rock),
        "B" | "Y" => Ok(RpsMove::Paper),
        "C" | "Z" => Ok(RpsMove::Scissors),
        _ => Err(format!("Invalid move {}", raw_move).into()),
    }
}

fn parse_rps_outcome(raw_outcome: &str) -> MyResult<RpsOutcome> {
    match raw_outcome {
        "X" => Ok(RpsOutcome::Loss),
        "Y" => Ok(RpsOutcome::Draw),
        "Z" => Ok(RpsOutcome::Win),
        _ => Err(format!("Invalid move {}", raw_outcome).into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn calculate_total_score_by_move_example() {
        let result = calculate_total_score_by_move(EXAMPLE_INPUT);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 15);
    }

    #[test]
    fn calculate_total_score_by_move_real() {
        let result = calculate_total_score_by_move(INPUT);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10404);
    }

    #[test]
    fn calculate_total_score_by_round_result_example() {
        let result = calculate_total_score_by_round_result(EXAMPLE_INPUT);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 12);
    }

    #[test]
    fn calculate_total_score_by_round_result_real() {
        let result = calculate_total_score_by_round_result(INPUT);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 10334);
    }
}
