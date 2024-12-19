use std::collections::{HashMap, HashSet};

fn main() {
    let content: Vec<String> = std::fs::read_to_string("src/puzzles/puzzle-day19.txt")
        .unwrap()
        .split("\n\n")
        .map(|l| l.to_string())
        .collect();

    let patterns: HashSet<String> = content[0].split(", ").map(|s| s.to_string()).collect();
    let designs: Vec<String> = content[1].split("\n").map(|s| (s.to_string())).collect();

    let possible_designs = part1(&designs, &patterns);
    println!("The number of possible designs is {:?}", possible_designs);

    let all_designs = part2(&designs, &patterns);
    println!("All possible designs are {:?}", all_designs);
}

fn part1(designs: &Vec<String>, patterns: &HashSet<String>) -> u64 {
    designs
        .iter()
        .filter(|design| is_any_design_possible(design, patterns))
        .count() as u64
}

fn part2(designs: &Vec<String>, patterns: &HashSet<String>) -> u64 {
    let mut cache: HashMap<String, u64> = HashMap::new();
    designs
        .iter()
        .map(|design| count_every_possible_design(design, patterns, &mut cache))
        .sum()
}

fn count_every_possible_design(
    design: &String,
    patterns: &HashSet<String>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if let Some(count) = cache.get(design) {
        return *count;
    }

    if design.is_empty() {
        return 1;
    }

    let result = patterns
        .iter()
        .map(|pattern| {
            design
                .split_at_checked(pattern.len())
                .map(|(_, suffix)| {
                    design
                        .starts_with(pattern)
                        .then(|| count_every_possible_design(&suffix.to_string(), patterns, cache))
                        .unwrap_or(0)
                })
                .unwrap_or(0)
        })
        .sum();

    cache.insert(design.clone(), result);
    result
}

fn is_any_design_possible(design: &String, patterns: &HashSet<String>) -> bool {
    if design.is_empty() {
        return true;
    }

    let result = patterns.iter().any(|pattern| {
        design
            .split_at_checked(pattern.len())
            .map(|(_, suffix)| {
                design.starts_with(pattern) && is_any_design_possible(&suffix.to_string(), patterns)
            })
            .unwrap_or(false)
    });

    result
}
