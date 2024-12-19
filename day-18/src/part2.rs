use std::collections::HashMap;

use lib::maps::prelude::{Direction, Map, Pos};

lib::day_main!(18, part2_answer);
lib::day_test!(18, part2_example, example => Pos::new(6, 1));
lib::day_test!(18, part2_answer, answer => Pos::new(60, 21));

#[allow(dead_code)]
fn part2_example(input: &str) -> Pos {
    part2(input, 6)
}

fn part2_answer(input: &str) -> Pos {
    part2(input, 70)
}

fn part2(input: &str, map_size: usize) -> Pos {
    let mut map = Map::new(map_size + 1, map_size + 1, '.');

    let mut pos_distance_cache = HashMap::<Pos, usize>::with_capacity(map_size * map_size);
    for pos in parse_byte_positions(input) {
        map[&pos] = '#';

        calc_distances(
            &map,
            &Pos::new(map_size, map_size),
            0,
            &mut pos_distance_cache,
        );

        if !pos_distance_cache.contains_key(&Pos::new(0, 0)) {
            return pos;
        }

        pos_distance_cache.clear();
    }

    panic!("");
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
    if cache.contains_key(current_pos) {
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
