use crate::MyResult;

pub mod day_01;
pub mod day_02;

pub fn run() -> MyResult<()> {
    day_01::solve()?;
    day_02::solve()?;

    Ok(())
}
