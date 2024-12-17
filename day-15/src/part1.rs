use lib::{
    maps::prelude::{Direction, Map, Pos},
    ToVec,
};

lib::day!(15, part1, example => 10092, answer =>  1_487_337);

fn part1(input: &str) -> usize {
    let mut map = Map::<char>::from(
        input
            .lines()
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
            .as_str(),
    );
    let moves = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .to_vec()
        .join("");

    for next_move in moves.chars() {
        let start_pos = map
            .all_pos_iter()
            .find(|pos| map[pos] == '@')
            .expect("Start not found");
        move_robot(&mut map, start_pos, parse_move(next_move));
    }

    calculate_coordinate_sum(&map)
}

fn parse_move(next_move: char) -> Direction {
    match next_move {
        '<' => Direction::Left,
        '>' => Direction::Right,
        'v' => Direction::Bottom,
        '^' => Direction::Top,
        _ => panic!("unexpected move {next_move}"),
    }
}

fn move_robot(map: &mut Map, current_roboter_position: Pos, next_move: Direction) {
    let direction = next_move.to_offset();

    let mut temp_map = map.clone();

    let mut pos = current_roboter_position;
    let mut current_moving_object = '@';
    loop {
        let next_pos = pos.try_add(&direction).expect("Map should have a border");

        match map[&next_pos] {
            '#' => return,
            '.' => {
                if current_moving_object == '@' {
                    temp_map.swap(&pos, &next_pos);
                } else {
                    temp_map[&next_pos] = current_moving_object;
                }
                break;
            }
            'O' => {
                if current_moving_object == '@' {
                    temp_map[&pos] = '.';
                    // Only set the new object if ther robot does its initial move, otherwhise these are all Boxes
                    temp_map[&next_pos] = current_moving_object;
                    current_moving_object = 'O';
                }
            }
            _ => panic!("unexpected field"),
        }

        pos = next_pos;
    }

    *map = temp_map;
}

fn calculate_coordinate_sum(map: &Map) -> usize {
    map.all_pos_iter()
        .filter(|pos| map[pos] == 'O')
        .map(|box_pos| box_pos.y * 100 + box_pos.x)
        .sum()
}
