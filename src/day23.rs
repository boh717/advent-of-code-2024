use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    let computers: Vec<String> = std::fs::read_to_string("src/inputs/input-day23.txt")
        .unwrap()
        .lines()
        .map(|l| l.to_string())
        .collect();

    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    let mut triangles: HashSet<(String, String, String)> = HashSet::new();

    for line in computers {
        let mut parts = line.split('-');
        let comp1 = parts.next().unwrap().to_string();
        let comp2 = parts.next().unwrap().to_string();

        connections
            .entry(comp1.clone())
            .or_default()
            .insert(comp2.clone());
        connections.entry(comp2).or_default().insert(comp1);
    }

    let sets_with_t_computers = part1(&mut connections, &mut triangles);

    println!(
        "Sets with at least one 't' computer are {}",
        sets_with_t_computers
    );

    let password = part2(&connections, &triangles);
    println!("LAN party password is {}", password);
}

fn part1(
    connections: &mut HashMap<String, HashSet<String>>,
    triangles: &mut HashSet<(String, String, String)>,
) -> u64 {
    for (computer_a, neighbors_a) in connections.iter() {
        for computer_b in neighbors_a.iter() {
            let neighbors_b = connections.get(computer_b).unwrap();
            for computer_c in neighbors_b.iter() {
                let neighbors_c = connections.get(computer_c).unwrap();
                if neighbors_c.contains(computer_a) {
                    let mut s = Vec::new();
                    s.push(computer_a);
                    s.push(computer_b);
                    s.push(computer_c);
                    s.sort();
                    let key = (s[0].clone(), s[1].clone(), s[2].clone());
                    triangles.insert(key);
                }
            }
        }
    }
    triangles
        .iter()
        .filter(|(c1, c2, c3)| c1.starts_with("t") || c2.starts_with("t") || c3.starts_with("t"))
        .count() as u64
}

fn part2(
    connections: &HashMap<String, HashSet<String>>,
    triangles: &HashSet<(String, String, String)>,
) -> String {
    let mut current_cliques: HashSet<BTreeSet<String>> = triangles
        .iter()
        .map(|(c1, c2, c3)| BTreeSet::from([c1.clone(), c2.clone(), c3.clone()]))
        .collect();

    let mut clique_size = 3;
    while current_cliques.len() > 1 {
        let new_cliques: HashSet<BTreeSet<String>> = current_cliques
            .iter()
            .flat_map(|clique| {
                let last_node = clique.iter().last().unwrap();
                connections
                    .keys()
                    .filter(move |candidate| candidate > &last_node)
                    .filter(|candidate| {
                        clique
                            .iter()
                            .all(|member| connections[*candidate].contains(member))
                    })
                    .map(|candidate| {
                        let mut new_clique = clique.clone();
                        new_clique.insert(candidate.clone());
                        new_clique
                    })
            })
            .collect();

        println!(
            "Found {} cliques of size {}",
            new_cliques.len(),
            clique_size
        );

        current_cliques = new_cliques;
        clique_size += 1;
    }

    current_cliques
        .iter()
        .next()
        .map(|clique| {
            clique
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap()
}
