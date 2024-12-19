use std::collections::HashSet;

use lib::ToVec;

lib::day!(19, part1, example => 6, answer => 300);

fn part1(input: &str) -> usize {
    let towels = input.lines().next().unwrap().split(", ").to_vec();

    let designs = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .to_vec();

    let mut possible_designs = towels.clone().into_iter().collect::<HashSet<&str>>();
    let mut impossible_designs = HashSet::new();

    designs
        .into_iter()
        .filter(|design| {
            can_design_be_arranged(
                &towels,
                design,
                &mut possible_designs,
                &mut impossible_designs,
            )
        })
        .count()
}

fn can_design_be_arranged<'a>(
    towels: &[&str],
    design: &'a str,
    possible_designs: &mut HashSet<&'a str>,
    impossible_designs: &mut HashSet<&'a str>,
) -> bool {
    if design.is_empty() || possible_designs.contains(design) {
        return true;
    }

    if impossible_designs.contains(design) {
        return false;
    }

    for towel in towels {
        if !design.starts_with(towel) {
            continue;
        }

        if can_design_be_arranged(
            towels,
            &design[towel.len()..],
            possible_designs,
            impossible_designs,
        ) {
            possible_designs.insert(design);

            return true;
        }
    }

    impossible_designs.insert(design);
    false
}
