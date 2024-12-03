use regex::Regex;
use regex::RegexBuilder;

fn main() {
    let content = std::fs::read_to_string("src/inputs/input-day3.txt").unwrap();
    let muls = Regex::new(r"mul\((?<X>\d+),(?<Y>\d+)\)").unwrap();
    let enablers = RegexBuilder::new(r"(?s)don't\(\)(.*?do\(\)|.*$)")
        .multi_line(true)
        .build()
        .unwrap();

    let cleaned_input = clear_line(&content, &enablers); // Cleaning is just for the second half
    let result = calculate_sum(&cleaned_input, &muls);

    println!("Adding multiplications result is {}", result);
}

fn calculate_sum(line: &str, muls: &Regex) -> u32 {
    muls.captures_iter(line)
        .map(|caps| {
            let x = caps["X"].parse::<u32>().unwrap();
            let y = caps["Y"].parse::<u32>().unwrap();
            x * y
        })
        .sum()
}

fn clear_line(line: &str, enablers: &Regex) -> String {
    enablers.replace_all(&line, "").to_string()
}
