use anyhow::{Context, Result};
use std::fs;

const NUMBER_OF_DIGITS: usize = 12;

fn solve_simplified_stack(line_str: &str) -> u64 {
    let line_chars: Vec<u64> = line_str
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u64))
        .collect();
    let k = NUMBER_OF_DIGITS;
    let n = line_chars.len();
    if n < k {
        return 0;
    }

    let mut stack = Vec::with_capacity(k);
    let mut to_drop = n - k;

    for &digit in &line_chars {
        while to_drop > 0 {
            if let Some(&last) = stack.last() {
                if last < digit {
                    stack.pop();
                    to_drop -= 1;
                    continue;
                }
            }
            break;
        }
        stack.push(digit);
    }
    stack.truncate(k);

    let mut number: u64 = 0;
    for &digit in stack.iter() {
        number = number * 10 + digit;
    }
    number
}

fn main() -> Result<()> {
    let input = fs::read_to_string("day03/input").context("Failed to read input")?;
    let input = input.trim();
    let mut sum: u64 = 0;

    for line in input.lines() {
        sum += solve_simplified_stack(line);
    }

    println!("Sum: {}", sum);
    Ok(())
}
