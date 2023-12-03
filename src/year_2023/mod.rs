use crate::MyResult;

mod day_01;
mod day_02;
mod day_03;

pub fn run() -> MyResult<()> {
    day_01::solve()?;
    day_02::solve()?;
    day_03::solve()?;

    Ok(())
}
