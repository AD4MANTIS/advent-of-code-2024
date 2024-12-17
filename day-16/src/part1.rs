use std::collections::{HashSet, VecDeque};

use lib::maps::prelude::{Direction, Map, Pos};

lib::day!(16, part1, example => 7036, answer => 0);

#[derive(Clone, PartialEq, Eq)]
struct ReindeerPos {
    pos: Pos,
    direction: Direction,
    score: usize,
    visited_pos: HashSet<Pos>,
}

fn part1(input: &str) -> usize {
    let map = Map::<char>::from(input);

    let start = map
        .all_pos_iter()
        .find(|pos| map[pos] == 'S')
        .expect("Should have start");
    let end = map
        .all_pos_iter()
        .find(|pos| map[pos] == 'E')
        .expect("Should have end");

    let mut positions = VecDeque::from([ReindeerPos {
        pos: start,
        direction: Direction::Right,
        score: 0,
        visited_pos: HashSet::new(),
    }]);

    let mut end_positions = vec![];

    while let Some(reindeer) = positions.pop_front() {
        if reindeer.pos == end {
            end_positions.push(reindeer.score);
            continue;
        }
        positions.extend(get_next_positions(&map, &reindeer));
    }

    end_positions.into_iter().min().unwrap_or_default()
}

fn get_next_positions(map: &Map, reindeer: &ReindeerPos) -> Vec<ReindeerPos> {
    let mut new_visited = reindeer.visited_pos.clone();
    new_visited.insert(reindeer.pos.clone());

    let mut positions = vec![];

    let can_move = |pos: &Pos| {
        if new_visited.contains(pos) {
            return false;
        }

        if map[pos] == '#' {
            return false;
        }

        true
    };

    for direction in [
        reindeer.direction.turn_left(),
        reindeer.direction.turn_right(),
    ] {
        let pos = reindeer
            .pos
            .try_add(&direction.to_offset())
            .expect("Map has borders");

        if can_move(&pos) {
            positions.push(ReindeerPos {
                pos,
                direction,
                score: reindeer.score + 1001,
                visited_pos: reindeer.visited_pos.clone(),
            });
        }
    }

    let pos = reindeer
        .pos
        .try_add(&reindeer.direction.to_offset())
        .unwrap();
    if can_move(&pos) {
        positions.push(ReindeerPos {
            pos,
            direction: reindeer.direction,
            score: reindeer.score + 1,
            visited_pos: new_visited,
        });
    }

    positions
}
