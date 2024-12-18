use std::collections::{HashMap, HashSet};

use pathfinding::prelude::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Reindeer {
    pos: Coordinate,
    dir: Direction,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
    line: i32,
    column: i32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Status {
    Free,
    Wall,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let content = std::fs::read_to_string("src/puzzles/puzzle-day16.txt").unwrap();
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    let mut maze: HashMap<Coordinate, Status> = HashMap::new();

    let mut start_pos = Coordinate { line: 0, column: 0 };
    let mut end_pos = Coordinate { line: 0, column: 0 };
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let coord = Coordinate {
                line: y as i32,
                column: x as i32,
            };
            match cell {
                'S' => {
                    start_pos = coord;
                    maze.insert(coord, Status::Free);
                }
                'E' => {
                    end_pos = coord;
                    maze.insert(coord, Status::Free);
                }
                '#' => {
                    maze.insert(coord, Status::Wall);
                }
                _ => {
                    maze.insert(coord, Status::Free);
                }
            }
        }
    }

    let initial_reindeer = Reindeer {
        pos: start_pos,
        dir: Direction::Right,
    };

    let lowest_score = part1(&initial_reindeer, end_pos, &maze);
    println!("The lowest possible score is {}", lowest_score);

    let tiles: u32 = part2(&initial_reindeer, end_pos, lowest_score, &maze);
    println!("Tiles number is {}", tiles);
}

fn part1(initial_reindeer: &Reindeer, goal: Coordinate, maze: &HashMap<Coordinate, Status>) -> u32 {
    dijkstra(
        initial_reindeer,
        |reindeer| get_successors(reindeer, maze),
        |reindeer| reindeer.pos == goal,
    )
    .unwrap()
    .1
}

fn part2(
    initial_reindeer: &Reindeer,
    goal: Coordinate,
    target_cost: u32,
    maze: &HashMap<Coordinate, Status>,
) -> u32 {
    let mut visited_tiles: HashSet<Coordinate> = HashSet::new();
    let mut prev_size = 0;
    let mut k = 1;

    loop {
        let result = yen(
            initial_reindeer,
            |reindeer| get_successors(reindeer, maze),
            |reindeer| reindeer.pos == goal,
            k,
        );

        result
            .iter()
            .filter(|path| path.1 == target_cost)
            .for_each(|path| {
                path.0.iter().for_each(|r| {
                    visited_tiles.insert(r.pos);
                })
            });

        if visited_tiles.len() == prev_size {
            break;
        }

        prev_size = visited_tiles.len();
        k += 1;
    }

    visited_tiles.len() as u32
}

fn get_successors(reindeer: &Reindeer, maze: &HashMap<Coordinate, Status>) -> Vec<(Reindeer, u32)> {
    let mut successors = Vec::new();

    let left_reindeer = Reindeer {
        pos: reindeer.pos,
        dir: get_new_direction(&reindeer.dir, true),
    };

    successors.push((left_reindeer, 1000));

    let right_reindeer = Reindeer {
        pos: reindeer.pos,
        dir: get_new_direction(&reindeer.dir, false),
    };

    successors.push((right_reindeer, 1000));

    let (line_offset, column_offset) = move_ahead(&reindeer.dir);
    let straight_reindeer = Reindeer {
        pos: Coordinate {
            line: reindeer.pos.line + line_offset,
            column: reindeer.pos.column + column_offset,
        },
        dir: reindeer.dir,
    };

    if is_valid_successor(&straight_reindeer.pos, maze) {
        successors.push((straight_reindeer, 1));
    }

    successors
}

fn is_valid_successor(coord: &Coordinate, maze: &HashMap<Coordinate, Status>) -> bool {
    maze.get_key_value(coord)
        .filter(|(_, status)| **status == Status::Free)
        .is_some()
}

fn get_new_direction(d: &Direction, clockwise: bool) -> Direction {
    match d {
        Direction::Up if clockwise => Direction::Right,
        Direction::Up => Direction::Left,
        Direction::Down if clockwise => Direction::Left,
        Direction::Down => Direction::Right,
        Direction::Left if clockwise => Direction::Up,
        Direction::Left => Direction::Down,
        Direction::Right if clockwise => Direction::Down,
        Direction::Right => Direction::Up,
    }
}

fn move_ahead(d: &Direction) -> (i32, i32) {
    match d {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}
