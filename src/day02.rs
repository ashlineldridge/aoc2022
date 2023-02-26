use std::str::FromStr;

use anyhow::{bail, Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let game: Game = read_game1(input)?;
    let score = game.score();
    Ok(score.1.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let game: Game = read_game2(input)?;
    let score = game.score();
    Ok(score.1.to_string())
}

struct Game {
    rounds: Vec<Round>,
}

type Round = (Hand, Hand);

impl Game {
    fn score(&self) -> (u32, u32) {
        let mut score = (0, 0);
        for (h1, h2) in &self.rounds {
            let p1 = *h1 as u32;
            let p2 = *h2 as u32;
            let (p1, p2) = if h1 == h2 {
                (p1 + 3, p2 + 3)
            } else if h1.beats(h2) {
                (p1 + 6, p2)
            } else {
                (p1, p2 + 6)
            };

            score.0 += p1;
            score.1 += p2;
        }

        score
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Hand {
    Rock = 1,
    Paper,
    Scissors,
}

impl Hand {
    fn beats(&self, other: &Hand) -> bool {
        matches!(
            (self, other),
            (Hand::Rock, Hand::Scissors)
                | (Hand::Paper, Hand::Rock)
                | (Hand::Scissors, Hand::Paper)
        )
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => bail!("Invalid hand: {}", s),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => bail!("Invalid outcome: {}", s),
        }
    }
}

fn read_game1(input: &str) -> Result<Game> {
    let mut rounds = vec![];
    for line in input.lines() {
        let (h1, h2) = line
            .split_once(' ')
            .context(format!("Invalid line: {}", line))?;
        rounds.push((h1.parse()?, h2.parse()?))
    }

    Ok(Game { rounds })
}

fn read_game2(input: &str) -> Result<Game> {
    let mut round_outcomes: Vec<(Hand, Outcome)> = vec![];
    for line in input.lines() {
        let (h1, h2) = line
            .split_once(' ')
            .context(format!("Invalid line: {}", line))?;
        round_outcomes.push((h1.parse()?, h2.parse()?))
    }

    let mut rounds = vec![];
    for (h1, outcome) in round_outcomes {
        let h2 = match (h1, outcome) {
            (_, Outcome::Draw) => h1,
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Rock, Outcome::Lose) => Hand::Scissors,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Paper, Outcome::Lose) => Hand::Rock,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
            (Hand::Scissors, Outcome::Lose) => Hand::Paper,
        };

        rounds.push((h1, h2));
    }

    Ok(Game { rounds })
}
