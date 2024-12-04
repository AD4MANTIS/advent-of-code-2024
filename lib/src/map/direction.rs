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
            Direction::Left.to_offset(),
            Direction::Left.to_offset() + Direction::Top.to_offset(),
            Direction::Top.to_offset(),
            Direction::Top.to_offset() + Direction::Right.to_offset(),
            Direction::Right.to_offset(),
            Direction::Right.to_offset() + Direction::Bottom.to_offset(),
            Direction::Bottom.to_offset(),
            Direction::Bottom.to_offset() + Direction::Left.to_offset(),
        ]
    }
}
