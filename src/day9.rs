fn main() {
    let content = std::fs::read_to_string("src/inputs/input-day9.txt").unwrap();

    let mut disk_map: Vec<Vec<String>> = content
        .chars()
        .enumerate()
        .filter_map(|(index, c)| {
            let number_of_blocks = c.to_string().parse::<usize>().unwrap();
            if number_of_blocks == 0 {
                return None;
            }
            let character = if index % 2 == 0 {
                (index as u64 / 2).to_string()
            } else {
                ".".to_string()
            };
            Some(generate_repeating_character(character, number_of_blocks))
        })
        .flatten()
        .collect();

    let first_checksum = part1(&mut disk_map.iter().flatten().cloned().collect());
    let second_checksum = part2(&mut disk_map);

    println!("The checksum is {}", first_checksum);
    println!("The second checksum is {}", second_checksum);
}

fn part1(disk_map: &mut Vec<String>) -> u64 {
    let disk_map_length = disk_map.len();
    for i in 0..disk_map_length {
        if disk_map[i] == "." {
            for j in (i + 1..disk_map_length).rev() {
                if disk_map[j] != "." {
                    disk_map.swap(i, j);
                    break;
                }
            }
        }
    }

    disk_map
        .iter()
        .enumerate()
        .filter(|(_, v)| **v != ".")
        .map(|(i, value)| i as u64 * (value.parse::<u64>().unwrap()))
        .sum()
}

fn part2(disk_map: &mut Vec<Vec<String>>) -> u64 {
    let disk_map_length = disk_map.len();
    for i in (0..disk_map_length).rev() {
        if disk_map[i].contains(&".".to_string()) {
            continue;
        }
        let space_needed = disk_map[i].len();
        for j in 0..i {
            if disk_map[j].iter().filter(|c| **c == ".").count() >= space_needed {
                let block_to_move: Vec<String> = disk_map[i].iter().cloned().collect();
                let first_dot_index = disk_map[j].iter().position(|e| *e == ".").unwrap();
                for (index, element) in block_to_move.iter().enumerate() {
                    disk_map[j][first_dot_index + index] = element.clone();
                    disk_map[i][index] = ".".to_string();
                }
                break;
            }
        }
    }

    disk_map
        .iter()
        .flatten()
        .enumerate()
        .filter(|(_, v)| **v != ".")
        .map(|(i, value)| i as u64 * (value.parse::<u64>().unwrap()))
        .sum()
}

fn generate_repeating_character(character_to_repeat: String, times: usize) -> Vec<Vec<String>> {
    vec![vec![character_to_repeat; times]]
}
