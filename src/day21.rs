use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate(i32, i32);

fn main() {
    let content = std::fs::read_to_string("src/inputs/input-day21.txt").unwrap();
    let codes: Vec<String> = content.lines().map(String::from).collect();

    let numpad = create_numpad();
    let numpad_coord = create_numpad_coord();
    let dirpad = create_dirpad();
    let dirpad_coord = create_dirpad_coord();

    let sequences_1 = transform_sequences(&codes, &numpad, &numpad_coord);
    let sequences_2 = transform_sequences(&sequences_1, &dirpad, &dirpad_coord);
    let sequences_3 = transform_sequences(&sequences_2, &dirpad, &dirpad_coord);

    let result: u32 = sequences_3
        .iter()
        .zip(codes.iter())
        .map(|(last, code)| {
            let number: u32 = code
                .chars()
                .filter(|d| d.is_numeric())
                .collect::<String>()
                .parse()
                .unwrap();
            number * last.len() as u32
        })
        .sum();

    println!("The sum of the complexities is {}", result);
}

fn transform_sequences(
    input: &[String],
    pad: &HashMap<char, Coordinate>,
    pad_coord: &HashMap<Coordinate, &str>,
) -> Vec<String> {
    input
        .iter()
        .map(|code| {
            let mut digits: Vec<char> = code.chars().collect();
            digits.insert(0, 'A');
            digits
                .windows(2)
                .map(|w| {
                    let start = pad.get(&w[0]).unwrap();
                    let end = pad.get(&w[1]).unwrap();
                    let path = dijkstra(
                        start,
                        |c| get_successors(c, pad_coord),
                        |end_coord| *end_coord == *end,
                    )
                    .unwrap()
                    .0;
                    format!("{}{}", get_dir_pad_buttons(path), "A")
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect()
}

fn get_successors(
    current_node: &Coordinate,
    pad: &HashMap<Coordinate, &str>,
) -> Vec<(Coordinate, i32)> {
    let directions = [((-1, 0), 2), ((1, 0), 3), ((0, 1), 4), ((0, -1), 1)];

    directions
        .into_iter()
        .filter_map(|((dx, dy), cost)| {
            let next = Coordinate(current_node.0 + dx, current_node.1 + dy);
            is_valid_successor(&next, pad).map(|coord| (coord, cost))
        })
        .collect()
}

fn is_valid_successor(coord: &Coordinate, pad: &HashMap<Coordinate, &str>) -> Option<Coordinate> {
    pad.get_key_value(coord).map(|(c, _)| c.clone())
}

fn get_dir_pad_buttons(coord: Vec<Coordinate>) -> String {
    let directions = [((-1, 0), '^'), ((1, 0), 'v'), ((0, -1), '<'), ((0, 1), '>')];
    let commands: Vec<char> = coord
        .windows(2)
        .map(|w| {
            let dir_command: Vec<char> = directions
                .into_iter()
                .filter_map(|((dx, dy), dir)| {
                    let c = Coordinate(w[1].0 - dx, w[1].1 - dy);
                    if w[0] == c {
                        Some(dir)
                    } else {
                        None
                    }
                })
                .collect();
            dir_command[0]
        })
        .collect();

    commands.iter().collect::<String>()
}

fn create_numpad() -> HashMap<char, Coordinate> {
    HashMap::from([
        ('7', Coordinate(0, 0)),
        ('8', Coordinate(0, 1)),
        ('9', Coordinate(0, 2)),
        ('4', Coordinate(1, 0)),
        ('5', Coordinate(1, 1)),
        ('6', Coordinate(1, 2)),
        ('1', Coordinate(2, 0)),
        ('2', Coordinate(2, 1)),
        ('3', Coordinate(2, 2)),
        ('0', Coordinate(3, 1)),
        ('A', Coordinate(3, 2)),
    ])
}

fn create_dirpad() -> HashMap<char, Coordinate> {
    HashMap::from([
        ('^', Coordinate(0, 1)),
        ('A', Coordinate(0, 2)),
        ('<', Coordinate(1, 0)),
        ('v', Coordinate(1, 1)),
        ('>', Coordinate(1, 2)),
    ])
}

fn create_numpad_coord() -> HashMap<Coordinate, &'static str> {
    HashMap::from([
        (Coordinate(0, 0), "7"),
        (Coordinate(0, 1), "8"),
        (Coordinate(0, 2), "9"),
        (Coordinate(1, 0), "4"),
        (Coordinate(1, 1), "5"),
        (Coordinate(1, 2), "6"),
        (Coordinate(2, 0), "1"),
        (Coordinate(2, 1), "2"),
        (Coordinate(2, 2), "3"),
        (Coordinate(3, 1), "0"),
        (Coordinate(3, 2), "A"),
    ])
}

fn create_dirpad_coord() -> HashMap<Coordinate, &'static str> {
    HashMap::from([
        (Coordinate(0, 1), "^"),
        (Coordinate(0, 2), "A"),
        (Coordinate(1, 0), "<"),
        (Coordinate(1, 1), "v"),
        (Coordinate(1, 2), ">"),
    ])
}
