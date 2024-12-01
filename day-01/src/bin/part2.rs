use std::collections::HashMap;

use lib::ToVec;

lib::day!(01, part2, example => 31, answer => 27732508);

fn part2(input: &str) -> usize {
    let location_ids = input
        .lines()
        .flat_map(|line| {
            line.split_whitespace().map(|location_id| {
                location_id
                    .parse::<usize>()
                    .expect("Should be a valid Number")
            })
        })
        .to_vec();

    let list_1 = location_ids.iter().cloned().step_by(2);
    let list_2 = location_ids.iter().skip(1).step_by(2);

    let mut counter = HashMap::<usize, usize>::new();

    for num in list_2 {
        *(counter.entry(*num).or_insert(0)) += 1;
    }

    list_1
        .into_iter()
        .map(|num| num * counter.get(&num).unwrap_or(&0))
        .sum()
}
