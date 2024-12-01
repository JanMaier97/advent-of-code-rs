use crate::MyResult;

mod day_01;

pub fn run() -> MyResult<()> {
    day_01::solve()?;

    Ok(())
}
