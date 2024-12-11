use lib::ToVec;

lib::day!(11, part1, example => 55312, answer => 217443);

type Stone = usize;
fn part1(input: &str) -> usize {
    let mut stones = input
        .split_whitespace()
        .flat_map(str::parse::<usize>)
        .to_vec();

    for _ in 0..25 {
        let mut stone_index = 0;

        while stone_index < stones.len() {
            if let Some(new_stone) = transform_stone(&mut stones[stone_index]) {
                stones.insert(stone_index + 1, new_stone);
                stone_index += 1;
            }

            stone_index += 1;
        }
    }

    stones.len()
}

fn transform_stone(stone: &mut Stone) -> Option<Stone> {
    if *stone == 0 {
        *stone = 1;
        return None;
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let parts = stone_str.split_at(stone_str.len() / 2);
        *stone = parts.0.parse().unwrap();

        return Some(parts.1.parse().unwrap());
    }

    *stone *= 2024;

    None
}
