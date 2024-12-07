use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/inputs/input-day7.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<(u64, Vec<u64>)> = reader
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let mut parts = line.split(':').map(str::trim);
            let expected_result = parts.next().unwrap().parse::<u64>().unwrap();
            let numbers: Vec<u64> = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            (expected_result, numbers)
        })
        .collect();

    let valid_equations = calculate_valid_equations(&lines, ["+", "*"].to_vec());
    let valid_equations_2 = calculate_valid_equations(&lines, ["+", "*", "||"].to_vec());

    println!(
        "The number of valid equations for part 1 is {}",
        valid_equations
    );
    println!(
        "The number of valid equations for part 2 is {}",
        valid_equations_2
    );
}

fn calculate_valid_equations(lines: &Vec<(u64, Vec<u64>)>, operators: Vec<&str>) -> u64 {
    lines
        .iter()
        .filter(|(expected, numbers)| {
            is_equation_valid(*expected, &numbers[1..], &operators, numbers[0])
        })
        .map(|(result, _)| result)
        .sum()
}

fn is_equation_valid(
    expected_result: u64,
    numbers: &[u64],
    operators: &[&str],
    partial_result: u64,
) -> bool {
    if partial_result > expected_result {
        return false;
    }

    if numbers.is_empty() {
        return partial_result == expected_result;
    }

    operators.iter().any(|&op| {
        let next_partial = match op {
            "+" => add(partial_result, numbers[0]),
            "*" => multiply(partial_result, numbers[0]),
            "||" => concat(partial_result, numbers[0]),
            _ => panic!("Unknown operator"),
        };

        is_equation_valid(expected_result, &numbers[1..], operators, next_partial)
    })
}

fn add(a: u64, b: u64) -> u64 {
    a + b
}

fn multiply(a: u64, b: u64) -> u64 {
    a * b
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}
