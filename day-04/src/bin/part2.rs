use lib::{
    map::prelude::{Direction, Map},
    ToVec,
};

lib::day!(04, part2, example => 9);

fn part2(input: &str) -> usize {
    let map = Map::from(input);
    let x_start_positions = map.all_pos_iter().filter(|pos| map[pos] == 'X').to_vec();

    let directions = Direction::all_directions_with_diagonals();

    let mut found_xmas_count = 0;

    for start in x_start_positions {
        for direction in &directions {
            let mut current_pos = start.clone();
            let str = (0..3)
                .map(|_| {
                    current_pos = current_pos.try_add(direction)?;
                    map.get(&current_pos)
                })
                .collect::<Option<Vec<_>>>();

            if matches!(str.as_deref(), Some(['M', 'A', 'S'])) {
                found_xmas_count += 1;
            }
        }
    }

    found_xmas_count
}
