use anyhow::Context;
use anyhow::Result;
use std::str::FromStr;

pub fn part1(input: &str) -> Result<String> {
    let assignments = parse_assignments(input)?;
    let mut sum = 0;
    for (a1, a2) in &assignments {
        if a1.contains(a2) || a2.contains(a1) {
            sum += 1;
        }
    }

    Ok(sum.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let assignments = parse_assignments(input)?;
    let mut sum = 0;
    for (a1, a2) in &assignments {
        if a1.overlaps(a2) {
            sum += 1;
        }
    }

    Ok(sum.to_string())
}

fn parse_assignments(input: &str) -> Result<Vec<(Assignment, Assignment)>> {
    let mut pairs = vec![];
    for line in input.lines() {
        let (a1, a2) = line.split_once(',').context("Bad line")?;
        let a1 = a1.parse()?;
        let a2 = a2.parse()?;

        pairs.push((a1, a2));
    }

    Ok(pairs)
}

struct Assignment {
    first: u32,
    last: u32,
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.first <= other.first && self.last >= other.last
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.first <= other.first && other.first <= self.last
            || other.first <= self.first && self.first <= other.last
    }
}

impl FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (first, last) = s.split_once('-').context("Bad line")?;
        let first = first.parse()?;
        let last = last.parse()?;

        Ok(Self { first, last })
    }
}
