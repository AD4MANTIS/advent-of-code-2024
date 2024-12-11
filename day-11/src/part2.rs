use std::collections::HashMap;

lib::day!(11, part2, answer => 257_246_536_026_785);

type Stone = usize;
const ITERATIONS: usize = 75;

fn part2(input: &str) -> usize {
    let mut cache = HashMap::new();

    input
        .split_whitespace()
        .flat_map(str::parse::<usize>)
        .map(|x| transform_stone_with_cache(x, 0, &mut cache))
        .sum()
}

fn transform_stone_with_cache(
    stone: Stone,
    blink_counter: usize,
    cache: &mut HashMap<(Stone, usize), usize>,
) -> usize {
    if let Some(cached_result) = cache.get(&(stone, blink_counter)) {
        return *cached_result;
    }

    let result = transform_stone(stone, blink_counter, cache);

    cache.insert((stone, blink_counter), result);

    result
}

fn transform_stone(
    stone: Stone,
    mut blink_counter: usize,
    cache: &mut HashMap<(Stone, usize), usize>,
) -> usize {
    if blink_counter == ITERATIONS {
        return 1;
    }

    blink_counter += 1;
    if stone == 0 {
        return transform_stone_with_cache(1, blink_counter, cache);
    }

    let stone_str = stone.to_string();
    if stone_str.len() % 2 == 0 {
        let parts = stone_str.split_at(stone_str.len() / 2);

        return transform_stone_with_cache(parts.0.parse().unwrap(), blink_counter, cache)
            + transform_stone_with_cache(parts.1.parse().unwrap(), blink_counter, cache);
    }

    transform_stone_with_cache(stone * 2024, blink_counter, cache)
}
