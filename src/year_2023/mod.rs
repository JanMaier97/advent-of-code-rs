use crate::MyResult;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
pub mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;

pub fn run() -> MyResult<()> {
    day_01::solve()?;
    day_02::solve()?;
    day_03::solve()?;
    day_04::solve()?;
    // day_05::solve()?;
    day_06::solve()?;
    day_07::solve()?;
    // day_08::solve()?;
    day_09::solve()?;

    Ok(())
}
