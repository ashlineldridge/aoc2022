use anyhow::anyhow;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::VecDeque, str::FromStr};

use anyhow::{Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let (mut dock, moves) = parse_note(input)?;
    for m in &moves {
        dock.exec9000(m)?;
    }

    Ok(dock.code())
}

pub fn part2(input: &str) -> Result<String> {
    let (mut dock, moves) = parse_note(input)?;
    for m in &moves {
        dock.exec9001(m)?;
    }

    Ok(dock.code())
}

fn parse_note(input: &str) -> Result<(Dock, Vec<Move>)> {
    let (top, bot) = input.split_once("\n\n").context("Invalid input")?;
    parse_dock(top).and_then(|dock| parse_moves(bot).map(|moves| (dock, moves)))
}

fn parse_dock(input: &str) -> Result<Dock> {
    input.parse()
}

fn parse_moves(input: &str) -> Result<Vec<Move>> {
    input.lines().map(|line| line.parse()).collect()
}

#[derive(Debug)]
struct Dock {
    cols: Vec<VecDeque<char>>,
}

impl Dock {
    fn new() -> Dock {
        Dock { cols: vec![] }
    }

    fn push(&mut self, ch: char, index: usize) {
        for _ in self.cols.len()..=index {
            self.cols.push(VecDeque::new());
        }

        let col = &mut self.cols[index];
        col.push_front(ch);
    }

    fn exec9000(&mut self, m: &Move) -> Result<()> {
        for _ in 0..m.amount {
            let ch = self
                .cols
                .get_mut(m.from - 1)
                .context("Invalid from column")?
                .pop_back()
                .context("Not enough crates")?;

            self.cols
                .get_mut(m.to - 1)
                .context("Invalid to column")?
                .push_back(ch);
        }
        Ok(())
    }

    fn exec9001(&mut self, m: &Move) -> Result<()> {
        let mut group = VecDeque::new();
        for _ in 0..m.amount {
            let ch = self
                .cols
                .get_mut(m.from - 1)
                .context("Invalid from column")?
                .pop_back()
                .context("Not enough crates")?;

            group.push_front(ch);
        }

        self.cols
            .get_mut(m.to - 1)
            .context("Invalid to column")?
            .append(&mut group);

        Ok(())
    }

    fn code(&self) -> String {
        let mut code = "".to_owned();
        for col in &self.cols {
            if let Some(ch) = col.back() {
                code.push(*ch);
            }
        }

        code
    }
}

impl FromStr for Dock {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut dock = Dock::new();
        for line in s.lines() {
            let buf = line.as_bytes();
            for (col_index, line_index) in (1..buf.len()).step_by(4).enumerate() {
                let ch = buf[line_index] as char;
                if ch.is_numeric() {
                    // We've reached the last line.
                    break;
                }

                if ch.is_alphabetic() {
                    // We've found a crate that needs to be pushed into a column.
                    dock.push(ch, col_index);
                }
            }
        }

        Ok(dock)
    }
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^move (?P<a>\d+) from (?P<f>\d+) to (?P<t>\d+)$").unwrap();
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| anyhow!("Invalid move: {}", s))?;

        let amount = caps["a"].parse()?;
        let from = caps["f"].parse()?;
        let to = caps["t"].parse()?;

        Ok(Self { amount, from, to })
    }
}
