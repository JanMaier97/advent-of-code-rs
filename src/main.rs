use std::error::Error;

mod year_2022;

type MyResult<T> = Result<T, Box<dyn Error>>;

fn main() -> MyResult<()> {
    year_2022::day_01::solve()?;
    year_2022::day_02::solve()?;
    year_2022::day_03::solve()?;
    year_2022::day_04::solve()?;
    year_2022::day_05::solve()?;
    year_2022::day_06::solve()?;
    year_2022::day_07::solve()?;
    year_2022::day_08::solve()?;
    year_2022::day_09::solve()?;
    year_2022::day_10::solve()?;
    year_2022::day_11::solve()?;
    year_2022::day_12::solve()?;
    year_2022::day_13::solve()?;

    Ok(())
}

fn print_challange_header(day: usize) {
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
