use anyhow::Result;
use std::error::Error;
use std::fs;

const NUMBER_OF_DIGITS: usize = 12; // This is how many digits should be used to create the final number

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day03/input")?;
    let input = input.trim();
    let mut sum: u64 = 0;
    for line in input.lines() {
        let line: Result<Vec<u64>, anyhow::Error> = line
            .chars()
            .rev()
            .map(|char| match char.to_digit(10) {
                Some(digit) => Ok(digit as u64),
                None => Err(anyhow::anyhow!("Cannot convert char into digit.")),
            })
            .collect();
        let line = line?;
        let mut positions = vec![0; NUMBER_OF_DIGITS];
        let mut previous_highest_digit_position: usize = line.len();
        for digit in (0 as usize..NUMBER_OF_DIGITS).rev() {
            let mut highest_number: u64 = 0;
            let mut highest_digit_position = digit;
            for pos in digit..previous_highest_digit_position {
                if line[pos] >= highest_number {
                    highest_number = line[pos];
                    highest_digit_position = pos;
                }
            }
            positions[digit] = highest_digit_position;
            previous_highest_digit_position = highest_digit_position;
        }
        let mut number: u64 = 0;
        positions.reverse();
        for pos in positions.into_iter() {
            number = line[pos] + number * 10;
        }
        sum = sum + number;
    }
    println!("Sum: {}", sum);
    Ok(())
}
