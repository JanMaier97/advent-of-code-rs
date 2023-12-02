use std::error::Error;

mod year_2022;
mod year_2023;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() -> MyResult<()> {
    year_2022::run()?;
    year_2023::run()?;

    Ok(())
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
