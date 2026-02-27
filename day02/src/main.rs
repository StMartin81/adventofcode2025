use anyhow::Result;
use std::error::Error;
use std::fs;

fn is_invalid(number: u64) -> bool {
    let number = number.to_string();
    let length = number.len();
    // string has to have an even length so that the pattern can repeat
    if length % 2 == 0 {
        let (first_part, second_part) = number.split_at(length / 2);
        if first_part == second_part {
            println!("Invalid number: {}", number);
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day02/input")?;
    let mut sum: u64 = 0;
    for line in input.lines() {
        let line = line.trim();
        if line == "" {
            println!("Line is empty.");
            break;
        }
        for range in line.split(',') {
            let range: Vec<&str> = range.split('-').collect();
            let range_begin = range[0].parse::<u64>()?;
            let range_end = range[1].parse::<u64>()?;
            for i in range_begin..=range_end {
                if is_invalid(i) {
                    sum = sum + i;
                }
            }
        }
    }
    println!("Sum is: {}", sum);
    Ok(())
}
