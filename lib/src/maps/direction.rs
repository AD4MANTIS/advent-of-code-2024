use super::prelude::Offset;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Top,
    Left,
    Right,
    Bottom,
}

impl Direction {
    pub const fn to_offset(self) -> Offset {
        match self {
            Self::Top => Offset::y(-1),
            Self::Left => Offset::x(-1),
            Self::Right => Offset::x(1),
            Self::Bottom => Offset::y(1),
        }
    }

    pub fn all_directions_with_diagonals() -> [Offset; 8] {
        [
            Self::Left.to_offset(),
            Self::Left.to_offset() + Self::Top.to_offset(),
            Self::Top.to_offset(),
            Self::Top.to_offset() + Self::Right.to_offset(),
            Self::Right.to_offset(),
            Self::Right.to_offset() + Self::Bottom.to_offset(),
            Self::Bottom.to_offset(),
            Self::Bottom.to_offset() + Self::Left.to_offset(),
        ]
    }

    pub const fn turn_right(self) -> Self {
        match self {
            Self::Top => Self::Right,
            Self::Right => Self::Bottom,
            Self::Bottom => Self::Left,
            Self::Left => Self::Top,
        }
    }

    pub const fn turn_left(self) -> Self {
        match self {
            Self::Top => Self::Left,
            Self::Left => Self::Bottom,
            Self::Bottom => Self::Right,
            Self::Right => Self::Top,
        }
    }
}
