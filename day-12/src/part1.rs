use lib::maps::{
    offset::Offset,
    prelude::{Direction, Map, Pos},
};

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
        .map(|area| area.len() * calc_perimeter(area.into_iter(), &map))
        .sum()
}

fn calc_perimeter(area: impl Iterator<Item = Pos>, map: &Map) -> usize {
    let directions: [Offset; 4] = Direction::all_directions().map(Direction::to_offset);

    area.map(|pos| {
        let plant_type = map[&pos];

        directions
            .iter()
            .filter(|direction| {
                pos.try_add(direction)
                    .and_then(|neigbour| map.get(&neigbour))
                    != Some(&plant_type)
            })
            .count()
    })
    .sum()
}
