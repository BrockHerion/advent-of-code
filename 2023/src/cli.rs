use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets the day of the Advent of Code challenge to run
    #[clap(short, long)]
    pub day: Option<u32>,

    /// Enables verbose mode
    #[clap(short, long)]
    pub sample: bool,
}
