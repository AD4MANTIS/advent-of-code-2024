use std::collections::HashMap;

use lib::ToVec;

lib::day!(19, part2, example => 16, answer => 624_802_218_898_092);

fn part2(input: &str) -> usize {
    let mut towels = input.lines().next().unwrap().split(", ").to_vec();
    towels.sort_by_key(|t| t.len());
    towels.reverse();

    let designs = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .to_vec();

    let mut cached_designs = HashMap::new();

    designs
        .into_iter()
        .map(|design| count_design_possible_arrangements(&towels, design, &mut cached_designs))
        .sum()
}

fn count_design_possible_arrangements<'a>(
    towels: &[&str],
    design: &'a str,
    cached_designs: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(count) = cached_designs.get(design) {
        return *count;
    }

    let count = towels
        .iter()
        .map(|towel| {
            if !design.starts_with(towel) {
                return 0;
            }

            count_design_possible_arrangements(towels, &design[towel.len()..], cached_designs)
        })
        .sum();

    cached_designs.insert(design, count);

    count
}
