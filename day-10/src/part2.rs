use lib::{
    maps::prelude::{Direction, Map, Pos},
    ToVec,
};

lib::day!(10, part1, example => 81, answer => 1324);

fn part1(input: &str) -> usize {
    let mut map: Map<u8> = Map::from(input);
    // Because the u8 parses as the char value we need to subtraction the value of '0'
    for pos in map.all_pos() {
        map[&pos] -= u8::try_from('0').unwrap();
    }

    let trailheads = find_trailheads(&map).to_vec();

    trailheads
        .into_iter()
        .map(|trailhead| {
            let mut current_routes = vec![trailhead];
            for height in 1..=9 {
                current_routes = find_next_positions(&map, &current_routes, height);
            }

            current_routes.len()
        })
        .sum()
}

fn find_trailheads(map: &Map<u8>) -> impl Iterator<Item = Pos> + use<'_> {
    map.all_pos_iter().filter_map(|pos| {
        if map[&pos] == 0 {
            return Some(pos);
        }

        None
    })
}

fn find_next_positions(
    map: &Map<u8>,
    current_route_positions: &[Pos],
    next_height: u8,
) -> Vec<Pos> {
    current_route_positions
        .iter()
        .flat_map(|current_trail_pos| {
            Direction::all_directions()
                .into_iter()
                .filter_map(|direction| {
                    let next_pos = current_trail_pos.try_add(&direction.to_offset())?;

                    if map.get(&next_pos) == Some(&next_height) {
                        return Some(next_pos);
                    }

                    None
                })
        })
        .to_vec()
}
