use std::{collections::HashMap, error::Error};

use linkme::distributed_slice;

mod cli;
pub mod common;
mod year_2022;
pub mod year_2023;
mod year_2024;
mod year_2025;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct ExecutionArgs {
    pub year: u16,
    pub day: u8,
    pub part: u8,
}

pub fn run() -> MyResult<()> {
    let args = cli::parse_args()?;

    let solvers = collect_solver_map()?;

    let date = SolverDate {
        year: args.year,
        day: args.day,
        part: args.part,
    };
    let Some(solver) = solvers.get(&date) else {
        return Err(format!(
            "No solution for year {} day {:02} part {} exists yet",
            args.year, args.day, args.part
        )
        .into());
    };

    let solution = (solver.func)(solver.input)?;

    println!(
        "Solution for year {} day {:02} part {}: {}",
        date.year, date.day, date.part, solution
    );

    Ok(())
}

fn collect_solver_map() -> MyResult<HashMap<SolverDate, SolverData<'static>>> {
    let mut map = HashMap::new();
    for solver in SOLVERS {
        let date = SolverDate {
            year: solver.year,
            day: solver.day,
            part: solver.part,
        };
        if map.contains_key(&date) {
            return Err(format!(
                "Found duplicate solver entry for Year {} Day {:02} Part {}",
                date.year, date.day, date.part
            )
            .into());
        }

        let data = SolverData {
            func: solver.func,
            input: solver.input,
        };

        map.insert(date, data);
    }

    Ok(map)
}

fn print_challenge_header(day: usize) {
    let day_label = format!("Day {:02}", day);
    let horizontal_border = "#".repeat(50);

    if day > 1 {
        println!("\n");
    }
    println!("{}", horizontal_border);
    println!("##{:^46}##", day_label);
    println!("{}", horizontal_border);
    println!();
}

type SolverFunc = fn(&str) -> anyhow::Result<String>;

#[derive(Hash, Eq, PartialEq)]
struct SolverDate {
    year: u16,
    day: u8,
    part: u8,
}

struct SolverData<'a> {
    func: SolverFunc,
    input: &'a str,
}

#[distributed_slice]
pub static SOLVERS: [SolverMetadata<'static>];

struct SolverMetadata<'a> {
    year: u16,
    day: u8,
    part: u8,
    func: SolverFunc,
    input: &'a str,
}
