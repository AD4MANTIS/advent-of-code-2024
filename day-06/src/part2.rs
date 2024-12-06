use std::{collections::HashSet, hash::Hash};

use lib::maps::prelude::{Direction, Map, Pos};

lib::day!(06, part2, example => 6, answer => 1748);

const GUARD: char = '^';
const OBSTACLE: char = '#';

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Guard {
    current_position: Pos,
    direction: Direction,
}

impl Hash for Guard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Faster hashing improves performance by 40%
        (self.current_position.x
            * (self.current_position.y + 1_000_000_000)
            * (match self.direction {
                Direction::Top => 1,
                Direction::Left => 2,
                Direction::Right => 3,
                Direction::Bottom => 4,
            }))
        .hash(state);
    }
}

fn part2(input: &str) -> usize {
    let map = Map::from(input);
    let guard = Guard {
        current_position: find_start_pos(&map).unwrap(),
        direction: Direction::Top,
    };
    let visited_pos = get_visited_positions(guard.clone(), &map);
    let mut tmp_visited_pos = HashSet::with_capacity(visited_pos.len());

    visited_pos
        .into_iter()
        .filter(|guard_position| {
            let mut map_with_obstacle = map.clone();

            *map_with_obstacle
                .get_mut(guard_position)
                .expect("Guard movment must be inside Map") = OBSTACLE;

            is_looping(guard.clone(), &map_with_obstacle, &mut tmp_visited_pos)
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

fn is_looping(mut guard: Guard, map: &Map, visited_pos: &mut HashSet<Guard>) -> bool {
    // reuse HashSet over iterations. This should only need to reserve the memory once and reduces
    // time by -55%
    visited_pos.clear();
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
