use anyhow::{Context, Result, anyhow};
use std::collections::VecDeque;
use std::fs::read_to_string;

#[derive(Clone)]
struct Range {
    begin: u64,
    end: u64,
}

fn main() -> Result<()> {
    let contents = read_to_string("day05/input").context("Cannot read file")?;
    let contents = contents.trim();
    let mut ranges = VecDeque::<Range>::new();
    for line in contents.lines() {
        if line.is_empty() {
            break;
        }

        let range = line.split('-');
        let range: Vec<&str> = range.collect();
        ranges.push_back(Range {
            begin: range[0].parse()?,
            end: range[1].parse()?,
        });
    }
    let mut ranges: Vec<&Range> = ranges.iter().collect();
    ranges.sort_by(|a: &&Range, b: &&Range| a.begin.cmp(&b.begin));
    let mut consolidated_ranges = VecDeque::new();
    consolidated_ranges.push_back(ranges[0].clone());
    for range in ranges {
        let mut consolidated_range = consolidated_ranges
            .pop_back()
            .ok_or(anyhow!("No ranges available"))?;
        if range.begin <= consolidated_range.end + 1 {
            // We're extending a range
            if range.end > consolidated_range.end {
                consolidated_range.end = range.end
            }
            consolidated_ranges.push_back(consolidated_range);
        } else {
            consolidated_ranges.push_back(consolidated_range);
            consolidated_ranges.push_back(range.clone());
        }
    }
    let mut total_fresh_item_ids: u64 = 0;
    for range in consolidated_ranges {
        total_fresh_item_ids = total_fresh_item_ids + range.end - range.begin + 1;
    }
    println!("Total number of fresh item IDs: {}", total_fresh_item_ids);
    Ok(())
}
