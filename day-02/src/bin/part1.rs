lib::day!(02, part1, example => 2, answer => 663);

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let mut line = line
                .split_whitespace()
                .flat_map(|level| level.parse::<usize>().ok())
                .peekable();

            let mut current_level = line.next().expect("Line should have at least one Item");
            let level = *line.peek().expect("Line should have at least two Items");
            let asc = current_level < level;

            for next_level in line {
                if next_level == current_level || next_level.abs_diff(current_level) > 3 {
                    return false;
                }

                if asc && next_level < current_level {
                    return false;
                }

                if !asc && next_level > current_level {
                    return false;
                }

                current_level = next_level;
            }

            true
        })
        .count()
}
