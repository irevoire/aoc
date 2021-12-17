mod coord;
mod cyclic_list;
mod direction;
mod grid;
pub mod iterator;
pub mod macros;
mod movement;
pub mod num;
pub mod parser;
mod range;
mod turtle;

pub use coord::Coord;
pub use cyclic_list::CyclicList;
pub use direction::Direction;
pub use grid::Grid;
pub use movement::Movement;
pub use range::Range;
pub use turtle::Turtle;

pub use anyhow::*;
pub use atty;
pub use indicatif;
pub use indicatif::{ParallelProgressIterator, ProgressIterator};
pub use itertools;
pub use itertools::Itertools;
pub use rayon::prelude::*;
pub use termion;

#[macro_export]
macro_rules! answer {
    () => (println!());
    ($base:tt, $($args:expr)*) => ({
        use $crate::termion::{color, style};
        print!("{}", color::Fg(color::LightWhite));
        print!($base
        $(, format!("{}{}{}{}{}{}",
            style::Bold, style::Blink, color::Fg(color::Yellow),
            $args, style::Reset, color::Fg(color::LightWhite))
        )*);
        println!("{}", style::Reset);
    })
}

/// Define specific operations defined only on sorted collections.
pub trait SortedCollection<T: Ord> {
    /// Insert an element into a sorted collection.
    fn binary_insert(&mut self, element: T);

    /// Remove an element from a sorted collection.
    fn binary_remove(&mut self, element: T);
}

/// Define specific operations defined only on sorted collections.
pub trait SortedCollectionByKey<E, O: Ord> {
    /// Insert an element into a sorted collection by key.
    fn binary_insert_by_key(&mut self, element: E, f: impl FnMut(&E) -> O);

    /// Insert an element into a sorted collection by key.
    fn binary_remove_by_key(&mut self, element: E, f: impl FnMut(&E) -> O);
}

impl<T: Ord> SortedCollection<T> for Vec<T> {
    /// Insert an element into a sorted collection.
    /// Only efficiant on small vectors, for big collections consider using a [std::collections::BinaryHeap] or an [std::collections::HashSet].
    ///
    /// See also [Vec::binary_remove] and [slice::binary_search].
    /// # Example
    /// ```
    /// use aoc::*;
    /// let mut a = vec![0, 1, 2, 4, 5];
    /// a.binary_insert(2);
    /// assert_eq!(a, vec![0, 1, 2, 2, 4, 5]);
    /// a.binary_insert(3);
    /// assert_eq!(a, vec![0, 1, 2, 2, 3, 4, 5]);
    /// ```
    fn binary_insert(&mut self, element: T) {
        let idx = self.binary_search(&element).unwrap_or_else(|idx| idx);
        self.insert(idx, element);
    }

    /// Remove an element from a sorted [Vec].
    /// Only efficiant on small vectors, for big collections consider using a [std::collections::BinaryHeap] or an [std::collections::HashSet].
    ///
    /// See also [Vec::binary_insert] and [slice::binary_search].
    /// # Example
    /// ```
    /// use aoc::*;
    /// let mut a = vec![0, 1, 2, 4, 5];
    /// a.binary_remove(2);
    /// assert_eq!(a, vec![0, 1, 4, 5]);
    /// a.binary_remove(3);
    /// assert_eq!(a, vec![0, 1, 4, 5]);
    /// ```
    fn binary_remove(&mut self, element: T) {
        if let std::result::Result::Ok(idx) = self.binary_search(&element) {
            self.remove(idx);
        }
    }
}

impl<E, O: Ord> SortedCollectionByKey<E, O> for Vec<E> {
    /// Insert an element by key into a sorted collection.
    /// Only efficiant on small vectors, for big collections consider using a [std::collections::BinaryHeap] or an [std::collections::HashSet].
    ///
    /// See also [Vec::binary_remove] and [slice::binary_search].
    /// # Example
    /// ```
    /// use aoc::*;
    /// let mut a: Vec<isize> = vec![0, -1, 2, 4, -5];
    /// a.binary_insert_by_key(-3, |n| n.abs());
    /// assert_eq!(a, vec![0, -1, 2, -3, 4, -5]);
    /// ```
    fn binary_insert_by_key(&mut self, element: E, mut f: impl FnMut(&E) -> O) {
        let idx = self
            .binary_search_by_key(&f(&element), f)
            .unwrap_or_else(|idx| idx);
        self.insert(idx, element);
    }

    /// Remove an element from a sorted [Vec].
    /// Only efficiant on small vectors, for big collections consider using a [std::collections::BinaryHeap] or an [std::collections::HashSet].
    ///
    /// See also [Vec::binary_insert] and [slice::binary_search].
    /// # Example
    /// ```
    /// use aoc::*;
    /// let mut a: Vec<isize> = vec![0, -1, 2, -3, 4, -5];
    /// a.binary_remove_by_key(-3, |n| n.abs());
    /// assert_eq!(a, vec![0, -1, 2, 4, -5]);
    /// ```
    fn binary_remove_by_key(&mut self, element: E, mut f: impl FnMut(&E) -> O) {
        if let std::result::Result::Ok(idx) = self.binary_search_by_key(&f(&element), f) {
            self.remove(idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_insert() {
        let mut a = vec![0, 1, 2, 4, 5];
        a.binary_insert(2);
        assert_eq!(a, vec![0, 1, 2, 2, 4, 5]);
        a.binary_insert(3);
        assert_eq!(a, vec![0, 1, 2, 2, 3, 4, 5]);
        a.binary_insert(1000);
        assert_eq!(a, vec![0, 1, 2, 2, 3, 4, 5, 1000]);
        a.binary_insert(-1000);
        assert_eq!(a, vec![-1000, 0, 1, 2, 2, 3, 4, 5, 1000]);
    }
}
