use std::iter::repeat_n;

use lib::ToVec;

lib::day!(21, part1, example => 126384, answer => 0);

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let code = line.chars().filter_map(|c| NumericKeypad::try_from(c).ok());
            let shortes_sequence = find_shortes_sequence(code);
            let shortes_sequence = find_shortes_sequence(shortes_sequence);
            let shortes_sequence = find_shortes_sequence(shortes_sequence);

            {
                let sequence = shortes_sequence.clone().to_vec();
                println!("{line} (len = {1}): {0:?}", sequence, sequence.len());
            }

            let code_num = line[..3].parse::<usize>().unwrap();

            shortes_sequence.count() * code_num
        })
        .sum()
}

/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
#[derive(Clone, Copy, Debug)]
enum NumericKeypad {
    Num(u8),
    Activate,
}

impl Keypad for NumericKeypad {
    fn row(self) -> u8 {
        match self {
            Self::Activate => 0,
            Self::Num(0) => 0,
            Self::Num(num) => ((num - 1) / 3) + 1,
        }
    }

    fn col(self) -> u8 {
        match self {
            Self::Activate => 2,
            Self::Num(0) => 1,
            Self::Num(num) => (num - 1) % 3,
        }
    }

    fn row_movement_first(self) -> bool {
        self.row() != 0
    }
}

impl Default for NumericKeypad {
    fn default() -> Self {
        Self::Activate
    }
}

impl TryFrom<char> for NumericKeypad {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => Self::Num(0),
            '1' => Self::Num(1),
            '2' => Self::Num(2),
            '3' => Self::Num(3),
            '4' => Self::Num(4),
            '5' => Self::Num(5),
            '6' => Self::Num(6),
            '7' => Self::Num(7),
            '8' => Self::Num(8),
            '9' => Self::Num(9),
            'A' => Self::Activate,
            _ => return Err(()),
        })
    }
}

/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
#[derive(Clone, Copy)]
enum DirectionalKeypad {
    Left,
    Right,
    Up,
    Down,
    Activate,
}

impl std::fmt::Debug for DirectionalKeypad {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Left => "<",
            Self::Right => ">",
            Self::Up => "^",
            Self::Down => "v",
            Self::Activate => "A",
        })
    }
}

impl Keypad for DirectionalKeypad {
    fn row(self) -> u8 {
        match self {
            Self::Left | Self::Right | Self::Down => 0,
            Self::Up | Self::Activate => 1,
        }
    }

    fn col(self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Down | Self::Up => 1,
            Self::Right | Self::Activate => 2,
        }
    }

    fn row_movement_first(self) -> bool {
        self.row() == 0
    }
}

impl Default for DirectionalKeypad {
    fn default() -> Self {
        Self::Activate
    }
}

impl TryFrom<char> for DirectionalKeypad {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            'A' => Self::Activate,
            _ => return Err(()),
        })
    }
}

trait Keypad: Copy + Default + TryFrom<char, Error = ()> {
    fn row(self) -> u8;
    fn col(self) -> u8;
    fn row_movement_first(self) -> bool;

    fn input_sequence_to(self, to: Self) -> impl Iterator<Item = DirectionalKeypad> {
        type Dir = DirectionalKeypad;

        let col_diff = to.col() as isize - self.col() as isize;
        let row_diff = to.row() as isize - self.row() as isize;

        let col_movement = repeat_n(
            if row_diff > 0 { Dir::Up } else { Dir::Down },
            row_diff.unsigned_abs(),
        );
        let row_movement = repeat_n(
            if col_diff > 0 { Dir::Right } else { Dir::Left },
            col_diff.unsigned_abs(),
        );

        if self.row_movement_first() {
            row_movement.chain(col_movement)
        } else {
            col_movement.chain(row_movement)
        }
    }
}

#[derive(Default)]
struct Robot<T: Keypad> {
    pub position: T,
}

fn find_shortes_sequence<T: Keypad>(
    code: impl Iterator<Item = T>,
) -> impl Iterator<Item = DirectionalKeypad> + Clone {
    let mut robot = Robot::<T>::default();

    code.flat_map(|c| {
        let sequence = robot
            .position
            .input_sequence_to(c)
            .chain([DirectionalKeypad::Activate]);
        robot.position = c;
        sequence
    })
    .to_vec()
    .into_iter()
}
