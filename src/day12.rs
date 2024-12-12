use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
struct Coordinate {
    line: i32,
    column: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct GardenPlot {
    coordinate: Coordinate,
    value: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Region {
    plots: Vec<GardenPlot>,
}

fn main() {
    let file = File::open("src/inputs/input-day12.txt").unwrap();
    let reader = BufReader::new(file);
    let mut garden_map: HashMap<Coordinate, String> = HashMap::new();

    reader
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .for_each(|(line_index, row)| {
            row.chars()
                .enumerate()
                .for_each(|(column_index, plot_type)| {
                    garden_map.insert(
                        Coordinate {
                            line: line_index as i32,
                            column: column_index as i32,
                        },
                        plot_type.to_string(),
                    );
                })
        });

    let total_price: u32 = calculate_fencing_price(&garden_map);

    println!("The total price of fencing is {}", total_price);
}

fn calculate_fencing_price(garden_map: &HashMap<Coordinate, String>) -> u32 {
    let mut visited_plots: HashSet<Coordinate> = HashSet::new();
    let mut total = 0;

    for (coord, value) in garden_map {
        if !visited_plots.contains(coord) {
            let plot = GardenPlot {
                coordinate: *coord,
                value: value.to_string(),
            };
            let mut region = Region { plots: vec![] };
            region = get_region(garden_map, plot, &mut region, &mut visited_plots);

            let area = region.plots.len() as u32;
            let perimeter = calculate_perimeter(garden_map, region.plots);
            total += area * perimeter;
        }
    }

    total
}

fn get_region(
    garden_map: &HashMap<Coordinate, String>,
    plot: GardenPlot,
    region: &mut Region,
    visited_plots: &mut HashSet<Coordinate>,
) -> Region {
    if visited_plots.contains(&plot.coordinate) {
        return region.clone();
    }

    visited_plots.insert(plot.coordinate);
    region.plots.push(plot);
    let neighbours = get_neighbours(garden_map, &region.plots.last().unwrap());

    neighbours.iter().for_each(|n| {
        get_region(garden_map, n.clone(), region, visited_plots);
    });

    region.clone()
}

fn get_neighbours(garden_map: &HashMap<Coordinate, String>, plot: &GardenPlot) -> Vec<GardenPlot> {
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];

    directions
        .iter()
        .filter_map(|(line_offset, column_offset)| {
            let coord = Coordinate {
                line: plot.coordinate.line + line_offset,
                column: plot.coordinate.column + column_offset,
            };
            garden_map.get(&coord).and_then(|p| {
                (*p == *plot.value).then(|| GardenPlot {
                    coordinate: coord,
                    value: p.clone(),
                })
            })
        })
        .collect()
}

fn calculate_perimeter(garden_map: &HashMap<Coordinate, String>, plots: Vec<GardenPlot>) -> u32 {
    let directions = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut perimeter = 0;

    plots.iter().for_each(|plot| {
        for (line_offset, column_offset) in directions {
            let coord = Coordinate {
                line: plot.coordinate.line + line_offset,
                column: plot.coordinate.column + column_offset,
            };
            match garden_map.get(&coord) {
                Some(p) if *plot.value == *p => continue,
                _ => perimeter += 1,
            };
        }
    });
    perimeter
}
