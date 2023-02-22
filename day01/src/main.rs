use std::{
    io::{self, Read},
    str::FromStr,
};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let elves = read_elves(input)?;
    let max_elf = elves
        .iter()
        .max_by(|e1, e2| e1.total().cmp(&e2.total()))
        .context("No elves")?;

    println!("Part 1 answer: {}", max_elf.total());

    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut elves = read_elves(input)?
        .iter()
        .map(Elf::total)
        .collect::<Vec<_>>();
    elves.sort_by(|t1, t2| t2.cmp(t1));
    let total: u32 = elves.iter().take(3).sum();

    println!("Part 2 answer: {}", total);

    Ok(())
}

fn read_elves(input: &str) -> Result<Vec<Elf>> {
    input
        .split("\n\n")
        .map(|chunk| chunk.parse::<Elf>())
        .collect::<Result<Vec<_>>>()
}

#[derive(Debug)]
struct Elf {
    cals: Vec<u32>,
}

impl Elf {
    fn total(&self) -> u32 {
        self.cals.iter().sum()
    }
}

impl FromStr for Elf {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cals = s
            .lines()
            .map(|line| line.parse().context("Invalid line"))
            .collect::<Result<_>>()?;

        Ok(Elf { cals })
    }
}
