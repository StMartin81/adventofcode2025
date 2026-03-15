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
    let mut items = VecDeque::<u64>::new();
    let mut reading_ranges = true;
    for line in contents.lines() {
        if line.is_empty() {
            reading_ranges = false;
            continue;
        }

        if reading_ranges {
            let range = line.split('-');
            let range: Vec<&str> = range.collect();
            ranges.push_back(Range {
                begin: range[0].parse()?,
                end: range[1].parse()?,
            });
        } else {
            items.push_back(line.parse()?);
        }
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
    let mut fresh_items: u64 = 0;
    let mut items_sorted: Vec<u64> = items.into_iter().collect();
    items_sorted.sort();
    let mut items_queue = VecDeque::new();
    for item in items_sorted {
        items_queue.push_back(item);
    }
    let consolidated_ranges = consolidated_ranges.into_iter();
    for range in consolidated_ranges {
        loop {
            if let Some(item) = items_queue.pop_front() {
                if item >= range.begin && item <= range.end {
                    fresh_items = fresh_items + 1;
                } else if item > range.end {
                    items_queue.push_front(item);
                    break;
                }
            } else {
                // no more items
                break;
            }
        }
    }
    println!("Number of fresh items: {}", fresh_items);
    Ok(())
}
