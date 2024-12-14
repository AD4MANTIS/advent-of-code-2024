use std::{
    collections::HashSet,
    convert::Infallible,
    fmt::{Debug, Display},
    ops::{Index, IndexMut},
    str::FromStr,
};

use super::prelude::{Direction, Offset, Pos};

#[derive(Clone, PartialEq, Eq)]
pub struct Map<T = char> {
    pub rows: Vec<Vec<T>>,
}

impl<T: Copy> Map<T> {
    pub fn new(widht: usize, height: usize, default: T) -> Self {
        Self {
            rows: (0..height).map(|_| [default].repeat(widht)).collect(),
        }
    }
}

impl<T> Index<&Pos> for Map<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, pos: &Pos) -> &Self::Output {
        &self.rows[pos.y][pos.x]
    }
}

impl<T> IndexMut<&Pos> for Map<T> {
    fn index_mut(&mut self, pos: &Pos) -> &mut Self::Output {
        &mut self.rows[pos.y][pos.x]
    }
}

impl<T: Display + Debug> std::fmt::Debug for Map<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            if f.alternate() {
                f.write_str(
                    &(row
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" ")
                        + "\n"),
                )?;
            } else {
                f.write_fmt(format_args!("{:?}\n", row))?;
            }
        }

        Ok(())
    }
}

impl<T> Map<T> {
    pub fn width(&self) -> usize {
        self.rows[0].len()
    }

    pub fn height(&self) -> usize {
        self.rows.len()
    }

    pub fn get(&self, pos: &Pos) -> Option<&T> {
        self.rows.get(pos.y)?.get(pos.x)
    }

    pub fn get_mut(&mut self, pos: &Pos) -> Option<&mut T> {
        self.rows.get_mut(pos.y)?.get_mut(pos.x)
    }
}

impl<T: Eq> Map<T> {
    pub fn get_all_continues_areas(&self) -> Vec<Vec<Pos>> {
        let mut areas = Vec::<Vec<Pos>>::new();
        let mut all_visited_pos = HashSet::<Pos>::with_capacity(self.width() * self.height());

        for pos in self.all_pos_iter() {
            if all_visited_pos.contains(&pos) {
                continue;
            }

            let mut positions_in_area = Vec::new();
            self._get_continuas_areas(pos, &mut all_visited_pos, &mut positions_in_area);

            all_visited_pos.extend(positions_in_area.clone());
            areas.push(positions_in_area);
        }

        areas
    }

    fn _get_continuas_areas(
        &self,
        start_pos: Pos,
        visited_pos: &mut HashSet<Pos>,
        positions_in_area: &mut Vec<Pos>,
    ) {
        if !visited_pos.insert(start_pos.clone()) {
            return;
        }

        let item = &self[&start_pos];

        for direction in Direction::all_directions() {
            if let Some(next_pos) = start_pos.try_add(&direction.to_offset()) {
                if self.get(&next_pos) == Some(item) {
                    self._get_continuas_areas(next_pos, visited_pos, positions_in_area);
                }
            }
        }

        positions_in_area.push(start_pos);
    }
}

impl<T: Clone> Map<T> {
    pub fn swap(&mut self, pos1: &Pos, pos2: &Pos) {
        let Some(val1) = self.get(pos1).cloned() else {
            return;
        };

        let Some(val2) = self.get(pos2).cloned() else {
            return;
        };

        *self.get_mut(pos1).unwrap() = val2;

        *self.get_mut(pos2).unwrap() = val1;
    }
}

impl<T> Map<T> {
    pub const fn columns(&self) -> ColumnsIter<T> {
        ColumnsIter(self, 0)
    }

    pub const fn column_iter(&self, col: usize) -> ColumnIter<T> {
        ColumnIter(self, Pos { x: col, y: 0 })
    }

    pub fn all_pos(&self) -> Vec<Pos> {
        self.all_pos_iter().collect()
    }

    pub const fn all_pos_iter(&self) -> AllPosIter<T> {
        AllPosIter(self, None)
    }
}

pub struct AllPosIter<'a, T>(&'a Map<T>, Option<Pos>);

impl<T> Iterator for AllPosIter<'_, T> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.1 {
            Some(current_pos) => {
                current_pos.x += 1;
                if current_pos.x == self.0.width() {
                    current_pos.x = 0;
                    current_pos.y += 1;

                    if current_pos.y == self.0.height() {
                        return None;
                    }
                }
            }
            None => self.1 = Some(Pos::default()),
        };

        self.1.clone()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.0.width() - self.1.as_ref().map(|pos| pos.x).unwrap_or_default() - 1,
            Some(self.0.width() * self.0.height()),
        )
    }
}

