use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

const WIDTH: i64 = 11;
const HEIGTH: i64 = 7;
const MEDIAN_WIDTH: i64 = 5;
const MEDIAN_HEIGTH: i64 = 3;

// Puzzle data
// const WIDTH: i64 = 101;
// const HEIGTH: i64 = 103;
// const MEDIAN_WIDTH: i64 = 50;
// const MEDIAN_HEIGTH: i64 = 51;

fn main() {
    let content = std::fs::read_to_string("src/inputs/input-day14.txt").unwrap();

    let robots: Vec<Robot> = content
        .split("\n")
        .map(|data| {
            let d = data.split(" ").collect::<Vec<&str>>();

            let p: Vec<i64> = d[0].split("=").collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| v.parse().unwrap())
                .collect();
            let v: Vec<i64> = d[1].split("=").collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|v| v.parse().unwrap())
                .collect();

            Robot {
                position: (p[0], p[1]),
                velocity: (v[0], v[1]),
            }
        })
        .collect();

    let safety_factor = part1(&robots);
    println!("The safety factor after 100 seconds is {}", safety_factor);

    let iteration_xmas_tree = part2(&robots);
    println!(
        "The iteration where Christmas tree shows up is {}",
        iteration_xmas_tree
    );
}

fn part1(robots: &Vec<Robot>) -> u64 {
    let mut quadrants: HashMap<i64, i64> = HashMap::new();
    let new_robots: Vec<Robot> = robots
        .iter()
        .map(|r| calculate_next_position(r, Some(100)))
        .filter(|r| r.position.0 != MEDIAN_WIDTH && r.position.1 != MEDIAN_HEIGTH)
        .collect();

    for r in new_robots {
        let quadrant = match (r.position.0, r.position.1) {
            (0..MEDIAN_WIDTH, 0..MEDIAN_HEIGTH) => 1,
            (MEDIAN_WIDTH.., 0..MEDIAN_HEIGTH) => 2,
            (0..MEDIAN_WIDTH, MEDIAN_HEIGTH..) => 3,
            (MEDIAN_WIDTH.., MEDIAN_HEIGTH..) => 4,
            _ => panic!(),
        };
        quadrants
            .entry(quadrant)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    quadrants.values().fold(1, |acc, e| acc * (*e as u64))
}

fn part2(robots: &Vec<Robot>) -> u64 {
    let mut new_robots: Vec<Robot> = robots.clone();
    let mut iteration: i64 = 1;

    loop {
        new_robots = new_robots
            .iter()
            .map(|r| calculate_next_position(r, None))
            .collect();
        if !xmas_tree_iteration(&new_robots) {
            iteration += 1;
        } else {
            return iteration as u64;
        }
    }
}

fn xmas_tree_iteration(robots: &Vec<Robot>) -> bool {
    let mut distinct_positions: HashSet<(i64, i64)> = HashSet::new();
    for r in robots {
        distinct_positions.insert((r.position.0, r.position.1));
    }

    distinct_positions.len() == robots.len()
}

fn calculate_next_position(robot: &Robot, seconds: Option<i64>) -> Robot {
    let s = match seconds {
        None => 1,
        Some(i) => i,
    };
    let new_x = (robot.position.0 + robot.velocity.0 * s).rem_euclid(WIDTH);
    let new_y = (robot.position.1 + robot.velocity.1 * s).rem_euclid(HEIGTH);

    Robot {
        position: (new_x, new_y),
        velocity: (robot.velocity.0, robot.velocity.1),
    }
}
