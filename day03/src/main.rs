use anyhow::Result;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day03/input")?;
    let input = input.trim();
    let mut sum: u64 = 0;
    for line in input.lines() {
        let mut highest_number: u64 = 0;
        let line: Vec<char> = line.chars().collect();
        for first_pos in 0..line.len() - 1 {
            for second_pos in first_pos + 1..line.len() {
                let first_number: u64 = (line[first_pos]
                    .to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Can't convert to digit"))?
                    * 10) as u64;
                let second_number = line[second_pos]
                    .to_digit(10)
                    .ok_or_else(|| anyhow::anyhow!("Can't convert to digit"))?
                    as u64;
                let number = first_number + second_number;
                if number > highest_number {
                    highest_number = number;
                }
            }
        }
        sum = sum + highest_number;
    }
    println!("Sum: {}", sum);
    Ok(())
}
