use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Coordinate {
    line: i32,
    column: i32,
}

impl Coordinate {
    fn distance(self, other: Coordinate) -> u32 {
        ((self.line - other.line).abs() + (self.column - other.column).abs()) as u32
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Status {
    Free,
    Wall,
}

fn main() {
    let content = std::fs::read_to_string("src/puzzles/puzzle-day20.txt").unwrap();
    let mut track: HashMap<Coordinate, Status> = HashMap::new();

    let mut start = Coordinate { line: 0, column: 0 };
    let mut end = Coordinate { line: 0, column: 0 };
    for (x, row) in content.lines().enumerate() {
        for (y, column) in row.char_indices() {
            let coord = Coordinate {
                line: x as i32,
                column: y as i32,
            };
            match column {
                'S' => {
                    start = coord;
                    track.insert(coord, Status::Free);
                }
                'E' => {
                    end = coord;
                    track.insert(coord, Status::Free);
                }
                '#' => {
                    track.insert(coord, Status::Wall);
                }
                _ => {
                    track.insert(coord, Status::Free);
                }
            }
        }
    }

    let picoseconds_to_save = 100;

    let cheats_1 = solve(&track, start, end, picoseconds_to_save, 2);
    println!(
        "Cheats that save at least {} picoseconds within 2 steps are {}",
        picoseconds_to_save, cheats_1
    );

    let cheats_2 = solve(&track, start, end, picoseconds_to_save, 20);
    println!(
        "Cheats that save at least {} picoseconds within 20 steps are {}",
        picoseconds_to_save, cheats_2
    );
}

fn solve(
    track: &HashMap<Coordinate, Status>,
    start: Coordinate,
    end: Coordinate,
    picoseconds_to_save: u32,
    max_cheat_steps: u32,
) -> u64 {
    let optimal_path = dijkstra(
        &start,
        |c| get_successors(c, &track),
        |end_coord| *end_coord == end,
    )
    .unwrap()
    .0;

    let mut distances: HashMap<Coordinate, u32> = HashMap::new();
    optimal_path.iter().enumerate().for_each(|(index, c)| {
        distances.insert(*c, index as u32);
    });

    let mut cheats: u64 = 0;
    for (i, start_point) in optimal_path.iter().enumerate() {
        for end_point in optimal_path.iter().skip(i + picoseconds_to_save as usize) {
            let path_distance = distances[end_point] - distances[start_point];
            let manhattan_distance = start_point.distance(*end_point);

            if (path_distance - manhattan_distance) >= picoseconds_to_save as u32
                && manhattan_distance <= max_cheat_steps
            {
                cheats += 1;
            }
        }
    }

    cheats
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
