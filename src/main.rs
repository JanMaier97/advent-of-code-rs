use advent_of_code_rs::run;

fn main() -> () {
    if let Err(error) = run() {
        eprintln!("{}", error);
        std::process::exit(1);
    };
}
