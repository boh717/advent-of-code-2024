use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // For first half
    let mut left_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut right_heap: BinaryHeap<Reverse<u32>> = BinaryHeap::new();
    let mut sum = 0;
    // For second half
    let mut count_right_occurences: HashMap<u32, u32> = HashMap::new();
    let mut similarity_score = 0;

    let file = File::open("src/inputs/input-day1.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let safe_line = line.unwrap();
        let mut numbers = safe_line.split_whitespace();

        let left_num = numbers.next().unwrap().parse::<u32>().unwrap();
        let right_num = numbers.next().unwrap().parse::<u32>().unwrap();

        left_heap.push(Reverse(left_num));
        right_heap.push(Reverse(right_num));
        *count_right_occurences.entry(right_num).or_insert(0) += 1;
    }

    while let (Some(Reverse(left_number)), Some(Reverse(right_number))) =
        (left_heap.pop(), right_heap.pop())
    {
        // For first half
        sum += left_number.abs_diff(right_number);

        // For second half
        let left_occurence_in_right = count_right_occurences.get(&left_number).unwrap_or(&0);
        similarity_score += left_number * left_occurence_in_right;
    }

    println!("The total distance between the lists is {}", sum);
    println!("The similarity score is {}", similarity_score);
}
