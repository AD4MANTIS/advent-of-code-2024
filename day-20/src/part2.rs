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
        .flat_map(|route_pos| {
            let distance = route_pos.distance;

            get_jumps_from_pos(&route, &route_pos.pos).filter_map(
                move |(cheated_pos, cheat_duration)| {
                    cheated_pos
                        .distance
                        .checked_sub(distance)
                        .map(|x| x - cheat_duration)
                },
            )
        })
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

    route
}

fn get_jumps_from_pos<'a>(
    map: &'a [RoutePos],
    start: &'a Pos,
) -> impl Iterator<Item = (&'a RoutePos, usize)> {
    map.iter().filter_map(|route_pos| {
        let distance = (&route_pos.pos - start).abs_distance();

        if distance > 20 {
            return None;
        }

        Some((route_pos, distance))
    })
}
