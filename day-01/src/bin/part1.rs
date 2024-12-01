#![allow(unused_variables)]

use lib::ToVec;

lib::day!(00, part1, example => 11, answer => 765748);

fn part1(input: &str) -> usize {
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

    let mut list_1 = location_ids.iter().cloned().step_by(2).to_vec();
    let mut list_2 = location_ids.into_iter().skip(1).step_by(2).to_vec();

    list_1.sort();
    list_2.sort();

    list_1
        .into_iter()
        .zip(list_2)
        .map(|ids| ids.0.abs_diff(ids.1))
        .sum()
}
