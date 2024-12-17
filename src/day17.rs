use regex::{Regex, RegexBuilder};

#[derive(Clone, Debug, Copy)]
struct RegisterA {
    value: u64,
}

#[derive(Clone, Debug, Copy)]
struct RegisterB {
    value: u64,
}

#[derive(Clone, Debug, Copy)]
struct RegisterC {
    value: u64,
}

#[derive(Clone, Debug, Copy)]
struct Registers {
    reg_a: RegisterA,
    reg_b: RegisterB,
    reg_c: RegisterC,
}

#[derive(Clone, Debug, Copy)]
struct Operand {
    value: u64,
}

#[derive(Clone, Debug, Copy)]
enum InstructionPointer {
    Incr(usize),
    Set(usize),
}

#[derive(Clone, Debug, Copy)]
struct Output(u64);

fn main() {
    let content = std::fs::read_to_string("src/inputs/input-day17.txt").unwrap();
    let re = RegexBuilder::new(r"(?s)Register A: (?<RegA>\d+)\nRegister B: (?<RegB>\d+)\nRegister C: (?<RegC>\d+)\n\nProgram: (?<Program>.*)")
    .multi_line(true)
    .build()
    .unwrap();

    let program_output = part1(&content, &re);
    println!("The program output is {:?}", program_output);

    let reg_a_value = part2(&content, &re);
    println!(
        "The lowest positive initial value for register A {:?}",
        reg_a_value
    );
}

fn part1(input: &String, re: &Regex) -> String {
    let caps = re.captures(&input).unwrap();
    let mut registers = Registers {
        reg_a: RegisterA {
            value: caps["RegA"].parse().unwrap(),
        },
        reg_b: RegisterB {
            value: caps["RegB"].parse().unwrap(),
        },
        reg_c: RegisterC {
            value: caps["RegC"].parse().unwrap(),
        },
    };

    let program: Vec<char> = caps["Program"]
        .to_string()
        .chars()
        .filter(|c| c.is_numeric())
        .collect();

    run_program(&program, &mut registers)
}

fn part2(input: &String, re: &Regex) -> u64 {
    let caps = re.captures(&input).unwrap();
    let program: Vec<char> = caps["Program"]
        .to_string()
        .chars()
        .filter(|c| c.is_numeric())
        .collect();
    let reversed_program: Vec<String> = program.iter().map(|c| c.to_string()).rev().collect();

    let program_length = program.len();

    let mut reg_a: u64 = 0;
    for i in 0..program_length {
        let expected_output: String = reversed_program
            .clone()
            .into_iter()
            .take(i + 1)
            .rev()
            .collect::<Vec<String>>()
            .join(",");
        let mut computed_output: String;

        loop {
            let mut registers = Registers {
                reg_a: RegisterA { value: reg_a },
                reg_b: RegisterB { value: 0 },
                reg_c: RegisterC { value: 0 },
            };
            computed_output = run_program(&program, &mut registers);

            if i == program_length - 1 && computed_output == expected_output {
                break;
            } else if computed_output == expected_output {
                reg_a = reg_a * 8;

                break;
            } else {
                reg_a += 1;
            }
        }
    }

    reg_a
}

fn run_program(program: &Vec<char>, registers: &mut Registers) -> String {
    let program_length = program.len();
    let mut output: Vec<u64> = Vec::new();
    let mut instruction_pointer = 0;
    while instruction_pointer < program_length {
        let op_code = program[instruction_pointer]
            .to_string()
            .parse::<u64>()
            .unwrap();
        let operand = Operand {
            value: program[instruction_pointer + 1]
                .to_string()
                .parse::<u64>()
                .unwrap(),
        };

        let new_ip_value = match op_code {
            0 => adv(operand, registers),
            1 => bxl(operand, registers),
            2 => bst(operand, registers),
            3 => jnz(operand, registers),
            4 => bxc(registers),
            5 => {
                let (o, ip) = out(operand, registers);
                output.push(o.0);
                ip
            }
            6 => bdv(operand, registers),
            7 => cdv(operand, registers),
            _ => panic!(),
        };

        match new_ip_value {
            InstructionPointer::Incr(v) => instruction_pointer += v,
            InstructionPointer::Set(v) => instruction_pointer = v,
        }
    }

    output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn adv(operand: Operand, regs: &mut Registers) -> InstructionPointer {
    let numerator = regs.reg_a.value;
    let denominator: u64 = 2_u64.pow(get_combo_operand_value(operand, *regs) as u32);
    let result = numerator / denominator;
    regs.reg_a.value = result;

    InstructionPointer::Incr(2)
}

fn bdv(operand: Operand, regs: &mut Registers) -> InstructionPointer {
    let numerator = regs.reg_a.value;
    let denominator: u64 = 2_u64.pow(get_combo_operand_value(operand, *regs) as u32);
    let result = numerator / denominator;
    regs.reg_b.value = result;

    InstructionPointer::Incr(2)
}

fn cdv(operand: Operand, regs: &mut Registers) -> InstructionPointer {
    let numerator = regs.reg_a.value;
    let denominator: u64 = 2_u64.pow(get_combo_operand_value(operand, *regs) as u32);
    let result = numerator / denominator;
    regs.reg_c.value = result;

    InstructionPointer::Incr(2)
}

fn bxl(operand: Operand, regs: &mut Registers) -> InstructionPointer {
    let result = regs.reg_b.value ^ operand.value;
    regs.reg_b.value = result;
    InstructionPointer::Incr(2)
}

fn bst(operand: Operand, regs: &mut Registers) -> InstructionPointer {
    let result = get_combo_operand_value(operand, *regs) % 8;
    regs.reg_b.value = result;
    InstructionPointer::Incr(2)
}

fn jnz(operand: Operand, regs: &mut Registers) -> InstructionPointer {
    match regs.reg_a.value {
        0 => InstructionPointer::Incr(2),
        _ => InstructionPointer::Set(operand.value as usize),
    }
}

fn bxc(regs: &mut Registers) -> InstructionPointer {
    let result = regs.reg_b.value ^ regs.reg_c.value;
    regs.reg_b.value = result;
    InstructionPointer::Incr(2)
}

fn out(operand: Operand, regs: &mut Registers) -> (Output, InstructionPointer) {
    (
        Output(get_combo_operand_value(operand, *regs) % 8),
        InstructionPointer::Incr(2),
    )
}

fn get_combo_operand_value(operand: Operand, regs: Registers) -> u64 {
    match operand.value {
        0..=3 => operand.value,
        4 => regs.reg_a.value,
        5 => regs.reg_b.value,
        6 => regs.reg_c.value,
        _ => panic!(),
    }
}
