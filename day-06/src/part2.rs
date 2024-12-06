use std::collections::HashSet;

use lib::maps::prelude::{Direction, Map, Pos};

lib::day!(06, part2, example => 6, answer => 1748);

const GUARD: char = '^';
const OBSTACLE: char = '#';

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Guard {
    current_position: Pos,
    direction: Direction,
}

fn part2(input: &str) -> usize {
    let map = Map::from(input);
    let guard = Guard {
        current_position: find_start_pos(&map).unwrap(),
        direction: Direction::Top,
    };
    let visited_pos = get_visited_positions(guard.clone(), &map);

    visited_pos
        .into_iter()
        .filter(|guard_position| {
            let mut map_with_obstacle = map.clone();

            *map_with_obstacle
                .get_mut(guard_position)
                .expect("Guard movment must be inside Map") = OBSTACLE;

            is_looping(guard.clone(), &map_with_obstacle)
        })
        .count()
}

fn get_visited_positions(mut guard: Guard, map: &Map) -> HashSet<Pos> {
    let mut visited_pos = Vec::new();

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

    visited_pos.into_iter().collect()
}

fn is_looping(mut guard: Guard, map: &Map) -> bool {
    let mut visited_pos = HashSet::new();
    visited_pos.insert(guard.clone());

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
            guard.current_position = next_pos;
            if !visited_pos.insert(guard.clone()) {
                // Guard movement was already recorded
                return true;
            }
        }
    }

    false
}

fn find_start_pos(map: &Map) -> Option<Pos> {
    map.all_pos_iter().find(|pos| map[pos] == GUARD)
}
