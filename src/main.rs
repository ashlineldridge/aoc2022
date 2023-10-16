use anyhow::Result;
use clap::Parser;
use std::{fs, path::PathBuf};

use aoc2022::*;

type PuzzlePart = &'static dyn Fn(&str) -> Result<String>;
const PUZZLE_PARTS: [[PuzzlePart; 2]; 25] = [
    [&day01::part1, &day01::part2],
    [&day02::part1, &day02::part2],
    [&day03::part1, &day03::part2],
    [&day04::part1, &day04::part2],
    [&day05::part1, &day05::part2],
    [&day06::part1, &day06::part2],
    [&day07::part1, &day07::part2],
    [&day08::part1, &day08::part2],
    [&day09::part1, &day09::part2],
    [&day10::part1, &day10::part2],
    [&day11::part1, &day11::part2],
    [&day12::part1, &day12::part2],
    [&day13::part1, &day13::part2],
    [&day14::part1, &day14::part2],
    [&day15::part1, &day15::part2],
    [&day16::part1, &day16::part2],
    [&day17::part1, &day17::part2],
    [&day18::part1, &day18::part2],
    [&day19::part1, &day19::part2],
    [&day20::part1, &day20::part2],
    [&day21::part1, &day21::part2],
    [&day22::part1, &day22::part2],
    [&day23::part1, &day23::part2],
    [&day24::part1, &day24::part2],
    [&day25::part1, &day25::part2],
];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Advent day (1-25 inclusive)
    #[arg(short, long, value_name = "NUM", value_parser = clap::value_parser!(u8).range(1..=25))]
    day: u8,

    /// Puzzle part (1 or 2)
    #[arg(short, long, value_name = "NUM", value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,

    /// Input file
    #[arg(short, long, value_name = "FILE", value_parser = clap::value_parser!(PathBuf))]
    file: Option<PathBuf>,
}

#[rustfmt::skip]
fn main() -> Result<()> {
    let cli = Cli::parse();
    let file = cli.file.unwrap_or(
        PathBuf::from(format!("data/day{:02}-input.txt", cli.day))
    );
    let input = fs::read_to_string(file)?;
    let day_index = cli.day as usize - 1;
    let part_index = cli.part as usize - 1;

    let answer = PUZZLE_PARTS[day_index][part_index](&input)?;
    println!("Day {}, part {}: {}", cli.day, cli.part, answer);

    Ok(())
}
