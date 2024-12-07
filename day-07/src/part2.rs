use lib::ToVec;

lib::day!(07, part2, example => 11387, answer => 424_977_609_625_985);

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
    Concat,
}

#[derive(Debug)]
struct Calibration {
    expected_result: usize,
    values: Vec<usize>,
}

#[derive(Debug)]
struct OperationChain {
    for_length: usize,
    operations: Vec<Vec<Operator>>,
}

fn part2(input: &str) -> usize {
    let calibrations = input.lines().map(parse_line).to_vec();

    let max_operations_to_perform = calibrations
        .iter()
        .map(|x| x.values.len())
        .max()
        .unwrap_or_default();

    // `- 2` because the two longest value lists never come this far and this saves 70% of the time
    let all_posible_operations = (1..=max_operations_to_perform - 2)
        .rev()
        .map(create_operation_chain)
        .to_vec();

    calibrations
        .into_iter()
        .filter(|cal| is_any_operation_possible(&all_posible_operations, cal))
        .map(|x| x.expected_result)
        .sum()
}

fn parse_line(line: &str) -> Calibration {
    let x = line
        .split_once(": ")
        .expect("Input should have a valid Format");

    Calibration {
        expected_result: x.0.parse().unwrap(),
        values: x
            .1
            .split_whitespace()
            .map(|val| val.parse().unwrap())
            .collect(),
    }
}

fn create_operation_chain(max_operations_to_perform: usize) -> OperationChain {
    let mut all_posible_operations = vec![
        vec![Operator::Add],
        vec![Operator::Mul],
        vec![Operator::Concat],
    ];
    for _ in 0..max_operations_to_perform {
        for x in &mut all_posible_operations {
            x.push(Operator::Add);
        }

        let base = all_posible_operations.clone();

        append_other_operator_option(&mut all_posible_operations, base.clone(), Operator::Mul);
        append_other_operator_option(&mut all_posible_operations, base, Operator::Concat);
    }

    OperationChain {
        for_length: max_operations_to_perform,
        operations: all_posible_operations,
    }
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

fn is_any_operation_possible(all_posible_operations: &[OperationChain], cal: &Calibration) -> bool {
    for operator_combination in &all_posible_operations
        .iter()
        .find(|opt_with_len| opt_with_len.for_length <= cal.values.len())
        .unwrap()
        .operations
    {
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

                // Check if we are already above the expected result, because all
                // operations only further increase the value
                if result > cal.expected_result {
                    return None;
                }

                Some(result)
            });

        if result == Some(cal.expected_result) {
            return true;
        }
    }

    false
}
