use anyhow::{Context, Result};
use std::{collections::HashMap, path::PathBuf, str::FromStr};

pub fn part1(input: &str) -> Result<String> {
    let dir_sizes = parse_dir_sizes(input)?;

    let mut total = 0;
    for size in dir_sizes.values() {
        if *size <= 100000 {
            total += size;
        }
    }

    Ok(total.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let dir_sizes = parse_dir_sizes(input)?;

    let used = *dir_sizes
        .get(&PathBuf::from("/"))
        .context("No root directory")?;
    let unused = 70000000 - used;
    let need = 30000000 - unused;

    let dir_size = dir_sizes
        .values()
        .filter(|size| **size >= need)
        .min()
        .context("No directory found")?;

    Ok(dir_size.to_string())
}

fn parse_dir_sizes(input: &str) -> Result<HashMap<PathBuf, usize>> {
    let lines = input
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<TermLine>>>()?;

    let mut dir_sizes: HashMap<PathBuf, usize> = HashMap::new();
    let mut cwd = PathBuf::new();
    for line in &lines {
        if let TermLine::CdCommand(dir) = line {
            if dir == ".." {
                cwd.pop();
            } else {
                cwd.push(dir);
            }
        } else if let TermLine::FileListing(_, size) = line {
            let mut cwd = cwd.clone();
            loop {
                *dir_sizes.entry(cwd.clone()).or_insert(0) += size;
                if !cwd.pop() {
                    break;
                }
            }
        }
    }

    Ok(dir_sizes)
}

#[derive(Debug, Clone)]
enum TermLine {
    CdCommand(String),
    LsCommand,
    DirListing(String),
    FileListing(String, usize),
}

impl FromStr for TermLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(dir) = s.strip_prefix("$ cd ") {
            Ok(Self::CdCommand(dir.to_owned()))
        } else if s == "$ ls" {
            Ok(Self::LsCommand)
        } else if let Some(dir) = s.strip_prefix("dir ") {
            Ok(Self::DirListing(dir.to_owned()))
        } else {
            let (size, file) = s.split_once(' ').context(format!("Bad command: {}", s))?;
            Ok(Self::FileListing(file.to_owned(), size.parse()?))
        }
    }
}
