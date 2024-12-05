use std::collections::HashSet;

use itertools::Itertools;
use lib::ToVec;

lib::day!(05, part1, example => 143, answer => 5747);

fn part1(input: &str) -> usize {
    let (rules, protocols) = input.split_once("\n\n").unwrap();

    let ordering_rules = rules
        .lines()
        .map(|rule| {
            let (x, y) = rule.split_once('|').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .into_grouping_map()
        .collect::<HashSet<usize>>();

    let protocols = protocols.lines().map(|protocol| {
        protocol
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .to_vec()
    });

    protocols
        .filter(|protocol| {
            for index in 1..protocol.len() {
                let current_num = protocol[index];

                let Some(rules) = ordering_rules.get(&current_num) else {
                    continue;
                };

                for previous_num in &protocol[0..index] {
                    if rules.contains(previous_num) {
                        return false;
                    }
                }
            }

            true
        })
        .map(|valid_protocol| valid_protocol[valid_protocol.len() / 2])
        .sum()
}
