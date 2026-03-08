use anyhow::{Context, Result, anyhow};
use std::fs::read_to_string;

const ADJACENT_ROLLS_LIMIT: u32 = 4;
const ROLL_CHAR: char = '@';

fn count_adjacent_rolls(grid: &[Vec<char>], row: usize, col: usize) -> u32 {
    let row_count = grid.len() as isize;
    let col_count = grid[0].len() as isize;
    let row = row as isize;
    let col = col as isize;

    let mut adjacent_rolls = 0;
    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let next_row = row + row_offset;
            let next_col = col + col_offset;
            if next_row < 0 || next_row >= row_count || next_col < 0 || next_col >= col_count {
                continue;
            }

            if grid[next_row as usize][next_col as usize] == ROLL_CHAR {
                adjacent_rolls += 1;
            }
        }
    }

    adjacent_rolls
}

fn main() -> Result<()> {
    let input = read_to_string("day04/input").context("Cannot read input file")?;
    let input = input.trim();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let row_count = grid.len();
    let col_count = grid.first().ok_or(anyhow!("No lines read"))?.len();
    if col_count == 0 {
        return Err(anyhow!("Input contains empty lines only"));
    }
    if grid.iter().any(|row| row.len() != col_count) {
        return Err(anyhow!("Input grid is not rectangular"));
    }

    let mut accessible_rolls: u32 = 0;
    for row in 0..row_count {
        for col in 0..col_count {
            if grid[row][col] != ROLL_CHAR {
                continue;
            }

            let adjacent_rolls = count_adjacent_rolls(&grid, row, col);
            if adjacent_rolls < ADJACENT_ROLLS_LIMIT {
                accessible_rolls += 1;
                println!("Accessible roll at {}, {}", row, col);
            }
        }
    }

    println!("Number of accessible rolls: {accessible_rolls}");
    Ok(())
}
