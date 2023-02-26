use std::{collections::HashSet, str::FromStr};

use anyhow::{bail, Result};

pub fn part1(input: &str) -> Result<String> {
    let rucksacks = read_rucksacks(input)?;

    let mut sum = 0;
    for r in rucksacks {
        let s1: HashSet<Item> = r.comp1.into_iter().collect();
        let s2: HashSet<Item> = r.comp2.into_iter().collect();
        let s3: HashSet<Item> = s1.intersection(&s2).cloned().collect();
        if s3.len() != 1 {
            bail!("Rucksack compartments did not both contain same item");
        }

        let item = s3.into_iter().next().unwrap();
        sum += item.priority();
    }

    Ok(sum.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let rucksacks = read_rucksacks(input)?;
    let groups = rucksacks.chunks(3);

    let mut sum = 0;
    for group in groups {
        let mut common_items = HashSet::new();
        for r in group {
            let mut items: HashSet<Item> = r.comp1.iter().cloned().collect();
            items.extend(&r.comp2);

            if common_items.is_empty() {
                common_items = items;
            } else {
                common_items = common_items.intersection(&items).cloned().collect::<HashSet<_>>();
            }
        }

        if common_items.len() != 1 {
            bail!("Expected a single common item but got {}", common_items.len())
        }

        let badge = common_items.iter().next().unwrap();
        sum += badge.priority()
    }

    Ok(sum.to_string())
}

fn read_rucksacks(input: &str) -> Result<Vec<Rucksack>> {
    input.lines().map(Rucksack::from_str).collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Item(char);

impl Item {
    fn priority(&self) -> u32 {
        if self.0.is_uppercase() {
            return self.0 as u32 - 38;
        }

        if self.0.is_lowercase() {
            return self.0 as u32 - 96;
        }

        0
    }
}

struct Rucksack {
    comp1: Vec<Item>,
    comp2: Vec<Item>,
}

impl Rucksack {
    fn new() -> Rucksack {
        Rucksack {
            comp1: vec![],
            comp2: vec![],
        }
    }
}

impl FromStr for Rucksack {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() % 2 != 0 {
            bail!("Uneven number of items in rucksack: {}", s);
        }

        let mut rucksack = Rucksack::new();
        for (i, ch) in s.chars().enumerate() {
            if i < s.len() / 2 {
                rucksack.comp1.push(Item(ch));
            } else {
                rucksack.comp2.push(Item(ch));
            }
        }

        Ok(rucksack)
    }
}
