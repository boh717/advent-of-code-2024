use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward,
    Backward,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct GridSize {
    lines: usize,
    columns: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Coordinate {
    line: usize,
    column: usize,
}

impl Coordinate {
    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate {
            line: self.line + other.line,
            column: self.column + other.column,
        }
    }

    fn subtract(self, other: Coordinate) -> Coordinate {
        Coordinate {
            line: self.line - other.line,
            column: self.column - other.column,
        }
    }
}

fn main() {
    let file = File::open("src/inputs/input-day8.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let grid_size = GridSize {
        lines: lines.len(),
        columns: lines[0].len(),
    };

    let mut antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();

    for (line_index, line) in lines.iter().enumerate() {
        for (column_index, character) in line.chars().enumerate() {
            let coord = Coordinate {
                line: line_index,
                column: column_index,
            };
            if character != '.' {
                antennas
                    .entry(character)
                    .and_modify(|v| v.push(coord))
                    .or_insert(vec![coord]);
            }
        }
    }

    let antinodes = part1(&mut antennas, grid_size);
    let repeating_antinodes = part2(&mut antennas, grid_size);

    println!("The number of unique antinodes are {}", antinodes.len());
    println!(
        "The number of repeating antinodes are {}",
        repeating_antinodes.len()
    );
}

fn part1(
    antennas: &mut HashMap<char, Vec<Coordinate>>,
    grid_size: GridSize,
) -> HashSet<Coordinate> {
    let mut antinodes: HashSet<Coordinate> = HashSet::new();
    for antenna_type in antennas.iter() {
        let antenna_type_coordinates = antenna_type.1;

        for (index, i) in antenna_type_coordinates.iter().enumerate() {
            for j in antenna_type_coordinates[index + 1..].iter() {
                let antenna1 = Coordinate {
                    line: i.line,
                    column: i.column,
                };
                let antenna2 = Coordinate {
                    line: j.line,
                    column: j.column,
                };

                let diff = antenna2.subtract(antenna1);
                get_next_antinode(antenna1, diff, grid_size, Direction::Backward)
                    .and_then(|c| antinodes.insert(c).then_some(()));
                get_next_antinode(antenna2, diff, grid_size, Direction::Forward)
                    .and_then(|c| antinodes.insert(c).then_some(()));
            }
        }
    }

    antinodes
}

fn part2(
    antennas: &mut HashMap<char, Vec<Coordinate>>,
    grid_size: GridSize,
) -> HashSet<Coordinate> {
    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    for antenna_type in antennas.iter() {
        let antenna_type_coordinates = antenna_type.1;

        for (index, i) in antenna_type_coordinates.iter().enumerate() {
            for j in antenna_type_coordinates[index + 1..].iter() {
                let antenna1 = Coordinate {
                    line: i.line,
                    column: i.column,
                };
                let antenna2 = Coordinate {
                    line: j.line,
                    column: j.column,
                };
                let diff = antenna2.subtract(antenna1);

                antinodes.insert(antenna1);
                antinodes.insert(antenna2);

                let mut iter_coord: Coordinate = antenna1;

                while let Some(coord) =
                    get_next_antinode(iter_coord, diff, grid_size, Direction::Backward)
                {
                    antinodes.insert(coord);
                    iter_coord = coord;
                }

                iter_coord = antenna2;

                while let Some(coord) =
                    get_next_antinode(iter_coord, diff, grid_size, Direction::Forward)
                {
                    antinodes.insert(coord);
                    iter_coord = coord;
                }
            }
        }
    }

    antinodes
}

fn is_valid_coordinate(coord: Coordinate, grid_size: GridSize) -> bool {
    coord.line < grid_size.lines && coord.column < grid_size.columns
}

fn get_next_antinode(
    coord: Coordinate,
    diff: Coordinate,
    grid_size: GridSize,
    direction: Direction,
) -> Option<Coordinate> {
    match direction {
        Direction::Forward => {
            let new_coord = coord.add(diff);
            is_valid_coordinate(new_coord, grid_size).then_some(new_coord)
        }
        Direction::Backward => {
            let new_coord = coord.subtract(diff);
            is_valid_coordinate(new_coord, grid_size).then_some(new_coord)
        }
    }
}
