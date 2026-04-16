use anyhow::{Context, Result, anyhow};
use std::fs::read_to_string;

#[derive(Clone)]
struct Range {
    begin: u64,
    end: u64,
}

fn parse_input(contents: &str) -> Result<(Vec<Range>, Vec<u64>)> {
    let mut parsed_ranges = Vec::<Range>::new();
    let mut parsed_items = Vec::<u64>::new();
    let mut reading_ranges = true;

    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        if line.is_empty() {
            reading_ranges = false;
            continue;
        }

        if reading_ranges {
            let (begin_part, end_part) = line.split_once('-').ok_or_else(|| {
                anyhow!("Invalid range format at line {}: '{}'", line_number, line)
            })?;

            let begin: u64 = begin_part.parse().with_context(|| {
                format!(
                    "Failed to parse range begin at line {}: '{}'",
                    line_number, line
                )
            })?;
            let end: u64 = end_part.parse().with_context(|| {
                format!(
                    "Failed to parse range end at line {}: '{}'",
                    line_number, line
                )
            })?;

            if begin > end {
                return Err(anyhow!(
                    "Invalid range at line {}: begin ({}) must be <= end ({}) in '{}'",
                    line_number,
                    begin,
                    end,
                    line
                ));
            }

            parsed_ranges.push(Range { begin, end });
        } else {
            parsed_items.push(line.parse().with_context(|| {
                format!("Failed to parse item at line {}: '{}'", line_number, line)
            })?);
        }
    }

    Ok((parsed_ranges, parsed_items))
}

fn consolidate_ranges(mut parsed_ranges: Vec<Range>) -> Result<Vec<Range>> {
    parsed_ranges.sort_unstable_by(|a, b| a.begin.cmp(&b.begin));

    let mut merged_ranges = Vec::<Range>::new();
    let mut parsed_iter = parsed_ranges.into_iter();
    let first_range = parsed_iter.next().ok_or(anyhow!("No ranges available"))?;
    merged_ranges.push(first_range);

    for range in parsed_iter {
        let last_merged = merged_ranges
            .last_mut()
            .ok_or(anyhow!("No ranges available"))?;

        if range.begin <= last_merged.end.saturating_add(1) {
            if range.end > last_merged.end {
                last_merged.end = range.end;
            }
        } else {
            merged_ranges.push(range);
        }
    }

    Ok(merged_ranges)
}

fn count_valid_items(merged_ranges: &[Range]) -> u64 {
    let mut valid_items_count: u64 = 0;

    for range in merged_ranges {
        valid_items_count += range.end - range.begin + 1;
    }

    valid_items_count
}

fn main() -> Result<()> {
    let contents = read_to_string("day05/input").context("Cannot read file")?;
    let contents = contents.trim();
    let (parsed_ranges, parsed_items) = parse_input(contents)?;
    let merged_ranges = consolidate_ranges(parsed_ranges)?;
    let valid_items = count_valid_items(&merged_ranges);

    println!("Number of valid items: {}", valid_items);
    Ok(())
}
