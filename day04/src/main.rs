use anyhow::{Context, Result, anyhow};
use std::fs::read_to_string;

const ADJACENT_ROLLS_LIMIT: u32 = 4;

fn main() -> Result<()> {
    let input = read_to_string("day04/input").context("Cannot read input file")?;
    let input = input.trim();
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| -> Vec<char> { line.chars().collect() })
        .collect();
    let number_of_rows = lines.len() as isize;
    let number_of_columns = lines.first().ok_or(anyhow!("No lines read"))?.len() as isize;
    let mut accessable_rolls: u32 = 0;
    for row in 0..number_of_rows {
        for column in 0..number_of_columns {
            if lines[row as usize][column as usize] != '@' {
                continue;
            }
            let mut adjacent_rolls: u32 = 0;
            for row_offset in -1..=1 {
                for column_offset in -1..=1 {
                    if (row + row_offset < 0)
                        || (row + row_offset >= number_of_rows)
                        || (column + column_offset < 0)
                        || (column + column_offset >= number_of_columns)
                        || (column_offset == 0 && row_offset == 0)
                    {
                        continue;
                    } else if lines[(row + row_offset) as usize][(column + column_offset) as usize]
                        == '@'
                    {
                        adjacent_rolls = adjacent_rolls + 1;
                    }
                }
            }
            if adjacent_rolls < ADJACENT_ROLLS_LIMIT {
                accessable_rolls = accessable_rolls + 1;
                println!("Accessable roll at {}, {}", row, column);
            }
        }
    }
    println!("Number of accessable rolls: {accessable_rolls}");
    Ok(())
}
