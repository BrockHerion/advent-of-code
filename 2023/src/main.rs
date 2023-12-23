mod cli;
mod day1;
mod solver;

use clap::Parser;
use cli::Cli;
use solver::Solver;

fn main() {
    let cli = Cli::parse();

    match cli.day {
        Some(day) => run_day(day, cli.sample),
        None => println!("Please specify the day using --day"),
    }
}

fn run_day(day: u32, sample: bool) {
    match day {
        1 => day1::Day1.solve(sample),
        _ => println!("Day {} not recognized", day),
    }
}
