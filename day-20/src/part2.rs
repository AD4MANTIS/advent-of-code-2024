use lib::maps::prelude::{Direction, Map, Pos};

lib::day!(20, part2, example => 0, answer => 989_316);

fn part2(input: &str) -> usize {
    let map = Map::from(input);

    let start = map
        .all_pos_iter()
        .find(|pos| map[pos] == 'S')
        .expect("Should have Start");

    let route = get_normal_route(&map, start);

    route
        .iter()
        .flat_map(|route_pos| get_time_saved_for_jumps_from_pos(&route, route_pos))
        .filter(|picosecond_saved| *picosecond_saved >= 100)
        .count()
}

#[derive(Clone, Debug)]
struct RoutePos {
    pos: Pos,
    distance: usize,
}

fn get_normal_route(map: &Map, start: Pos) -> Vec<RoutePos> {
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

    route
}

fn get_time_saved_for_jumps_from_pos<'a>(
    map: &'a [RoutePos],
    start: &'a RoutePos,
) -> impl Iterator<Item = usize> + use<'a> {
    map.iter().filter_map(|cheated_route_pos| {
        let cheated_distance = cheated_route_pos.distance.checked_sub(start.distance)?;

        let cheat_distance = (&cheated_route_pos.pos - &start.pos).abs_distance();

        if cheat_distance > 20 {
            return None;
        }

        cheated_distance.checked_sub(cheat_distance)
    })
}
