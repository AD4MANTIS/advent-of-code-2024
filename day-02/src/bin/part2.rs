use lib::ToVec;

lib::day!(02, part2, example => 4, test raw("9 1 2 3") => 1, test2 raw("1 99 2 3 4") => 1, answer => 692);

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.

fn part2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let line = line
                .split_whitespace()
                .flat_map(|level| level.parse::<usize>().ok())
                .enumerate()
                .to_vec();

            test_line::<0>(&line) || test_line::<1>(&line) || test_line::<999999>(&line)
        })
        .count()
}

fn test_line<const SKIP: usize>(line: &[(usize, usize)]) -> bool {
    let mut line = line
        .iter()
        .filter(|level| level.0 != SKIP)
        .map(|level| level.1)
        .peekable();

    let mut current_level = line.next().expect("Line should have at least one Item");
    let level = *line.peek().expect("Line should have at least two Items");
    let asc = current_level < level;
    let mut can_remove_level = SKIP == 999999;

    for next_level in line {
        if next_level == current_level
            || next_level.abs_diff(current_level) > 3
            || (asc && next_level < current_level)
            || (!asc && next_level > current_level)
        {
            if can_remove_level {
                can_remove_level = false;
                continue;
            } else {
                return false;
            }
        }

        current_level = next_level;
    }

    true
}
