use lib::maps::prelude::{Direction, Map, Pos};

lib::day!(12, part1,
    example_1 raw(r"AAAA
BBCD
BBCC
EEEC") => 140,
    example_2 raw(r"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO") => 772,
    example => 1930,
    answer => 1_549_354);

fn part1(input: &str) -> usize {
    let map = Map::<char>::from(input);

    map.get_all_continues_areas()
        .into_iter()
        .map(|area| calc_perimeter(area.iter(), &map) * area.len())
        .sum()
}

fn calc_perimeter<'a>(area: impl Iterator<Item = &'a Pos>, map: &Map) -> usize {
    area.map(|pos| {
        let plant_type = map[pos];

        Direction::all_directions()
            .into_iter()
            .filter(|direction| {
                pos.try_add(&direction.to_offset())
                    .and_then(|neigbour| map.get(&neigbour))
                    .copied()
                    != Some(plant_type)
            })
            .count()
    })
    .sum()
}
