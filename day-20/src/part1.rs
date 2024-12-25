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
        .values()
        .flat_map(|route_pos| {
            let RoutePos { distance, pos } = route_pos;

            get_jumps_from_pos(&map, &route_pos.pos)
                .filter_map(|cheated_pos| route.get(&cheated_pos))
                .filter(move |cheated_pos| cheated_pos.distance > *distance)
                .map(move |cheated_pos| {
                    cheated_pos.distance - distance - (&cheated_pos.pos - pos).abs_distance()
                })
        })
        .filter(|picosecond_saved| *picosecond_saved >= 100)
        .count()
}

#[derive(Clone, Debug)]
struct RoutePos {
    pos: Pos,
    distance: usize,
}

fn get_normal_route(map: &Map, start: Pos) -> HashMap<Pos, RoutePos> {
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
            .filter_map(|direction| current_pos.pos.try_add(direction))
            .find(|next_pos| match map.get(next_pos) {
                Some('#') => false,
                _ => route.len() == 1 || &route[route.len() - 2].pos != next_pos,
            })
            .expect("Should be a valid route");

        route.push(RoutePos {
            pos: next_pos,
            distance: current_pos.distance + 1,
        });

        current_pos = route.last().unwrap();
    }

    route.into_iter().map(|x| (x.pos.clone(), x)).collect()
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
