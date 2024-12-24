use std::collections::HashMap;

use lib::maps::prelude::{Direction, Map, Pos};

lib::day!(20, part1, example => 0, answer => 1286);

fn part1(input: &str) -> usize {
    let map = Map::from(input);

    let start = map
        .all_pos_iter()
        .find(|pos| map[pos] == 'S')
        .expect("Should have Start");
    let route = get_normal_route(&map, start);

    route
        .iter()
        .flat_map(|(pos, distance)| {
            get_jumps_from_pos(&map, pos)
                .filter_map(|cheated_pos| route.get(&cheated_pos))
                .filter(move |cheated_distance| *cheated_distance > distance)
                .map(move |cheated_distance| cheated_distance - distance - 2)
        })
        .filter(|picosecond_saved| *picosecond_saved >= 100)
        .count()
}

#[derive(Clone, Debug)]
struct RoutePos {
    pos: Pos,
    distance: usize,
}

fn get_normal_route(map: &Map, start: Pos) -> HashMap<Pos, usize> {
    let mut route = Vec::<RoutePos>::with_capacity(map.height() * map.width() / 2);
    route.push(RoutePos {
        distance: 0,
        pos: start,
    });

    let offsets = Direction::all_directions().map(Direction::to_offset);

    let mut current_pos = route.last().unwrap();

    while map[&current_pos.pos] != 'E' {
        let next_pos = offsets
            .iter()
            .find_map(|direction| {
                current_pos
                    .pos
                    .try_add(direction)
                    .and_then(|next_pos| match map.get(&next_pos) {
                        None | Some('#') => None,
                        _ => Some(next_pos),
                    })
                    .filter(|next_pos| {
                        route.len() == 1
                            || route
                                .get(route.len() - 2)
                                .map_or(true, |pos| &pos.pos != next_pos)
                    })
            })
            .expect("Should be a valid route");

        route.push(RoutePos {
            pos: next_pos,
            distance: current_pos.distance + 1,
        });

        current_pos = route.last().unwrap();
    }

    route.into_iter().map(|x| (x.pos, x.distance)).collect()
}

fn get_jumps_from_pos<'a>(map: &'a Map, start: &'a Pos) -> impl Iterator<Item = Pos> + use<'a> {
    Direction::all_directions()
        .map(Direction::to_offset)
        .into_iter()
        .filter_map(|direction| {
            let cheat = start.try_add(&direction)?;

            let cheat_2 = cheat.try_add(&direction)?;

            if map[&cheat] != '#' || map.get(&cheat_2).map_or(true, |x| *x == '#') {
                return None;
            }

            Some(cheat_2)
        })
}
