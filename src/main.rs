use clap::Parser;

mod day_01;
mod day_02;
mod day_03;

#[derive(Parser)]
struct Cli {
    /// Run solution for this day
    #[arg(long)]
    day: u8,
    /// Run solution for this part
    #[arg(long)]
    part: u8,
    /// Use demo input
    #[arg(long)]
    demo: bool,
}

fn main() {
    let args = Cli::parse();

    println!("======================");
    println!("Advent of Code - 2025!");
    println!("======================");

    match args.day {
        1 => day_01::solve(args.part, args.demo),
        2 => day_02::solve(args.part, args.demo),
        3 => day_03::solve(args.part, args.demo),
        _ => println!("Unsolved for day {:?}", args.day),
    }
}
