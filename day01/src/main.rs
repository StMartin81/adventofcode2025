use anyhow::{Result, anyhow};
use std::fs::read_to_string;

pub enum Direction {
    Left,
    Right,
}

fn main() -> Result<()> {
    let mut number_of_zeros: u32 = 0;
    let contents = read_to_string("day01/input")?;
    let mut position: u32 = 50;
    let max_pos: u32 = 100; // Actually this is max_pos + 1
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
        let clicks = clicks.parse::<u32>().unwrap() % max_pos;
        // println!("Number of clicks: {}", clicks);
        match direction {
            Direction::Left => {
                if position >= clicks {
                    position = position - clicks
                } else {
                    position = max_pos - (clicks - position)
                }
            }
            Direction::Right => position = (position + clicks) % max_pos,
        };
        // println!("New position: {}", position);
        if position == 0 {
            number_of_zeros = number_of_zeros + 1;
        }
    }
    println!("Number of zeros: {}", number_of_zeros);
    Ok(())
}
