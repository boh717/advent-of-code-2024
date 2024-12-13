use regex::RegexBuilder;

#[derive(Clone, Debug)]
struct ButtonA {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug)]
struct ButtonB {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug)]
struct Prize {
    x: f64,
    y: f64,
}

#[derive(Clone, Debug)]
struct Machine {
    button_a: ButtonA,
    button_b: ButtonB,
    prize: Prize,
}

fn main() {
    let content = std::fs::read_to_string("src/inputs/input-day13.txt").unwrap();
    let raw_machines: Vec<&str> = content.split("\n\n").collect();
    let re = RegexBuilder::new(r"(?s)Button A: X\+(?<Xa>\d+), Y\+(?<Ya>\d+)\nButton B: X\+(?<Xb>\d+), Y\+(?<Yb>\d+)\nPrize: X=(?<Xf>\d+), Y=(?<Yf>\d+)")
        .multi_line(true)
        .build()
        .unwrap();

    let machines: Vec<Machine> = raw_machines
        .iter()
        .map(|m| {
            let caps = re.captures(m).unwrap();
            Machine {
                button_a: ButtonA {
                    x: caps["Xa"].parse().unwrap(),
                    y: caps["Ya"].parse().unwrap(),
                },
                button_b: ButtonB {
                    x: caps["Xb"].parse().unwrap(),
                    y: caps["Yb"].parse().unwrap(),
                },
                prize: Prize {
                    x: caps["Xf"].parse().unwrap(),
                    y: caps["Yf"].parse().unwrap(),
                },
            }
        })
        .collect();

    let tokens = part1(&machines);
    let bigger_tokens = part2(&machines);

    println!("Tokens I have to spend are {}", tokens);
    println!("Bigger tokens I have to spend are {}", bigger_tokens);
}

fn part1(machines: &Vec<Machine>) -> f64 {
    machines.iter().map(|m| calculate_tokens(m)).sum()
}

fn part2(machines: &Vec<Machine>) -> f64 {
    machines
        .iter()
        .map(|m| {
            let new_prize = Prize {
                x: m.prize.x + 10000000000000.0,
                y: m.prize.y + 10000000000000.0,
            };

            Machine {
                button_a: m.button_a.clone(),
                button_b: m.button_b.clone(),
                prize: new_prize,
            }
        })
        .map(|m| calculate_tokens(&m))
        .sum()
}

fn calculate_tokens(m: &Machine) -> f64 {
    let (xa, ya) = (m.button_a.x, m.button_a.y);
    let (xb, yb) = (m.button_b.x, m.button_b.y);
    let (xf, yf) = (m.prize.x, m.prize.y);
    let b_pushes: f64 = (xa * yf - ya * xf) / (yb * xa - xb * ya);

    if b_pushes.fract() == 0.0 {
        let a_pushes: f64 = (xf - xb * b_pushes) / xa;
        a_pushes * 3.0 + b_pushes
    } else {
        0.0
    }
}
