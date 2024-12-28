use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let re = Regex::new(
        "(?<Operand1>.{0,3}) (?<Operator>(AND|OR|XOR)) (?<Operand2>.{0,3}) -> (?<Result>.{0,3})",
    )
    .unwrap();
    let raw_data: Vec<String> = std::fs::read_to_string("src/inputs/input-day24.txt")
        .unwrap()
        .split("\n\n")
        .map(|l| l.to_string())
        .collect();

    let mut wires: HashMap<String, u8> = raw_data[0]
        .lines()
        .map(|s| {
            let (wire, value) = s.split_once(":").unwrap();
            (wire.to_string(), value.trim().parse().unwrap())
        })
        .collect();

    let mut gates: VecDeque<&str> = raw_data[1].split("\n").collect();

    while let Some(element) = gates.pop_front() {
        let caps = re.captures(element).unwrap();
        let operand1 = caps["Operand1"].to_string();
        let operand2 = caps["Operand2"].to_string();
        let operator = &caps["Operator"];
        let result = caps["Result"].to_string();

        if let (Some(a), Some(b)) = (wires.get(&operand1), wires.get(&operand2)) {
            let r = process(a, b, &operator);
            wires.insert(result, r);
        } else {
            gates.push_back(element);
        }
    }

    let mut z_wires: Vec<_> = wires
        .iter()
        .filter_map(|(k, v)| {
            k.strip_prefix('z')
                .and_then(|num| num.parse().ok())
                .map(|num: String| (num, v))
        })
        .collect();
    z_wires.sort_by(|a, b| b.0.cmp(&a.0));
    let bin_number: String = z_wires.iter().map(|(_, v)| v.to_string()).collect();

    println!(
        "Decimal number on the wires is {}",
        isize::from_str_radix(&bin_number, 2).unwrap()
    );
}

fn process(operand1: &u8, operand2: &u8, operator: &str) -> u8 {
    let o1 = *operand1 != 0;
    let o2 = *operand2 != 0;
    let res = match operator {
        "AND" => o1 && o2,
        "OR" => o1 || o2,
        "XOR" => o1 ^ o2,
        _ => panic!(),
    };

    res as u8
}
