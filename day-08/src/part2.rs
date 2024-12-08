use std::collections::{HashMap, HashSet};

use lib::maps::prelude::{Map, Pos};

lib::day!(08, part2, example => 34, answer => 1019);

fn part2(input: &str) -> usize {
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
    if antennas.len() <= 1 {
        return vec![];
    }

    let mut positions = Vec::with_capacity(antennas.len());

    for antenna in antennas.iter().enumerate() {
        positions.push(antenna.1.clone());

        for second_antenna in antennas.iter().skip(antenna.0 + 1) {
            let offset = antenna.1 - second_antenna;

            let mut antinode_pos = antenna.1.clone();
            while let Some(antinode) = antinode_pos.try_add_in_map(map, &offset) {
                positions.push(antinode.clone());
                antinode_pos = antinode;
            }

            let offset = -offset;
            antinode_pos = second_antenna.clone();
            while let Some(antinode) = antinode_pos.try_add_in_map(map, &offset) {
                positions.push(antinode.clone());
                antinode_pos = antinode;
            }
        }
    }
    positions
}
