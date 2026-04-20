use anyhow::{Context, Result, anyhow};
use std::collections::VecDeque;
use std::fs::{read, read_to_string};

enum Operator {
    Sum,
    Product,
}

fn calc(operator: Operator, mut numbers: Vec<i64>) -> i64 {
    let number = numbers.pop().expect("Array is empty");
    if numbers.len() == 0 {
        return number;
    } else {
        match operator {
            Operator::Sum => number + calc(operator, numbers),
            Operator::Product => number * calc(operator, numbers),
        }
    }
}

fn main() -> Result<()> {
    let contents = read_to_string("./day06/input").context("Cannot read file")?;
    let contents = contents.trim();
    let line_count = contents.lines().count();
    println!("line count: {line_count}");
    let mut numbers = Vec::new();
    let mut operators = Vec::new();
    for (line_number, line) in contents.lines().enumerate() {
        if line_number + 1 < line_count {
            let mut number_row: Vec<i64> = line
                .split_whitespace()
                .map(|number| number.parse::<i64>())
                .collect::<Result<Vec<_>, _>>()
                .context("Couldn't parse number.")?;
            numbers.push(number_row);
        } else {
            operators = line.split_whitespace().collect::<Vec<&str>>();
        }
    }
    let mut final_number: i64 = 0;
    for _ in 0..operators.len() {
        let mut intermediate_numbers: Vec<i64> = Vec::new();
        for idx in 0..numbers.len() {
            intermediate_numbers.push(numbers[idx].pop().ok_or_else(|| anyhow!("Array empty"))?);
        }
        let operator = match operators
            .pop()
            .ok_or_else(|| anyhow!("No more operators"))?
        {
            "+" => Operator::Sum,
            "*" => Operator::Product,
            _ => return Err(anyhow!("Unsupported operator")),
        };
        final_number += calc(operator, intermediate_numbers);
    }
    println!("Final number: {}", final_number);
    Ok(())
}
