use anyhow::{Result, anyhow};
use std::fs::read_to_string;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

fn main() -> Result<()> {
    let mut number_of_zeros: i32 = 0;
    let contents = read_to_string("day01/input")?;
    let mut position: i32 = 50;
    let mut old_position: i32 = 50;
    let max_pos: i32 = 100; // Actually this is max_pos + 1
    for line in contents.lines() {
        let (direction, clicks) = line.split_at(1);
        let direction = match direction {
            "L" => {
                // println!("Turning left");
                Direction::Left
            }
            "R" => {
                // println!("Turning right");
                Direction::Right
            }
            _ => Err(anyhow!("Unexpected character.")).unwrap(),
        };
        let mut clicks = clicks.parse::<i32>().unwrap();
        println!(
            "Number of clicks: {}, direction: {:?}, current position: {}",
            clicks, direction, position
        );
        let overflowing_zeros = clicks / max_pos;
        if overflowing_zeros != 0 {
            println!("Overflow zeros: {}", overflowing_zeros);
        }
        number_of_zeros = number_of_zeros + overflowing_zeros;
        clicks = clicks % max_pos;
        match direction {
            Direction::Left => {
                position = position - clicks;
            }
            Direction::Right => {
                position = position + clicks;
            }
        };
        if position >= max_pos {
            position = position - max_pos;
            number_of_zeros = number_of_zeros + 1;
        } else if position < 0 {
            position = position + max_pos;
            if old_position != 0 {
                number_of_zeros = number_of_zeros + 1;
            }
        } else if position == 0 {
            number_of_zeros = number_of_zeros + 1;
        }
        println!(
            "Number of zeros: {},New position: {}",
            number_of_zeros, position
        );
        old_position = position;
    }
    println!("Number of zeros: {}", number_of_zeros);
    Ok(())
}
