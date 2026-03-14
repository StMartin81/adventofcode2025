use anyhow::{Context, Result, anyhow};
use std::fs::read_to_string;

const ADJACENT_ROLLS_LIMIT: u32 = 4;
const ROLL_CHAR: char = '@';

fn main() -> Result<()> {
    let input = read_to_string("day04/input").context("Cannot read input file")?;
    let input = input.trim();
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let row_count = grid.len() as isize;
    let col_count = grid.first().ok_or(anyhow!("No lines read"))?.len() as isize;
    if col_count == 0 {
        return Err(anyhow!("Input contains empty lines only"));
    }
    if grid.iter().any(|row| row.len() != col_count as usize) {
        return Err(anyhow!("Input grid is not rectangular"));
    }

    let mut total_accessible_rolls: u32 = 0;
    loop {
        let mut accessible_rolls: u32 = 0;
        for row in 0..row_count as isize {
            for col in 0..col_count as isize {
                if grid[row as usize][col as usize] != ROLL_CHAR {
                    continue;
                }
                let mut adjacent_rolls = 0;
                for row_offset in -1..=1 as isize {
                    for col_offset in -1..=1 as isize {
                        if row_offset == 0 && col_offset == 0 {
                            continue;
                        }

                        let next_row = row + row_offset;
                        let next_col = col + col_offset;
                        if next_row < 0
                            || next_row >= row_count
                            || next_col < 0
                            || next_col >= col_count
                        {
                            continue;
                        }

                        if grid[next_row as usize][next_col as usize] == ROLL_CHAR {
                            adjacent_rolls += 1;
                        }
                    }
                }
                if adjacent_rolls < ADJACENT_ROLLS_LIMIT {
                    accessible_rolls += 1;
                    grid[row as usize][col as usize] = '.';
                }
            }
        }
        total_accessible_rolls = total_accessible_rolls + accessible_rolls;
        if accessible_rolls == 0 {
            break;
        }
    }

    println!("Number of accessible rolls: {total_accessible_rolls}");
    Ok(())
}
