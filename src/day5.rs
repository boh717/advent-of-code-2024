use std::collections::HashSet;

fn main() {
    let content = std::fs::read_to_string("src/puzzles/puzzle-day5.txt").unwrap();
    let rules_and_updates: Vec<_> = content.split("\n\n").collect();
    let rules = rules_and_updates[0].split("\n").map(|s| s.to_string());
    let updates: Vec<String> = rules_and_updates[1]
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    let mut rules_set: HashSet<String> = HashSet::new();
    for rule in rules {
        rules_set.insert(rule);
    }

    let count_valid_middle_elements: u32 = part1(&updates, &rules_set);
    let count_invalid_middle_elements: u32 = part2(&updates, &rules_set);
    println!(
        "The valid middle elements sum is {}",
        count_valid_middle_elements
    );
    println!(
        "The invalid middle elements sum is {}",
        count_invalid_middle_elements
    );
}

fn part1(updates: &Vec<String>, rules_set: &HashSet<String>) -> u32 {
    updates
        .iter()
        .map(|u| u.split(',').map(String::from).collect::<Vec<String>>())
        .filter(|v| is_valid_update(v, rules_set))
        .map(get_middle_page_number)
        .sum()
}

fn part2(updates: &Vec<String>, rules_set: &HashSet<String>) -> u32 {
    updates
        .iter()
        .map(|u| u.split(',').map(String::from).collect::<Vec<String>>())
        .filter(|v| !is_valid_update(v, rules_set))
        .map(|v| transform_sequence(v, rules_set))
        .map(get_middle_page_number)
        .sum()
}

fn is_valid_update(update: &Vec<String>, rules_set: &HashSet<String>) -> bool {
    update.iter().enumerate().all(|(i, e)| {
        let (_, subsequence) = update.split_at(i + 1);
        is_subsequence_valid(e, &subsequence.to_vec(), rules_set)
    })
}

fn is_subsequence_valid(
    head: &String,
    subsequence: &Vec<String>,
    rules_set: &HashSet<String>,
) -> bool {
    subsequence
        .iter()
        .all(|e| rules_set.contains(&format!("{}|{}", head, e)))
}

fn get_middle_page_number(update: Vec<String>) -> u32 {
    update[update.len() / 2].parse().unwrap()
}

fn transform_sequence(update: Vec<String>, rules_set: &HashSet<String>) -> Vec<String> {
    let mut result = update;

    // Sort the sequence according to the rules
    result.sort_by(|a, b| {
        if rules_set.contains(&format!("{}|{}", a, b)) {
            std::cmp::Ordering::Less
        } else if rules_set.contains(&format!("{}|{}", b, a)) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    });

    result
}
