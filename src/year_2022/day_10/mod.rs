use crate::{print_challange_header, MyResult};

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Add(i32),
    Noop,
}

pub fn solve() -> MyResult<()> {
    print_challange_header(10);

    println!(
        "1) The aggregated signal strenth is {}",
        solve_part_one(&INPUT)?
    );
    println!(
        "2) The CTR produces the following image: \n{}",
        solve_part_two(&INPUT)?
    );

    Ok(())
}

struct Cpu {
    cycle: u32,
    register_x: i32,
}

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn from_file(input: &str) -> MyResult<Self> {
        let mut instructions = Vec::new();
        for line in input.lines() {
            if line == "noop" {
                instructions.push(Instruction::Noop);
                continue;
            }

            if line.starts_with("addx ") {
                let Some((_, value)) = line.split_once(' ') else {
                    return Err("invalid instruction".into());
                };

                instructions.push(Instruction::Add(value.parse()?))
            }
        }
        Ok(Program { instructions })
    }
}

impl Cpu {
    fn new() -> Self {
        Cpu {
            cycle: 0,
            register_x: 1,
        }
    }

    fn execute(mut self, program: &Program, mut callback: impl FnMut(&Cpu)) {
        for instruction in program.instructions.iter() {
            self.handle_instruction_cycles(&instruction, &mut callback);
        }
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => {
                // do nothing
            }
            Instruction::Add(value) => {
                self.register_x += value;
            }
        }
    }

    fn handle_instruction_cycles(
        &mut self,
        instruction: &Instruction,
        callback: &mut impl FnMut(&Cpu),
    ) {
        let mut cycles_to_wait = self.get_cycle_count_for_instruction(&instruction);
        loop {
            if cycles_to_wait > 0 {
                cycles_to_wait -= 1;
            }

            self.cycle += 1;

            callback(&self);

            if cycles_to_wait == 0 {
                self.apply_instruction(&instruction);
                break;
            }
        }
    }

    fn get_cycle_count_for_instruction(&self, instruction: &Instruction) -> u32 {
        match instruction {
            Instruction::Noop => 1,
            Instruction::Add(_) => 2,
        }
    }
}

fn solve_part_one(input: &str) -> MyResult<i32> {
    let program = Program::from_file(input)?;
    let cpu = Cpu::new();

    let mut summed_signal_strength = 0;

    cpu.execute(&program, |x| {
        if x.cycle == 20 || (x.cycle > 20 && (x.cycle - 20) % 40 == 0) {
            summed_signal_strength += x.cycle as i32 * x.register_x;
        }
    });

    Ok(summed_signal_strength)
}

fn solve_part_two(input: &str) -> MyResult<String> {
    let program = Program::from_file(input)?;
    let cpu = Cpu::new();

    let mut output = String::new();
    let mut drawing_pos = 0;
    cpu.execute(&program, |cpu| {
        let pixel_positions = cpu.register_x - 1..cpu.register_x + 2;

        if pixel_positions.contains(&drawing_pos) {
            output.push('#');
        } else {
            output.push('.');
        }

        drawing_pos = (drawing_pos + 1) % 40;

        if drawing_pos == 0 {
            output.push('\n');
        }
    });

    Ok(output.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("example.txt");

    #[test]
    fn solve_part_one_example() {
        let result = solve_part_one(EXAMPLE_INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 13140);
    }

    #[test]
    fn solve_part_one_real() {
        let result = solve_part_one(INPUT);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 13720);
    }

    #[test]
    fn solve_part_two_example() {
        let result = solve_part_two(EXAMPLE_INPUT);

        let output = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), output);
    }

    #[test]
    fn solve_part_two_real() {
        let result = solve_part_two(INPUT);

        let output = "####.###..#..#.###..#..#.####..##..#..#.
#....#..#.#..#.#..#.#..#....#.#..#.#..#.
###..###..#..#.#..#.####...#..#....####.
#....#..#.#..#.###..#..#..#...#....#..#.
#....#..#.#..#.#.#..#..#.#....#..#.#..#.
#....###...##..#..#.#..#.####..##..#..#.";
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), output);
    }
}
