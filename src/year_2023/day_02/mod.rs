use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

type GameSet = Vec<RevealedCube>;

#[derive(Debug)]
struct RevealedCube {
    count: usize,
    color: Color,
}

struct Game {
    id: usize,
    sets: Vec<GameSet>,
}

pub fn solve() -> MyResult<()> {
    print_challange_header(2);
    println!("The sum of ids is: {}", solve_part_one(INPUT)?);
    println!("The sum of game powers is: {}", solve_part_two(INPUT)?);

    Ok(())
}

fn solve_part_one(input: &str) -> MyResult<usize> {
    let games = parse_input(input)?;

    let sum = games
        .iter()
        .filter(|game| game_is_possible(12, 13, 14, &game))
        .map(|game| game.id)
        .sum();

    return Ok(sum);
}

fn solve_part_two(input: &str) -> MyResult<usize> {
    let games = parse_input(input)?;
    let mut total_power = 0;

    for game in games {
        let red_power = get_power_for_color(&game, Color::Red);
        let green_power = get_power_for_color(&game, Color::Green);
        let blue_power = get_power_for_color(&game, Color::Blue);

        total_power += blue_power * green_power * red_power;
    }

    return Ok(total_power);
}

fn get_power_for_color(game: &Game, color: Color) -> usize {
    game.sets
        .iter()
        .flatten()
        .filter(|g| g.color == color)
        .map(|g| g.count)
        .max()
        .unwrap()
}

fn game_is_possible(red_count: usize, green_count: usize, blue_count: usize, game: &Game) -> bool {
    for cube in game.sets.iter().flatten() {
        let max_count = match cube.color {
            Color::Red => red_count,
            Color::Green => green_count,
            Color::Blue => blue_count,
        };

        if cube.count > max_count {
            return false;
        }
    }

    return true;
}

fn parse_input(input: &str) -> MyResult<Vec<Game>> {
    let mut games = Vec::new();

    for (line_index, line) in input.lines().enumerate() {
        let line = line;
        let game_id = line_index + 1;
        let mut sets = Vec::new();

        let raw_game = line.replace(&format!("Game {}: ", game_id), "");
        let raw_sets = raw_game.split("; ");
        for raw_set in raw_sets {
            let mut game_set: GameSet = Vec::new();
            let raw_cubes = raw_set.split(", ");

            for raw_cube in raw_cubes {
                let (raw_count, raw_color) = raw_cube.split_once(" ").unwrap();

                let count = raw_count.parse::<usize>().unwrap();
                let color = match raw_color {
                    "red" => Color::Red,
                    "blue" => Color::Blue,
                    "green" => Color::Green,
                    _ => unimplemented!(),
                };

                game_set.push(RevealedCube { count, color });
            }

            sets.push(game_set);
        }

        games.push(Game {
            id: line_index + 1,
            sets,
        });
    }

    Ok(games)
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day_02::{solve_part_two, INPUT};

    use super::solve_part_one;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn example_part_one_correct() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn input_part_one_correct() {
        let result = solve_part_one(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2348);
    }

    #[test]
    fn example_part_two_correct() {
        let result = solve_part_two(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2286);
    }

    #[test]
    fn input_part_two_correct() {
        let result = solve_part_two(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 76008);
    }
}
