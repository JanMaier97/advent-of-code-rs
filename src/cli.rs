use clap::Parser;

use crate::{ExecutionArgs, MyResult};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[arg(value_parser = clap::value_parser!(u16).range(2015..=2024))]
    pub year: u16,

    #[arg(value_parser = clap::value_parser!(u8).range(1..=25))]
    pub day: u8,

    #[arg(value_parser = clap::value_parser!(u8).range(1..=2))]
    pub part: u8,
}

pub fn parse_args() -> MyResult<ExecutionArgs> {
    let args = CliArgs::try_parse()?;

    let res = ExecutionArgs {
        year: args.year,
        day: args.day,
        part: args.part,
    };

    Ok(res)
}
