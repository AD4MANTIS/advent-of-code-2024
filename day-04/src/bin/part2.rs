use lib::map::{
    offset::Offset,
    prelude::{Map, Pos},
};

lib::day!(04, part2, example => 9, answer => 1824);

fn part2(input: &str) -> usize {
    let map = Map::from(input);

    map.all_pos_iter()
        .filter(|pos| map[pos] == 'A')
        .filter(|start| {
            check_diagonals(
                &map,
                start,
                &Offset { x: -1, y: -1 },
                &Offset { x: 1, y: 1 },
            ) && check_diagonals(
                &map,
                start,
                &Offset { x: 1, y: -1 },
                &Offset { x: -1, y: 1 },
            )
        })
        .count()
}

fn check_diagonals(map: &Map, start: &Pos, offset_1: &Offset, offset_2: &Offset) -> bool {
    matches!(
        (
            char_at(map, start.try_add(offset_1)),
            char_at(map, start.try_add(offset_2)),
        ),
        (Some('M'), Some('S')) | (Some('S'), Some('M'))
    )
}

fn char_at(map: &Map, pos: Option<Pos>) -> Option<char> {
    pos.and_then(|pos| map.get(&pos)).copied()
}
