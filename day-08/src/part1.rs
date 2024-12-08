use std::collections::{HashMap, HashSet};

use lib::maps::prelude::{Map, Pos};

lib::day!(08, part1, example => 14, answer => 301);

fn part1(input: &str) -> usize {
    let map = Map::from(input);
    let antennas = get_all_antennas(&map);

    antennas
        .values()
        .flat_map(|antenna_positions| get_all_antinodes(&map, antenna_positions))
        .collect::<HashSet<_>>()
        .len()
}

fn get_all_antennas(map: &Map) -> HashMap<char, Vec<Pos>> {
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();

    for pos in map.all_pos_iter() {
        antennas.entry(map[&pos]).or_default().push(pos);
    }
    antennas.remove(&' ');
    antennas.remove(&'.');

    antennas
}

fn get_all_antinodes(map: &Map, antennas: &[Pos]) -> Vec<Pos> {
    let mut positions = Vec::new();

    for antenna in antennas.iter().enumerate() {
        for second_antenna in antennas.iter().skip(antenna.0 + 1) {
            let offset = antenna.1 - second_antenna;

            if let Some(antinode) = antenna.1.try_add_in_map(map, &offset) {
                positions.push(antinode);
            }

            if let Some(antinode) = second_antenna.try_add_in_map(map, &-offset) {
                positions.push(antinode);
            }
        }
    }
    positions
}
