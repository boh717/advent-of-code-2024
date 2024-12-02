use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/inputs/input-day2.txt").unwrap();
    let reader = BufReader::new(file);
    let safe_reports = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|report| {
            report
                .split_whitespace()
                .map(|e| e.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|report| {
            is_report_safe(report)
                // Added for second half
                || (0..report.len()).any(|i| {
                    let report_without_one_level: Vec<_> = report
                        .iter()
                        .enumerate()
                        .filter(|&(index, _)| index != i)
                        .map(|(_, &value)| value)
                        .collect();
                    is_report_safe(&report_without_one_level)
                })
        })
        .count() as u32;
    println!("The number of safe reports is {}", safe_reports);
}

fn is_report_safe(levels: &Vec<u32>) -> bool {
    are_adjacent_levels_safe(&levels)
        && (levels.is_sorted_by(|a, b| a < b) || levels.is_sorted_by(|a, b| a > b))
}

fn are_adjacent_levels_safe(levels: &Vec<u32>) -> bool {
    levels
        .windows(2)
        .map(|w| is_permitted_adjacency(w[0], w[1]))
        .fold(true, |acc, e| acc && e)
}

fn is_permitted_adjacency(left: u32, right: u32) -> bool {
    let adj = left.abs_diff(right);
    1 <= adj && adj <= 3
}
