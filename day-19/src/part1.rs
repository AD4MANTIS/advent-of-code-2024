use lib::ToVec;

lib::day!(19, part1, example => 6, answer => 0);

fn part1(input: &str) -> usize {
    let towels = input.lines().next().unwrap().split(", ").to_vec();

    let designs = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .to_vec();

    designs
        .into_iter()
        .filter(|design| can_design_be_arranged(&towels, design))
        .count()
}

fn can_design_be_arranged(towels: &[&str], design: &str) -> bool {
    if design.is_empty() {
        return true;
    }

    for towel in towels {
        if !design.starts_with(*towel) {
            continue;
        }

        if can_design_be_arranged(towels, &design[towel.len()..]) {
            return true;
        }
    }

    false
}
