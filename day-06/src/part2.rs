use std::collections::HashSet;

use lib::maps::prelude::{Direction, Map, Pos};

lib::day!(06, part2, example => 6, answer => 0);

const GUARD: char = '^';
const OBSTACLE: char = '#';

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Guard {
    current_position: Pos,
    direction: Direction,
}

fn part2(input: &str) -> usize {
    let map = Map::from(input);
    let mut guard = Guard {
        current_position: find_start_pos(&map).unwrap(),
        direction: Direction::Top,
    };
    let mut visited_pos = HashSet::new();
    visited_pos.insert(guard.current_position.clone());

    loop {
        let Some(next_pos) = guard.current_position.try_add(&guard.direction.to_offset()) else {
            // Out of bounds
            break;
        };

        let Some(place) = map.get(&next_pos) else {
            // Outside of Map
            break;
        };

        if *place == OBSTACLE {
            guard.direction = guard.direction.turn_right();
        } else {
            visited_pos.insert(next_pos.clone());
            guard.current_position = next_pos;
        }
    }

    visited_pos.len()
}

fn find_start_pos(map: &Map) -> Option<Pos> {
    map.all_pos_iter().find(|pos| map[pos] == GUARD)
}
