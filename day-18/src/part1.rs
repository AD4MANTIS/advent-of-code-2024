use std::collections::HashMap;

use lib::maps::prelude::{Direction, Map, Pos};

lib::day_main!(18, part1_answer);
lib::day_test!(18, part1_example, example => 22);
lib::day_test!(18, part1_answer, answer => 322);

#[allow(dead_code)]
fn part1_example(input: &str) -> usize {
    part1(input, 6, 12)
}

fn part1_answer(input: &str) -> usize {
    part1(input, 70, 1024)
}

fn part1(input: &str, map_size: usize, bytes: usize) -> usize {
    let mut map = Map::new(map_size + 1, map_size + 1, '.');

    for pos in parse_byte_positions(input).take(bytes) {
        map[&pos] = '#';
    }

    let mut pos_distance_cache = HashMap::<Pos, usize>::with_capacity(map_size * map_size);

    calc_distances(
        &map,
        &Pos::new(map_size, map_size),
        0,
        &mut pos_distance_cache,
    );

    pos_distance_cache[&Pos::new(0, 0)]
}

fn parse_byte_positions(input: &str) -> impl Iterator<Item = Pos> + use<'_> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| Pos::new(x.parse().unwrap(), y.parse().unwrap()))
}

fn calc_distances(
    map: &Map,
    current_pos: &Pos,
    current_distance: usize,
    cache: &mut HashMap<Pos, usize>,
) {
    if cache
        .get(current_pos)
        .map_or(false, |distance| current_distance >= *distance)
    {
        return;
    }

    cache.insert(current_pos.clone(), current_distance);

    if *current_pos == Pos::new(0, 0) {
        return;
    }

    for direction in Direction::all_directions() {
        let Some(next_pos) = current_pos.try_add(&direction.to_offset()) else {
            continue;
        };

        if map.get(&next_pos) == Some(&'.') {
            calc_distances(map, &next_pos, current_distance + 1, cache);
        }
    }
}
