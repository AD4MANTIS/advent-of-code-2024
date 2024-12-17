use std::collections::HashSet;

use lib::maps::prelude::{Direction, Map, Pos};

lib::day!(16, part1, example => 7036, example_2 raw(r"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################") => 11048);

const MAX_SCORE: usize = 300_000;

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

    iterate(
        &map,
        NextPositionsIterator::new(
            &map,
            ReindeerPos {
                pos: start,
                direction: Direction::Right,
                score: 0,
                visited_pos: HashSet::new(),
            },
        ),
    )
    .take(10)
    .min()
    .unwrap_or_default()
}

fn iterate<'a>(
    map: &'a Map,
    iter: NextPositionsIterator<'a>,
) -> impl Iterator<Item = usize> + use<'a> {
    iter.into_iter().filter(|x| x.score <= MAX_SCORE).flat_map(
        |reindeer| -> Box<dyn Iterator<Item = usize> + '_> {
            if map[&reindeer.pos] == 'E' {
                return Box::new(std::iter::once(reindeer.score));
            }

            Box::new(iterate(map, NextPositionsIterator::new(map, reindeer)))
        },
    )
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum NextPosState {
    Start,
    Left,
    Right,
    Forward,
    Done,
}

impl NextPosState {
    const fn next(self) -> Self {
        match self {
            Self::Start => Self::Left,
            Self::Left => Self::Right,
            Self::Right => Self::Forward,
            Self::Forward | Self::Done => Self::Done,
        }
    }
}

struct NextPositionsIterator<'a> {
    map: &'a Map,
    initial_pos: ReindeerPos,
    state: NextPosState,
    new_visited_pos: Option<HashSet<Pos>>,
}

impl<'a> NextPositionsIterator<'a> {
    const fn new(map: &'a Map, initial_pos: ReindeerPos) -> Self {
        Self {
            map,
            initial_pos,
            state: NextPosState::Start,
            new_visited_pos: None,
        }
    }

    fn can_move(&self, pos: &Pos) -> bool {
        if self.initial_pos.score > MAX_SCORE {
            return false;
        }

        if self.map[pos] == '#' {
            return false;
        }

        if self.initial_pos.visited_pos.contains(pos) {
            return false;
        }

        true
    }

    fn try_left_or_right_movement(&self, direction: Direction) -> Option<ReindeerPos> {
        let pos = self
            .initial_pos
            .pos
            .try_add(&direction.to_offset())
            .unwrap();

        if self.can_move(&pos) {
            return Some(ReindeerPos {
                pos,
                direction,
                score: self.initial_pos.score + 1001,
                visited_pos: self.new_visited_pos.clone().unwrap(),
            });
        }
        None
    }
}

impl Iterator for NextPositionsIterator<'_> {
    type Item = ReindeerPos;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.state = self.state.next();
            match self.state {
                NextPosState::Start => {}
                NextPosState::Left => {
                    let mut new_visited_pos = self.initial_pos.visited_pos.clone();
                    new_visited_pos.insert(self.initial_pos.pos.clone());

                    self.new_visited_pos = Some(new_visited_pos);

                    let next =
                        self.try_left_or_right_movement(self.initial_pos.direction.turn_left());
                    if next.is_some() {
                        return next;
                    }
                }
                NextPosState::Right => {
                    let next =
                        self.try_left_or_right_movement(self.initial_pos.direction.turn_right());
                    if next.is_some() {
                        return next;
                    }
                }
                NextPosState::Forward => {
                    let pos = self
                        .initial_pos
                        .pos
                        .try_add(&self.initial_pos.direction.to_offset())
                        .unwrap();
                    if self.can_move(&pos) {
                        return Some(ReindeerPos {
                            pos,
                            direction: self.initial_pos.direction,
                            score: self.initial_pos.score + 1,
                            visited_pos: self.new_visited_pos.take().unwrap(),
                        });
                    }
                }
                NextPosState::Done => return None,
            };
        }
    }
}
