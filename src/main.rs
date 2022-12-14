use aoc_runner::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Which day to run
    #[clap(short, long, value_parser)]
    day: String,

    /// Run test data
    #[clap(short, long, action)]
    test: bool,
}

aoc_days!(aoc1, aoc2, aoc3, aoc4, aoc5, aoc6, aoc7, aoc8, aoc9, aoc10, aoc11, aoc12, aoc13);

fn main() {
    let args = Args::parse();

    let day = args.day;

    let test = args.test;

    if test {
        match_day_test(&day);
    } else {
        match_day(&day);
    }
}
