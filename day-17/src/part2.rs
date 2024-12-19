lib::day!(17, part2, example raw(r"Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0") => 117_440);

#[allow(non_camel_case_types)]
enum Instruction {
    adv,
    bxl,
    bst,
    jnz,
    bxc,
    out,
    bdv,
    cdv,
}

impl Instruction {
    const fn from_usize(instruction: usize) -> Self {
        match instruction {
            0 => Self::adv,
            1 => Self::bxl,
            2 => Self::bst,
            3 => Self::jnz,
            4 => Self::bxc,
            5 => Self::out,
            6 => Self::bdv,
            7 => Self::cdv,
            _ => panic!("Unknown Instruction"),
        }
    }
}

struct Registers {
    a: usize,
    b: usize,
    c: usize,
}

fn part2(input: &str) -> usize {
    let instructions = parse_instructions(input);

    for reg_a in 0.. {
        if reg_a % 100_000_000 == 0 {
            dbg!(reg_a);
        }

        if output_matches_instructions(reg_a, &instructions) {
            return reg_a;
        }
    }

    0
}

fn output_matches_instructions(reg_a: usize, instructions: &[usize]) -> bool {
    let mut reg = Registers {
        a: reg_a,
        b: 0,
        c: 0,
    };

    let mut instruction_pointer = 0;
    let mut output_index = 0;

    while let Some(inst) = instructions.get(instruction_pointer..=instruction_pointer + 1) {
        let op = inst[1];
        match Instruction::from_usize(inst[0]) {
            Instruction::adv => {
                reg.a /= 2_usize.pow(get_combo_operant_value(op, &reg).try_into().unwrap());
            }
            Instruction::bxl => {
                reg.b ^= op;
            }
            Instruction::bst => {
                reg.b = get_combo_operant_value(op, &reg) % 8;
            }
            Instruction::jnz => {
                if reg.a != 0 {
                    instruction_pointer = op;
                    continue;
                }
            }
            Instruction::bxc => {
                reg.b ^= reg.c;
            }
            Instruction::out => {
                let output = get_combo_operant_value(op, &reg) % 8;

                if output != instructions[output_index] {
                    return false;
                }

                output_index += 1;
                if output_index == instructions.len() {
                    return true;
                }
            }
            Instruction::bdv => {
                reg.b = reg.a / 2_usize.pow(get_combo_operant_value(op, &reg).try_into().unwrap());
            }
            Instruction::cdv => {
                reg.c = reg.a / 2_usize.pow(get_combo_operant_value(op, &reg).try_into().unwrap());
            }
        }

        instruction_pointer += 2;
    }

    false
}

#[inline]
const fn get_combo_operant_value(op: usize, reg: &Registers) -> usize {
    match op {
        0..=3 => op,
        4 => reg.a,
        5 => reg.b,
        6 => reg.c,
        7.. => panic!("reserved!"),
    }
}

fn parse_instructions(input: &str) -> Vec<usize> {
    input
        .lines()
        .last()
        .unwrap()
        .split_once(' ')
        .unwrap()
        .1
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap()
}
