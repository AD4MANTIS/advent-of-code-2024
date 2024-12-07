use lib::ToVec;

lib::day!(07, part1, example => 3749, answer => 28_730_327_770_375);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

fn part1(input: &str) -> usize {
    let calibrations = input.lines().map(parse_line).to_vec();

    let max_operations_to_perform = calibrations
        .iter()
        .map(|x| x.values.len())
        .max()
        .unwrap_or_default();

    let mut all_posible_operations = vec![vec![Operator::Add], vec![Operator::Mul]];
    for _ in 0..max_operations_to_perform {
        for x in &mut all_posible_operations {
            x.push(Operator::Add);
        }

        all_posible_operations.append(
            &mut all_posible_operations
                .clone()
                .into_iter()
                .map(|mut x| {
                    *x.last_mut().unwrap() = Operator::Mul;
                    x
                })
                .to_vec(),
        );
    }

    calibrations
        .into_iter()
        .filter(|cal| {
            for operator_combination in &all_posible_operations {
                let result = cal.values.iter().skip(1).zip(operator_combination).fold(
                    cal.values[0],
                    |result, (value, operator)| match operator {
                        Operator::Add => result + value,
                        Operator::Mul => result * value,
                    },
                );

                if result == cal.result {
                    return true;
                }
            }

            false
        })
        .map(|x| x.result)
        .sum()
}

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
