use std::num::ParseIntError;

use itertools::Itertools;
use lib::maps::{offset::Offset, prelude::Pos};
use thiserror::Error;

lib::day!(13, part1, example => 480, answer => 29388);

fn part1(input: &str) -> usize {
    let mut machines: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .tuples::<(_, _, _)>()
        .map(Machine::try_from)
        .try_collect()
        .unwrap();

    let all_combinations = get_all_combinations(100)
        .sorted_by_key(|comb| comb.iter().map(|button| button.get_cost()).sum::<usize>())
        .collect_vec();

    machines
        .iter_mut()
        .filter_map(|machine| {
            all_combinations.iter().find_map(|combination| {
                let mut current_pos = Pos::new(0, 0);
                let mut cost = 0;

                for button in combination {
                    current_pos = current_pos
                        .try_add(machine.get_button(*button))
                        .expect("Should only be positive an not overflow");

                    cost += button.get_cost();

                    if current_pos == machine.price {
                        return Some(cost);
                    }
                }

                None
            })
        })
        .sum()
}

fn get_all_combinations(max_taps: usize) -> impl Iterator<Item = Vec<Button>> {
    (0..=max_taps).flat_map(move |a_taps| {
        (0..=max_taps)
            .map(move |b_taps| [[Button::A].repeat(a_taps), [Button::B].repeat(b_taps)].concat())
    })
}

#[derive(Clone, Copy)]
enum Button {
    A,
    B,
}

impl Button {
    const fn get_cost(self) -> usize {
        match self {
            Self::A => 3,
            Self::B => 1,
        }
    }
}

struct Machine {
    button_a: Offset,
    button_b: Offset,
    price: Pos,
}

impl Machine {
    const fn get_button(&self, btn: Button) -> &Offset {
        match btn {
            Button::A => &self.button_a,
            Button::B => &self.button_b,
        }
    }
}

#[derive(Error, Debug)]
enum MachineError {
    #[error("{0}")]
    Parse(#[from] ParseIntError),
    #[error("Encountered more than 2 Elements")]
    MoreThanTwoElements,
}

impl TryFrom<(&str, &str, &str)> for Machine {
    type Error = MachineError;

    fn try_from(value: (&str, &str, &str)) -> Result<Self, Self::Error> {
        let binding = value
            .2
            .chars()
            .filter(|char| char.is_numeric() || *char == ',')
            .collect::<String>();
        let price = binding
            .split_once(',')
            .ok_or(MachineError::MoreThanTwoElements)?;

        Ok(Self {
            button_a: parse_button(value.0)?,
            button_b: parse_button(value.1)?,
            price: Pos {
                x: price.0.parse()?,
                y: price.1.parse()?,
            },
        })
    }
}

fn parse_button(line: &str) -> Result<Offset, MachineError> {
    let binding = line
        .split_once(':')
        .ok_or(MachineError::MoreThanTwoElements)?
        .1
        .chars()
        .filter(|char| char.is_numeric() || *char == ',')
        .collect::<String>();
    let split = binding
        .split_once(',')
        .ok_or(MachineError::MoreThanTwoElements)?;

    Ok(Offset {
        x: split.0.parse()?,
        y: split.1.parse()?,
    })
}
