use std::collections::HashMap;

fn main() {
    let content = std::fs::read_to_string("src/puzzles/puzzle-day11.txt").unwrap();

    let stones_number_1 = part1(content.clone());
    let stones_number_2 = part2(content);

    println!("After 25 blinks, I have {:?} stones", stones_number_1);
    println!("After 75 blinks, I have {:?} stones", stones_number_2);
}

fn part1(content: String) -> usize {
    let stones: Vec<u64> = content
        .split_whitespace()
        .map(|c| c.parse::<u64>().unwrap())
        .collect();

    (0..25)
        .fold(stones, |stones, _| stones.iter().flat_map(blink).collect())
        .len()
}

fn part2(content: String) -> usize {
    let stones: HashMap<u64, usize> = content
        .split_whitespace()
        .map(|c| (str::parse(c).unwrap(), 1))
        .collect();

    (0..75)
        .fold(stones, |stones, _| generate_stone_map(&stones))
        .values()
        .sum()
}

fn generate_stone_map(stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut partial_stones: HashMap<u64, usize> = HashMap::new();

    stones.iter().for_each(|(stone, count)| {
        for new_stone in blink(stone) {
            partial_stones
                .entry(new_stone)
                .and_modify(|new_count| *new_count += *count)
                .or_insert(*count);
        }
    });
    partial_stones
}

fn blink(n: &u64) -> Vec<u64> {
    match *n {
        0 => vec![1],
        n => get_even_length(&n)
            .map(|(s, length)| {
                let (first, last) = s.split_at(length / 2);
                vec![first.parse().unwrap(), last.parse().unwrap()]
            })
            .unwrap_or_else(|| vec![n * 2024]),
    }
}

fn get_even_length(n: &u64) -> Option<(String, usize)> {
    let s = n.to_string();
    let length = s.len();

    if length % 2 != 0 {
        return None;
    }

    Some((s, length))
}
