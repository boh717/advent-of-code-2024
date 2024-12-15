use std::collections::HashMap;

#[derive(Clone, Debug, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
enum Status {
    Wall,
    Box,
    Free,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Coordinate {
    line: i32,
    column: i32,
}

#[derive(Clone, Debug)]
struct Cell {
    coordinate: Coordinate,
    status: Status,
}

#[derive(Clone, Debug, Copy)]
struct Robot {
    coordinate: Coordinate,
}

impl Robot {
    fn new(line: i32, column: i32) -> Self {
        Self {
            coordinate: Coordinate { line, column },
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("src/inputs/input-day15.txt").unwrap();
    let data: Vec<&str> = content.split("\n\n").collect();

    let lines: Vec<&str> = data[0].lines().collect();
    let instructions: String = data[1].replace("\n", "");

    let gps = part1(&lines, instructions);

    println!("Sum of all boxes' GPS coordinates is {}", gps);
}

fn part1(lines: &Vec<&str>, instructions: String) -> i32 {
    let (mut map, mut robot) = create_map_and_robot(&lines);

    for c in instructions.chars() {
        let direction = match c {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!(),
        };
        match get_next_cell(&robot, &map, direction) {
            None => continue,
            Some(cell_to_check) => {
                if cell_to_check.status == Status::Free {
                    robot.coordinate = cell_to_check.coordinate;
                } else {
                    map.get_mut(&robot.coordinate).map(|c| *c = Status::Free);
                    robot.coordinate = cell_to_check.coordinate;
                    update_box_position(&cell_to_check, direction, &mut map);
                }
            }
        }
    }

    map.iter()
        .filter(|(_, v)| **v == Status::Box)
        .map(|(k, _)| 100 * k.line + k.column)
        .sum()
}

fn create_map_and_robot(lines: &Vec<&str>) -> (HashMap<Coordinate, Status>, Robot) {
    let mut robot = Robot::new(0, 0);
    let mut map: HashMap<Coordinate, Status> = HashMap::new();

    for (line_index, line) in lines.iter().enumerate() {
        for (column_index, character) in line.chars().enumerate() {
            let coordinate = Coordinate {
                line: line_index as i32,
                column: column_index as i32,
            };

            match character {
                '#' | '.' | 'O' => {
                    map.insert(coordinate, get_status(character));
                }
                _ => {
                    robot = Robot::new(line_index as i32, column_index as i32);
                    map.insert(coordinate, Status::Free);
                }
            }
        }
    }

    (map, robot)
}

fn update_box_position(cell: &Cell, direction: Direction, map: &mut HashMap<Coordinate, Status>) {
    let initial_box_coord = cell.coordinate;

    let free_space_coord = std::iter::successors(Some(cell.coordinate), |&coord| {
        let (line_offset, column_offset) = get_offset(direction);
        Some(Coordinate {
            line: coord.line.wrapping_add(line_offset),
            column: coord.column.wrapping_add(column_offset),
        })
    })
    .find(|coord| map.get(coord) == Some(&Status::Free))
    .unwrap();

    map.insert(initial_box_coord, Status::Free);
    map.insert(free_space_coord, Status::Box);
}

fn get_next_cell(
    robot: &Robot,
    map: &HashMap<Coordinate, Status>,
    direction: Direction,
) -> Option<Cell> {
    let (line_offset, column_offset) = get_offset(direction);

    let coordinate_to_check = Coordinate {
        line: robot.coordinate.line.wrapping_add(line_offset),
        column: robot.coordinate.column.wrapping_add(column_offset),
    };

    map.get_key_value(&coordinate_to_check)
        .map(|(k, v)| Cell {
            coordinate: *k,
            status: *v,
        })
        .filter(|c| is_robot_free_to_move(c, direction, map))
}

fn is_robot_free_to_move(
    cell: &Cell,
    direction: Direction,
    map: &HashMap<Coordinate, Status>,
) -> bool {
    match cell.status {
        Status::Wall => false,
        Status::Free => true,
        Status::Box => std::iter::successors(Some(cell.coordinate), |&coord| {
            let (line_offset, column_offset) = get_offset(direction);
            Some(Coordinate {
                line: coord.line.wrapping_add(line_offset),
                column: coord.column.wrapping_add(column_offset),
            })
        })
        .take_while(|coord| map.get(coord) != Some(&Status::Wall))
        .filter_map(|coord| map.get(&coord))
        .any(|status| *status == Status::Free),
    }
}

fn get_offset(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Right => (0, 1),
        Direction::Left => (0, -1),
    }
}

fn get_status(character: char) -> Status {
    match character {
        '#' => Status::Wall,
        'O' => Status::Box,
        _ => Status::Free,
    }
}
