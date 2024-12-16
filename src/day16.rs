use pathfinding::prelude::*;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Reindeer {
    pos: (i32, i32),
    dir: (i32, i32),
}

fn main() {
    let content = std::fs::read_to_string("src/inputs/input-day16.txt").unwrap();
    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start_pos = (x as i32, y as i32);
            } else if cell == 'E' {
                end_pos = (x as i32, y as i32);
            }
        }
    }

    let initial_state = Reindeer {
        pos: start_pos,
        dir: (1, 0),
    };

    let result = dijkstra(
        &initial_state,
        |state| get_successors(state, &grid),
        |state| state.pos == end_pos,
    )
    .unwrap();

    println!("The lowest possible score is {}", result.1);
}

fn get_successors(state: &Reindeer, grid: &Vec<Vec<char>>) -> Vec<(Reindeer, u32)> {
    let mut successors = Vec::new();

    let left_dir = (-state.dir.1, state.dir.0);
    successors.push((
        Reindeer {
            pos: state.pos,
            dir: left_dir,
        },
        1000,
    ));

    let right_dir = (state.dir.1, -state.dir.0);
    successors.push((
        Reindeer {
            pos: state.pos,
            dir: right_dir,
        },
        1000,
    ));

    let new_pos = (state.pos.0 + state.dir.0, state.pos.1 + state.dir.1);
    if new_pos.0 >= 0
        && new_pos.1 >= 0
        && new_pos.1 < grid.len() as i32
        && new_pos.0 < grid[0].len() as i32
        && grid[new_pos.1 as usize][new_pos.0 as usize] != '#'
    {
        successors.push((
            Reindeer {
                pos: new_pos,
                dir: state.dir,
            },
            1,
        ));
    }

    successors
}
