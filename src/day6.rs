use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Status {
    Free,
    Obstructed,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Coordinate {
    line: u32,
    column: u32,
}

#[derive(Clone, Debug)]
struct Cell {
    coordinate: Coordinate,
    status: Status,
}

#[derive(Clone, Debug, Copy)]
struct Guard {
    coordinate: Coordinate,
    direction: Direction,
}

impl Guard {
    fn new(line: u32, column: u32, character: char) -> Self {
        let direction = match character {
            '^' => Direction::Up,
            '>' => Direction::Right,
            '<' => Direction::Left,
            'v' => Direction::Down,
            _ => panic!("Invalid guard character"),
        };
        Self {
            coordinate: Coordinate { line, column },
            direction,
        }
    }

    fn with_cell(self, cell: Cell) -> Self {
        Self {
            coordinate: cell.coordinate,
            direction: self.direction,
        }
    }

    fn with_new_direction(self) -> Self {
        let direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        Self {
            direction,
            coordinate: self.coordinate,
        }
    }
}

fn main() {
    let file = File::open("src/inputs/input-day6.txt").unwrap();
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let visited_positions = part1(&lines);
    let loops_number = part2(&lines, &visited_positions);

    println!("The guard visited {} positions", visited_positions.len());
    println!("The possible loops identified are {}", loops_number);
}

fn part1(lines: &Vec<String>) -> HashSet<Coordinate> {
    let mut visited_positions: HashSet<Coordinate> = HashSet::new();

    let (map, mut guard) = create_map_and_guard(lines);
    visited_positions.insert(guard.coordinate);

    while let Some(next_cell) = get_next_cell(&guard, &map) {
        visited_positions.insert(guard.coordinate);
        guard = move_guard(guard, next_cell);
    }

    visited_positions.insert(guard.coordinate);
    visited_positions
}

fn part2(lines: &Vec<String>, visited_positions: &HashSet<Coordinate>) -> u32 {
    let (map, initial_guard) = create_map_and_guard(lines);
    let initial_coordinate = initial_guard.coordinate;

    visited_positions
        .iter()
        .filter(|coord| **coord != initial_coordinate)
        .filter(|coord| {
            let mut modified_map = map.clone();
            modified_map.insert(**coord, Status::Obstructed);

            is_infinite_loop(&modified_map, initial_guard)
        })
        .count() as u32
}

fn is_infinite_loop(map: &HashMap<Coordinate, Status>, initial_guard: Guard) -> bool {
    let max_iterations = 10_000; // Dirty and faster workaround
    let mut current_guard = initial_guard;

    for _ in 0..max_iterations {
        match get_next_cell(&current_guard, map) {
            Some(next_cell) => {
                current_guard = move_guard(current_guard, next_cell);
            }
            None => return false,
        }
    }
    return true;
}

fn create_map_and_guard(lines: &Vec<String>) -> (HashMap<Coordinate, Status>, Guard) {
    let mut guard = Guard::new(0, 0, '^');
    let mut map: HashMap<Coordinate, Status> = HashMap::new();
    for (line_index, line) in lines.iter().enumerate() {
        for (column_index, character) in line.chars().enumerate() {
            let coordinate = Coordinate {
                line: line_index as u32,
                column: column_index as u32,
            };

            match character {
                '#' | '.' => {
                    map.insert(coordinate, get_map_status(character));
                }
                _ => {
                    guard = Guard::new(line_index as u32, column_index as u32, character);
                    map.insert(coordinate, Status::Free);
                }
            }
        }
    }
    (map, guard)
}

fn get_map_status(character: char) -> Status {
    match character {
        '#' => Status::Obstructed,
        _ => Status::Free,
    }
}

fn get_next_cell(guard: &Guard, map: &HashMap<Coordinate, Status>) -> Option<Cell> {
    let (line_offset, column_offset) = match guard.direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Right => (0, 1),
        Direction::Left => (0, -1),
    };

    let coordinate_to_check = Coordinate {
        line: guard.coordinate.line.wrapping_add(line_offset as u32),
        column: guard.coordinate.column.wrapping_add(column_offset as u32),
    };

    map.get(&coordinate_to_check).map(|status| Cell {
        coordinate: coordinate_to_check,
        status: *status,
    })
}

fn move_guard(guard: Guard, next_cell: Cell) -> Guard {
    match next_cell.status {
        Status::Free => guard.with_cell(next_cell),
        Status::Obstructed => guard.with_new_direction(),
    }
}
