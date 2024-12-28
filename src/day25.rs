use std::fs;

fn main() {
    let input = fs::read_to_string("src/inputs/input-day25.txt").unwrap();
    let patterns: Vec<&str> = input.split("\n\n").collect();

    let (locks, keys): (Vec<&str>, Vec<&str>) = patterns.iter().partition(|&s| s.starts_with('#'));

    let lock_heights: Vec<Vec<u32>> = locks.iter().map(|&lock| calculate_heights(lock)).collect();
    let key_heights: Vec<Vec<u32>> = keys.iter().map(|&key| calculate_heights(key)).collect();

    let unique_pairs = key_heights
        .iter()
        .flat_map(|key| {
            lock_heights
                .iter()
                .filter(|lock| key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 5))
        })
        .count();

    println!("The number of unique pairs is {:?}", unique_pairs);
}

fn calculate_heights(pattern: &str) -> Vec<u32> {
    let digits: Vec<u32> = pattern
        .split('\n')
        .flat_map(|line| line.chars().map(|c| if c == '.' { 0 } else { 1 }))
        .collect();

    digits[5..30].chunks(5).fold(vec![0; 5], |acc, chunk| {
        acc.iter().zip(chunk.iter()).map(|(&a, &b)| a + b).collect()
    })
}
