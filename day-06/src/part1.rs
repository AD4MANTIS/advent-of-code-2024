use std::collections::HashSet;

use lib::maps::prelude::{Direction, Map, Pos};

lib::day!(06, part1, example => 41, answer => 5312);

const GUARD: char = '^';
const OBSTACLE: char = '#';

struct Guard {
    current_position: Pos,
    direction: Direction,
}

fn part1(input: &str) -> usize {
    let map = Map::from(input);
    let mut guard = Guard {
        current_position: find_start_pos(&map).unwrap(),
        direction: Direction::Top,
    };
    let mut visited_pos = Vec::new();
    visited_pos.push(guard.current_position.clone());

    loop {
        let Some(next_pos) = guard.current_position.try_add(&guard.direction.to_offset()) else {
            break;
        };
        let Some(place) = map.get(&next_pos) else {
            // Outside of Map
            break;
        };

        if *place == OBSTACLE {
            guard.direction = guard.direction.turn_right();
        } else {
            visited_pos.push(next_pos.clone());
            guard.current_position = next_pos;
        }
    }

    visited_pos.into_iter().collect::<HashSet<_>>().len()
}

fn find_start_pos(map: &Map) -> Option<Pos> {
    map.all_pos_iter().find(|pos| map[pos] == GUARD)
}
