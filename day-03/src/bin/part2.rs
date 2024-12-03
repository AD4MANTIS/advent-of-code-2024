lib::day!(03, part2, example("./example-input-part2.txt") => 48, answer => 92626942);

fn part2(input: &str) -> usize {
    let len = input.len();
    let mut index = 0;
    let mut sum = 0;
    let mut mul_enabled = true;

    while index < len {
        let current_check = input.get(index..).unwrap();

        if mul_enabled && current_check.starts_with("don't()") {
            mul_enabled = false;
            index += 7;
            continue;
        } else if !mul_enabled && current_check.starts_with("do()") {
            mul_enabled = true;
            index += 4;
        } else if mul_enabled && current_check.starts_with("mul(") {
            if let Some((num1, num2)) = parse_nums(current_check.get(4..).unwrap()) {
                sum += num1 * num2;
            }

            index += 4;
        } else {
            index += 1;
        }
    }

    sum
}

fn parse_nums(check: &str) -> Option<(usize, usize)> {
    check
        .split_once(",")
        .and_then(|x| x.1.split_once(")").map(|num2| (x.0, num2.0)))
        .map(|x| (x.0.parse(), x.1.parse()))
        .and_then(|x| match x {
            (Ok(num1), Ok(num2)) => Some((num1, num2)),
            _ => None,
        })
}
