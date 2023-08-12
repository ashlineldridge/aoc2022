use anyhow::Result;
use aoc2022::*;
use std::{fs, path::PathBuf};

use clap::Parser;

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
    file: PathBuf,
}

#[rustfmt::skip]
fn main() -> Result<()> {
    let cli = Cli::parse();
    let input = fs::read_to_string(cli.file)?;
    let func = match cli.day {
        01 => if cli.part == 1 { day01::part1 } else { day01::part2 },
        02 => if cli.part == 1 { day02::part1 } else { day02::part2 },
        03 => if cli.part == 1 { day03::part1 } else { day03::part2 },
        04 => if cli.part == 1 { day04::part1 } else { day04::part2 },
        05 => if cli.part == 1 { day05::part1 } else { day05::part2 },
        06 => if cli.part == 1 { day06::part1 } else { day06::part2 },
        07 => if cli.part == 1 { day07::part1 } else { day07::part2 },
        08 => if cli.part == 1 { day08::part1 } else { day08::part2 },
        09 => if cli.part == 1 { day09::part1 } else { day09::part2 },
        10 => if cli.part == 1 { day10::part1 } else { day10::part2 },
        11 => if cli.part == 1 { day11::part1 } else { day11::part2 },
        12 => if cli.part == 1 { day12::part1 } else { day12::part2 },
        13 => if cli.part == 1 { day13::part1 } else { day13::part2 },
        14 => if cli.part == 1 { day14::part1 } else { day14::part2 },
        15 => if cli.part == 1 { day15::part1 } else { day15::part2 },
        16 => if cli.part == 1 { day16::part1 } else { day16::part2 },
        17 => if cli.part == 1 { day17::part1 } else { day17::part2 },
        18 => if cli.part == 1 { day18::part1 } else { day18::part2 },
        19 => if cli.part == 1 { day19::part1 } else { day19::part2 },
        20 => if cli.part == 1 { day20::part1 } else { day20::part2 },
        21 => if cli.part == 1 { day21::part1 } else { day21::part2 },
        22 => if cli.part == 1 { day22::part1 } else { day22::part2 },
        23 => if cli.part == 1 { day23::part1 } else { day23::part2 },
        24 => if cli.part == 1 { day24::part1 } else { day24::part2 },
        25 => if cli.part == 1 { day25::part1 } else { day25::part2 },
        _ => panic!("Invalid state: Check CLI validation"),
    };

    let answer = func(&input)?;
    println!("Day {}, part {}: {}", cli.day, cli.part, answer);

    Ok(())
}
