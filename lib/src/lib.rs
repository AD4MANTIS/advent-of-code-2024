pub extern crate paste;
pub mod bench_macro;
pub mod day_macro;
pub mod maps;
mod print_timer;
pub mod test_macro;

pub use print_timer::*;

pub trait ToVec {
    type Item;

    fn to_vec(self) -> Vec<Self::Item>;
}

impl<T: Iterator> ToVec for T {
    type Item = T::Item;

    fn to_vec(self) -> Vec<Self::Item> {
        self.collect()
    }
}
