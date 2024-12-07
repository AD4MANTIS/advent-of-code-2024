use lib::ToVec;

lib::day!(07, part2, example => 11387, answer => 424_977_609_625_985);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

fn part2(input: &str) -> usize {
    let calibrations = input.lines().map(parse_line).to_vec();

    let max_operations_to_perform = calibrations
        .iter()
        .map(|x| x.values.len())
        .max()
        .unwrap_or_default();

    let mut all_posible_operations = vec![
        vec![Operator::Add],
        vec![Operator::Mul],
        vec![Operator::Concat],
    ];
    for _ in 0..max_operations_to_perform - 2 {
        for x in &mut all_posible_operations {
            x.push(Operator::Add);
        }

        let base = all_posible_operations.clone();

        append_other_operator_option(&mut all_posible_operations, base.clone(), Operator::Mul);
        append_other_operator_option(&mut all_posible_operations, base, Operator::Concat);
    }

    calibrations
        .into_iter()
        .filter(|cal| {
            for operator_combination in &all_posible_operations {
                let result = cal
                    .values
                    .iter()
                    .skip(1)
                    .zip(operator_combination)
                    .try_fold(cal.values[0], |mut result, (value, operator)| {
                        match operator {
                            Operator::Add => result += value,
                            Operator::Mul => result *= value,
                            Operator::Concat => {
                                result = format!("{result}{value}").parse().unwrap();
                            }
                        };

                        if result > cal.result {
                            return None;
                        }

                        Some(result)
                    });

                if result == Some(cal.result) {
                    return true;
                }
            }

            false
        })
        .map(|x| x.result)
        .sum()
}

fn append_other_operator_option(
    all_posible_operations: &mut Vec<Vec<Operator>>,
    base: Vec<Vec<Operator>>,
    other_op: Operator,
) {
    all_posible_operations.append(
        &mut base
            .into_iter()
            .map(|mut x| {
                *x.last_mut().unwrap() = other_op;
                x
            })
            .to_vec(),
    );
}

#[derive(Debug)]
struct Calibration {
    result: usize,
    values: Vec<usize>,
}

fn parse_line(line: &str) -> Calibration {
    let x = line
        .split_once(": ")
        .expect("Input should have a valid Format");

    Calibration {
        result: x.0.parse().unwrap(),
        values: x
            .1
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect(),
    }
}