pub struct ColumnIter<'a, T>(&'a Map<T>, Pos);
pub struct ColumnsIter<'a, T>(&'a Map<T>, usize);

impl<T: Copy> Iterator for ColumnIter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0.get(&self.1)?;

        self.1 = self.1.clone().try_add_consuming(Offset::y(1))?;

        Some(*current)
    }
}

impl<'a, T: Copy> Iterator for ColumnsIter<'a, T> {
    type Item = ColumnIter<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 >= self.0.rows.first()?.len() {
            return None;
        }

        self.1 += 1;

        Some(self.0.column_iter(self.1 - 1))
    }
}

impl<T: Default + Clone> Map<T> {
    pub fn with_size(x: usize, y: usize) -> Self {
        let row = (0..x).map(|_| T::default()).collect::<Vec<_>>();
        Self {
            rows: (0..y).map(|_| row.clone()).collect(),
        }
    }
}

impl<T> FromStr for Map<T>
where
    Self: for<'a> From<&'a str>,
{
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl<T: TryFrom<char>> From<&str> for Map<T> {
    fn from(value: &str) -> Self {
        Self {
            rows: value
                .lines()
                .map(|line| line.chars().flat_map(T::try_from).collect::<Vec<_>>())
                .collect(),
        }
    }
}

#[cfg(test)]
pub(super) fn get_test_map() -> Map<char> {
    Map::<char> {
        rows: vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
        ],
    }
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn create_map() {
        let result = Map::<char>::from_str(
            "\
123
456
789
abc
def
",
        )
        .unwrap();

        let expected = get_test_map();

        assert_eq!(result, expected);
    }

    #[test]
    fn get_map() {
        let map = get_test_map();

        assert_eq!(map.get(&Pos { x: 0, y: 0 }), Some(&'1'));
        assert_eq!(map.get(&Pos { x: 1, y: 0 }), Some(&'2'));
        assert_eq!(map.get(&Pos { x: 0, y: 1 }), Some(&'4'));
        assert_eq!(map.get(&Pos { x: 2, y: 4 }), Some(&'f'));
        assert_eq!(map.get(&Pos { x: 3, y: 0 }), None);
        assert_eq!(map.get(&Pos { x: 2, y: 5 }), None);
    }

    #[test]
    fn get_all_pos() {
        let map = get_test_map().all_pos();

        assert_eq!(
            map,
            vec![
                Pos { x: 0, y: 0 },
                Pos { x: 1, y: 0 },
                Pos { x: 2, y: 0 },
                Pos { x: 0, y: 1 },
                Pos { x: 1, y: 1 },
                Pos { x: 2, y: 1 },
                Pos { x: 0, y: 2 },
                Pos { x: 1, y: 2 },
                Pos { x: 2, y: 2 },
                Pos { x: 0, y: 3 },
                Pos { x: 1, y: 3 },
                Pos { x: 2, y: 3 },
                Pos { x: 0, y: 4 },
                Pos { x: 1, y: 4 },
                Pos { x: 2, y: 4 },
            ]
        );
    }

    #[test]
    fn get_all_pos_iter() {
        let map = get_test_map();
        let mut pos_iter = map.all_pos_iter();

        assert_eq!(pos_iter.next(), Some(Pos { x: 0, y: 0 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 1, y: 0 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 2, y: 0 }));
        assert_eq!(pos_iter.next(), Some(Pos { x: 0, y: 1 }));
        let mut pos_iter = pos_iter.skip(10);
        assert_eq!(pos_iter.next(), Some(Pos { x: 2, y: 4 }));
        assert_eq!(pos_iter.next(), None);
    }

    #[test]
    fn column_iterator() {
        let map = &get_test_map();
        let mut col_iter = map.column_iter(0);

        assert_eq!(col_iter.next(), Some('1'));
        assert_eq!(col_iter.next(), Some('4'));
        assert_eq!(col_iter.next(), Some('7'));
        assert_eq!(col_iter.next(), Some('a'));
        assert_eq!(col_iter.next(), Some('d'));
        assert_eq!(col_iter.next(), None);

        col_iter = map.column_iter(99);
        assert_eq!(col_iter.next(), None);
    }
}
