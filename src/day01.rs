use std::str::FromStr;

use anyhow::{Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let elves = read_elves(input)?;
    let max_elf = elves
        .iter()
        .max_by(|e1, e2| e1.total().cmp(&e2.total()))
        .context("No elves")?;

    Ok(max_elf.total().to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let mut elves = read_elves(input)?
        .iter()
        .map(Elf::total)
        .collect::<Vec<_>>();

    elves.sort_by(|t1, t2| t2.cmp(t1));
    let total: u32 = elves.iter().take(3).sum();

    Ok(total.to_string())
}

fn read_elves(input: &str) -> Result<Vec<Elf>> {
    input
        .split("\n\n")
        .map(|chunk| chunk.parse::<Elf>())
        .collect::<Result<Vec<_>>>()
}

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
