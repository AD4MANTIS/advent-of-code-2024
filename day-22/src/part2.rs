use std::{collections::HashMap, usize};

use itertools::Itertools;
use lib::{combinations::CombinationsIter, ToVec};

use crate::part1::calc_next_secret;

lib::day!(22, part2, example raw(r"1
2
3
2024") => 23, answer => 1986);

fn part2(input: &str) -> usize {
    let sellers = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .map(|mut secret| {
            let mut price_changes = Vec::new();
            for _ in 0..2000 {
                let next_secret = calc_next_secret(secret);

                price_changes.push((
                    next_secret % 10,
                    next_secret as isize % 10 - secret as isize % 10,
                ));

                secret = next_secret;
            }

            price_changes
                .into_iter()
                .tuple_windows()
                .map(|(a, b, c, d)| ([a.1, b.1, c.1, d.1], d.0))
                .unique_by(|changes| changes.0)
                .collect::<HashMap<_, _>>()
        })
        .to_vec();

    CombinationsIter::combinations(-9..=9_isize, 4)
        .filter(|changes| {
            for window_len in 2..=4 {
                for change in changes.windows(window_len) {
                    if change.iter().sum::<isize>().abs() > 18 {
                        return false;
                    }
                }
            }

            true
        })
        .map(|changes| {
            sellers
                .iter()
                .map(|seller_prices| {
                    seller_prices
                        .get(&changes[..4])
                        .copied()
                        .unwrap_or_default()
                })
                .sum()
        })
        .max()
        .unwrap_or_default()
}
