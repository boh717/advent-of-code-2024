use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
    line: i32,
    column: i32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Status {
    Free,
    Corrupted,
}

const WIDTH: i32 = 7;
const HEIGTH: i32 = 7;
const BYTES_NUMBER: usize = 12;

// Puzzle data
// const WIDTH: i32 = 71;
// const HEIGTH: i32 = 71;
// const BYTES_NUMBER: usize = 1024;

fn main() {
    let content = std::fs::read_to_string("src/inputs/input-day18.txt").unwrap();

    let memory_space: HashMap<Coordinate, Status> = (0..WIDTH)
        .flat_map(|i| {
            (0..HEIGTH).map(move |j| {
                (
                    Coordinate {
                        line: j as i32,
                        column: i as i32,
                    },
                    Status::Free,
                )
            })
        })
        .collect();
    let byte_positions: Vec<Coordinate> = content
        .lines()
        .map(|line| {
            let raw_coord: Vec<String> = line.split(",").map(|l| l.to_string()).collect();
            Coordinate {
                line: raw_coord[1].parse::<i32>().unwrap(),
                column: raw_coord[0].parse::<i32>().unwrap(),
            }
        })
        .collect();

    let mininum_steps = part1(&memory_space, &byte_positions);

    println!(
        "The minimum number of steps needed to reach the exit is {}",
        mininum_steps
    );

    let coordinate_that_stops_runaway = part2(&memory_space, &byte_positions);
    println!(
        "The coordinate that stops the runaway is ({},{})",
        coordinate_that_stops_runaway.column, coordinate_that_stops_runaway.line
    );
}

fn part1(memory_space: &HashMap<Coordinate, Status>, byte_positions: &Vec<Coordinate>) -> u32 {
    let mut memory = memory_space.clone();
    initialize_memory(&mut memory, byte_positions);

    let initial_coord = Coordinate { line: 0, column: 0 };
    let goal_coord = Coordinate {
        line: WIDTH - 1,
        column: HEIGTH - 1,
    };

    dijkstra(
        &initial_coord,
        |c| get_successors(c, &memory),
        |end_coord| *end_coord == goal_coord,
    )
    .unwrap()
    .1
}

fn part2(
    memory_space: &HashMap<Coordinate, Status>,
    byte_positions: &Vec<Coordinate>,
) -> Coordinate {
    let mut memory = memory_space.clone();
    initialize_memory(&mut memory, byte_positions);

    let initial_coord = Coordinate { line: 0, column: 0 };
    let goal_coord = Coordinate {
        line: WIDTH - 1,
        column: HEIGTH - 1,
    };

    byte_positions
        .iter()
        .skip(BYTES_NUMBER)
        .find(|coord| {
            memory.insert(**coord, Status::Corrupted);
            dijkstra(
                &initial_coord,
                |c| get_successors(c, &memory),
                |end_coord| *end_coord == goal_coord,
            )
            .is_none()
        })
        .cloned()
        .unwrap()
}

fn initialize_memory(
    memory_space: &mut HashMap<Coordinate, Status>,
    byte_positions: &Vec<Coordinate>,
) {
    byte_positions
        .iter()
        .take(BYTES_NUMBER)
        .for_each(|c| _ = memory_space.insert(c.clone(), Status::Corrupted));
}

fn get_successors(
    current_node: &Coordinate,
    memory_space: &HashMap<Coordinate, Status>,
) -> Vec<(Coordinate, u32)> {
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    directions
        .into_iter()
        .filter_map(|(dx, dy)| {
            let next = Coordinate {
                line: current_node.line + dx,
                column: current_node.column + dy,
            };
            is_valid_successor(&next, memory_space).map(|coord| (coord, 1))
        })
        .collect()
}

fn is_valid_successor(
    coord: &Coordinate,
    memory_space: &HashMap<Coordinate, Status>,
) -> Option<Coordinate> {
    memory_space
        .get_key_value(coord)
        .filter(|(_, status)| **status == Status::Free)
        .map(|(c, _)| c.clone())
}
