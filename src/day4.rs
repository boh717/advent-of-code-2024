use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("src/inputs/input-day4.txt").unwrap();
    let reader = BufReader::new(file);
    let mut matrix: HashMap<(usize, usize), String> = HashMap::new();

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    for (line_index, line) in lines.iter().enumerate() {
        for (column_index, character) in line.chars().enumerate() {
            matrix.insert((line_index, column_index), character.to_string());
        }
    }

    let xmas_count = part1(&lines, &matrix);
    let crossed_mas_count = part2(&lines, &matrix);

    println!("XMAS count is {}", xmas_count);
    println!("Crossed MAS count is {}", crossed_mas_count);
}

fn part1(lines: &Vec<String>, matrix: &HashMap<(usize, usize), String>) -> u32 {
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'X')
                .map(move |(column_index, _)| count_xmas_word(line_index, column_index, matrix))
        })
        .sum()
}

fn part2(lines: &Vec<String>, matrix: &HashMap<(usize, usize), String>) -> u32 {
    lines
        .iter()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == 'A')
                .filter(move |(column_index, _)| {
                    right_diagonal(line_index, *column_index, matrix)
                        && left_diagonal(line_index, *column_index, matrix)
                })
        })
        .count() as u32
}

fn right_diagonal(
    line_index: usize,
    column_index: usize,
    matrix: &HashMap<(usize, usize), String>,
) -> bool {
    let (diagonal_up_right, diagonal_down_left) = (
        matrix.get(&(line_index - 1, column_index + 1)),
        matrix.get(&(line_index + 1, column_index - 1)),
    );

    (diagonal_up_right.is_some_and(is_m) && diagonal_down_left.is_some_and(is_s))
        || (diagonal_up_right.is_some_and(is_s) && diagonal_down_left.is_some_and(is_m))
}

fn left_diagonal(
    line_index: usize,
    column_index: usize,
    matrix: &HashMap<(usize, usize), String>,
) -> bool {
    let (diagonal_up_left, diagonal_down_right) = (
        matrix.get(&(line_index - 1, column_index - 1)),
        matrix.get(&(line_index + 1, column_index + 1)),
    );

    (diagonal_up_left.is_some_and(is_m) && diagonal_down_right.is_some_and(is_s))
        || (diagonal_up_left.is_some_and(is_s) && diagonal_down_right.is_some_and(is_m))
}

fn count_xmas_word(
    line_index: usize,
    column_index: usize,
    matrix: &HashMap<(usize, usize), String>,
) -> u32 {
    let directions = [
        // horizontal offsets
        [(0, 1), (0, 2), (0, 3)],
        [(0, -1), (0, -2), (0, -3)],
        // vertical offsets
        [(1, 0), (2, 0), (3, 0)],
        [(-1, 0), (-2, 0), (-3, 0)],
        // diagonals offsets
        [(1, 1), (2, 2), (3, 3)],
        [(1, -1), (2, -2), (3, -3)],
        [(-1, 1), (-2, 2), (-3, 3)],
        [(-1, -1), (-2, -2), (-3, -3)],
    ];

    directions
        .iter()
        .filter(|&direction| {
            let coordinates: Vec<(usize, usize)> = direction
                .iter()
                .map(|(line_offset, column_offset)| {
                    (
                        (line_index as i32 + line_offset) as usize,
                        (column_index as i32 + column_offset) as usize,
                    )
                })
                .collect();

            let chars: Vec<_> = coordinates
                .iter()
                .filter_map(|pos| matrix.get(pos))
                .collect();

            chars.len() == 3 && is_mas(&chars[0], &chars[1], &chars[2])
        })
        .count() as u32
}

fn is_mas(m: &String, a: &String, s: &String) -> bool {
    is_m(m) && is_a(a) && is_s(s)
}

fn is_m(m: &String) -> bool {
    m == "M"
}

fn is_a(a: &String) -> bool {
    a == "A"
}

fn is_s(s: &String) -> bool {
    s == "S"
}
