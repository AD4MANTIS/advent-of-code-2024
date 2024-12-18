use lib::ToVec;

lib::day!(17, part1, example => "4,6,3,5,6,3,5,2,1,0", answer => "7,3,5,7,5,7,4,3,0");

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
    fn from_usize(instruction: usize) -> Self {
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

fn part1(input: &str) -> String {
    let mut reg = parse_registers(input);

    let instructions = parse_instructions(input);

    let mut instruction_pointer = 0;
    let mut output = vec![];

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
                output.push(get_combo_operant_value(op, &reg) % 8);
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

    output
        .into_iter()
        .map(|out| out.to_string())
        .to_vec()
        .join(",")
}

fn get_combo_operant_value(op: usize, reg: &Registers) -> usize {
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

fn parse_registers(input: &str) -> Registers {
    let [reg_a, reg_b, reg_c] = input
        .lines()
        .take(3)
        .map(|line| line.split_once(':').unwrap().1.trim())
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .try_into()
        .unwrap();

    Registers {
        a: reg_a,
        b: reg_b,
        c: reg_c,
    }
}
