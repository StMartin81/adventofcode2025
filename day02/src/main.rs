use anyhow::Result;
use std::error::Error;
use std::fs;

fn compare<'a>(
    first_pattern: &'a str,
    rest_pattern: &'a str,
    pattern_length: usize,
) -> (bool, &'a str) {
    let mut pattern_repeated: bool = true;
    let mut second_pattern: &'a str = rest_pattern;
    if rest_pattern.len() != pattern_length {
        let (first_pattern, rest_pattern) = rest_pattern.split_at(pattern_length);
        (pattern_repeated, second_pattern) = compare(first_pattern, rest_pattern, pattern_length);
    }
    if pattern_repeated == true && first_pattern == second_pattern {
        return (true, first_pattern);
    }
    (false, first_pattern)
}

fn is_invalid(number: u64) -> bool {
    let number = number.to_string();
    let length = number.len();
    let max_pattern_length = length / 2;
    for pattern_length in 1..=max_pattern_length {
        if length % pattern_length == 0 && length / pattern_length != 0 {
            let (first_part, second_part) = number.split_at(pattern_length);
            let (pattern_repeated, _) = compare(first_part, second_part, pattern_length);
            if pattern_repeated == true {
                return true;
            };
        }
    }
    false
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day02/input")?;
    let mut sum: u64 = 0;
    for line in input.lines() {
        let line = line.trim();
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
