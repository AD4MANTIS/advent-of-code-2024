use std::{num::ParseIntError, str::FromStr};

use lib::maps::offset::Offset;

lib::day_test!(14, part1_example, example => 12);
lib::day_test!(14, part1_answer, answer => 230_172_768);

#[allow(dead_code)]
fn part1_example(input: &str) -> usize {
    part1(input, 11, 7)
}

#[allow(dead_code)]
fn part1_answer(input: &str) -> usize {
    part1(input, 101, 103)
}

fn part1(input: &str, width: isize, height: isize) -> usize {
    let mut robots = input
        .lines()
        .map(Robot::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for _ in 0..100 {
        for robot in &mut robots {
            robot.pos = robot.pos.clone() + robot.velocity.clone();

            if robot.pos.x >= width {
                robot.pos.x %= width;
            } else if robot.pos.x < 0 {
                robot.pos.x += width;
            }

            if robot.pos.y >= height {
                robot.pos.y %= height;
            } else if robot.pos.y < 0 {
                robot.pos.y += height;
            }
        }
    }

    robots
        .iter()
        .filter(|r| (r.pos.x < width / 2) && (r.pos.y < height / 2))
        .count()
        * robots
            .iter()
            .filter(|r| (r.pos.x > width / 2) && (r.pos.y < height / 2))
            .count()
        * robots
            .iter()
            .filter(|r| (r.pos.x < width / 2) && (r.pos.y > height / 2))
            .count()
        * robots
            .iter()
            .filter(|r| (r.pos.x > width / 2) && (r.pos.y > height / 2))
            .count()
}

#[derive(Debug)]
struct Robot {
    pos: Offset,
    velocity: Offset,
}

impl FromStr for Robot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once(' ').unwrap();

        let pos = parse_two_numbers(pos)?;
        let vel = parse_two_numbers(vel)?;
        Ok(Self {
            pos: Offset::new(pos.0, pos.1),
            velocity: Offset::new(vel.0, vel.1),
        })
    }
}

fn parse_only_numbers<T: FromStr>(s: &str) -> Result<T, T::Err> {
    s.chars()
        .filter(|c| c.is_numeric() || *c == '-')
        .collect::<String>()
        .parse()
}

fn parse_two_numbers<T: FromStr>(s: &str) -> Result<(T, T), T::Err> {
    let s = s.split_once(',').unwrap();
    Ok((parse_only_numbers(s.0)?, parse_only_numbers(s.1)?))
}
