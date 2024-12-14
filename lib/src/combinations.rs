#[derive(Clone)]
pub struct CombinationsIterator<T: Copy + Eq> {
    options: Vec<T>,
    max_combinations: usize,
    last_combination: Option<Vec<T>>,
}

impl<T: Copy + Eq> Iterator for CombinationsIterator<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(last_combination) = &mut self.last_combination else {
            self.last_combination =
                Some([*self.options.first().unwrap()].repeat(self.max_combinations));
            return self.last_combination.clone();
        };

        let last = self.options.last().unwrap();

        if last_combination.iter().all(|x| *x == *last) {
            return None;
        }

        for item in last_combination.iter_mut().rev() {
            if item == last {
                *item = self.options[0];
            } else {
                *item = self.options[self
                    .options
                    .iter()
                    .position(|x| x == item)
                    .expect("Options should contain all options")
                    + 1];
                break;
            }
        }

        self.last_combination.clone()
    }
}

pub trait CombinationsIter<T: Copy + Eq>: Iterator<Item = T> {
    fn combinations(self, length: usize) -> CombinationsIterator<T>
    where
        Self: Sized,
    {
        let options: Vec<_> = self.into_iter().collect();
        assert!(!options.is_empty());
        CombinationsIterator {
            options,
            max_combinations: length,
            last_combination: None,
        }
    }
}

impl<T: Copy + Eq, I: Iterator<Item = T>> CombinationsIter<T> for I {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn two_options() {
        let combinations = ['a', 'b'].into_iter().combinations(3).collect::<Vec<_>>();
        assert_eq!(
            combinations,
            [
                ['a', 'a', 'a'],
                ['a', 'a', 'b'],
                ['a', 'b', 'a'],
                ['a', 'b', 'b'],
                ['b', 'a', 'a'],
                ['b', 'a', 'b'],
                ['b', 'b', 'a'],
                ['b', 'b', 'b']
            ]
        );
    }

    #[test]
    fn two_options_25_combinations() {
        for _ in [1, 2].iter().combinations(25) {}
    }

    #[test]
    fn three_options() {
        let combinations = [1, 2, 3].into_iter().combinations(2).collect::<Vec<_>>();
        assert_eq!(
            combinations,
            [
                [1, 1],
                [1, 2],
                [1, 3],
                [2, 1],
                [2, 2],
                [2, 3],
                [3, 1],
                [3, 2],
                [3, 3]
            ]
        )
    }
}
