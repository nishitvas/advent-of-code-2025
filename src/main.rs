use clap::Parser;
mod common;
use common::solution::Part;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;

#[derive(Parser)]
struct Cli {
    /// Run solution for this day
    #[arg(long)]
    day: u8,
    /// Run solution for this part
    #[arg(long)]
    part: Option<u8>,
    /// Use demo input
    #[arg(long)]
    demo: bool,
}

fn main() {
    let args = Cli::parse();

    println!("======================");
    println!("Advent of Code - 2025!");
    println!("======================");

    let part: Part = match args.part {
        Some(1) => Part::ONE,
        Some(2) => Part::TWO,
        Some(_) => Part::ALL,
        None => Part::ALL,
    };

    match args.day {
        1 => day_01::solve(part, args.demo),
        2 => day_02::solve(part, args.demo),
        3 => day_03::solve(part, args.demo),
        4 => day_04::solve(part, args.demo),
        5 => day_05::solve(part, args.demo),
        6 => day_06::solve(part, args.demo),
        7 => day_07::solve(part, args.demo),
        8 => day_08::solve(part, args.demo),
        _ => println!("Unsolved for day {:?}", args.day),
    }
}
