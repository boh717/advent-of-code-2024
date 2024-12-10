use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Coordinate {
    line: i32,
    column: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Place {
    coordinate: Coordinate,
    value: u32,
}

fn main() {
    let file = File::open("src/inputs/input-day10.txt").unwrap();
    let reader = BufReader::new(file);
    let mut places: HashMap<Coordinate, u32> = HashMap::new();

    reader
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .for_each(|(line, row)| {
            row.chars()
                .map(|n| n.to_string().parse::<u32>().unwrap())
                .enumerate()
                .for_each(|(column, number)| {
                    let c = Place {
                        coordinate: Coordinate {
                            line: line as i32,
                            column: column as i32,
                        },
                        value: number,
                    };
                    places.insert(c.coordinate, number);
                });
        });

    let trailheads_score = calculate_hiking_score(&places, false);
    let trailheads_ratings = calculate_hiking_score(&places, true);

    println!("Trailheads score is {}", trailheads_score);
    println!("Trailheads score with rating is {}", trailheads_ratings);
}

fn calculate_hiking_score(positions: &HashMap<Coordinate, u32>, support_ratings: bool) -> u32 {
    positions
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, v)| {
            let mut visited_places: HashSet<Place> = HashSet::new();
            let start_place = Place {
                coordinate: *k,
                value: *v,
            };
            get_hiking_trails_score(positions, &mut visited_places, start_place, support_ratings)
        })
        .sum()
}

fn get_hiking_trails_score(
    positions: &HashMap<Coordinate, u32>,
    visited_peaks: &mut HashSet<Place>,
    place: Place,
    support_ratings: bool,
) -> u32 {
    if place.value == 9 {
        if !visited_peaks.contains(&place) {
            if !support_ratings {
                visited_peaks.insert(place);
            }
            return 1;
        } else {
            return 0;
        };
    }

    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    directions
        .iter()
        .filter_map(|&(dx, dy)| {
            let new_coord = Coordinate {
                line: place.coordinate.line + dx,
                column: place.coordinate.column + dy,
            };

            positions.get(&new_coord).and_then(|&v| {
                (v == place.value + 1).then(|| {
                    get_hiking_trails_score(
                        positions,
                        visited_peaks,
                        Place {
                            coordinate: new_coord,
                            value: v,
                        },
                        support_ratings,
                    )
                })
            })
        })
        .sum()
}
