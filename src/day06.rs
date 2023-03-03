use std::collections::HashSet;

use anyhow::{Result, bail};

pub fn part1(input: &str) -> Result<String> {
    let start = detect_start_marker(input, 4)?;
    Ok(start.to_string())
}

pub fn part2(input: &str) -> Result<String> {
    let start = detect_start_marker(input, 14)?;
    Ok(start.to_string())
}

fn detect_start_marker(input: &str, width: usize) -> Result<usize> {
    let buf = input.as_bytes();
    for (i, win) in buf.windows(width).enumerate() {
        let set = win.iter().collect::<HashSet<_>>();
        if set.len() == width {
            return Ok(i + width)
        }
    }

    bail!("No unique {} character sequence found", width);
}
