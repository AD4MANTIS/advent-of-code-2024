lib::day!(00, part1, example => 161, answer => 159892596);

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let len = line.len();
            let mut index = 0;
            let mut sum = 0;

            while index < len {
                if line.get(index..index + 4) == Some("mul(") {
                    if let Some((num1, num2)) = parse_nums(line, index + 4) {
                        sum += num1 * num2;
                    }

                    index += 4;
                } else {
                    index += 1;
                }
            }

            sum
        })
        .sum()
}

fn parse_nums(input: &str, start: usize) -> Option<(usize, usize)> {
    input
        .get(start..)
        .and_then(|x| x.split_once(","))
        .and_then(|x| x.1.split_once(")").map(|num2| (x.0, num2.0)))
        .map(|x| (x.0.parse(), x.1.parse()))
        .and_then(|x| match x {
            (Ok(num1), Ok(num2)) => Some((num1, num2)),
            _ => None,
        })
}
