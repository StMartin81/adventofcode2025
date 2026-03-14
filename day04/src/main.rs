use anyhow::{Context, Result, anyhow};
use std::collections::VecDeque;
use std::fs::read_to_string;

const ADJACENT_ROLLS_LIMIT: u32 = 4;
const ROLL_CHAR: char = '@';

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

    let mut active = vec![vec![false; col_count]; row_count];
    let mut degree = vec![vec![0_u8; col_count]; row_count];

    for row in 0..row_count {
        for col in 0..col_count {
            if grid[row][col] == ROLL_CHAR {
                active[row][col] = true;
            }
        }
    }

    for row in 0..row_count {
        for col in 0..col_count {
            if !active[row][col] {
                continue;
            }

            let row_start = row.saturating_sub(1);
            let row_end = (row + 1).min(row_count - 1);
            let col_start = col.saturating_sub(1);
            let col_end = (col + 1).min(col_count - 1);

            let mut adjacent_rolls = 0_u8;
            for next_row in row_start..=row_end {
                for next_col in col_start..=col_end {
                    if next_row == row && next_col == col {
                        continue;
                    }
                    if active[next_row][next_col] {
                        adjacent_rolls += 1;
                    }
                }
            }

            degree[row][col] = adjacent_rolls;
        }
    }

    let mut queue = VecDeque::new();
    let limit = ADJACENT_ROLLS_LIMIT as u8;
    for row in 0..row_count {
        for col in 0..col_count {
            if active[row][col] && degree[row][col] < limit {
                queue.push_back((row, col));
            }
        }
    }

    let mut total_accessible_rolls: u32 = 0;
    while let Some((row, col)) = queue.pop_front() {
        if !active[row][col] {
            continue;
        }

        active[row][col] = false;
        total_accessible_rolls += 1;

        let row_start = row.saturating_sub(1);
        let row_end = (row + 1).min(row_count - 1);
        let col_start = col.saturating_sub(1);
        let col_end = (col + 1).min(col_count - 1);

        for next_row in row_start..=row_end {
            for next_col in col_start..=col_end {
                if next_row == row && next_col == col {
                    continue;
                }
                if !active[next_row][next_col] {
                    continue;
                }

                degree[next_row][next_col] -= 1;
                if degree[next_row][next_col] < limit {
                    queue.push_back((next_row, next_col));
                }
            }
        }
    }

    println!("Number of accessible rolls: {total_accessible_rolls}");
    Ok(())
}
