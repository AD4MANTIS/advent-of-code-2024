use crate::part1::calc;

lib::day!(21, part2, answer => 294209504640384);

fn part2(input: &str) -> usize {
    calc(input, 25)
}
