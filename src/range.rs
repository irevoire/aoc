use crate::coord::Coord;
use anyhow::{ensure, Result};
use std::fmt;
use std::ops;

#[derive(Debug, Clone)]
pub struct Range<I = usize> {
    pub start: Coord<I>,
    current: Coord<I>,
    finished: bool,
    pub end: Coord<I>,
}

impl<I: fmt::Debug + Clone + Ord> Range<I> {
    pub fn new(start: Coord<I>, end: Coord<I>) -> Result<Self> {
        ensure!(
            start <= end,
            "You can’t create a range starting after the end: {:?} {:?}",
            start,
            end
        );
        Ok(Self {
            start: start.clone(),
            current: start,
            finished: false,
            end,
        })
    }

    /// Return `true` if the given coordinate is contained in the [Range].
    ///
    /// # Example
    ///
    /// ```
    /// use aoc::Coord;
    ///
    /// let range = Coord::at(10, 10).to(Coord::at(15, 15)).unwrap();
    ///
    /// assert_eq!(range.contains((12, 13)), true);
    /// assert_eq!(range.contains((10, 10)), true);
    /// assert_eq!(range.contains((15, 15)), true);
    /// assert_eq!(range.contains((14, 16)), false);
    /// assert_eq!(range.contains((9, 12)), false);
    ///
    /// let range = Coord::at(20,-10).to(Coord::at(30, -5)).unwrap();
    /// assert_eq!(range.contains((21, -10)), true);
    ///
    /// ```
    pub fn contains(&self, other: impl Into<Coord<I>>) -> bool {
        let other: Coord<I> = other.into();

        (&self.start.x..=&self.end.x).contains(&&other.x)
            && (&self.start.y..=&self.end.y).contains(&&other.y)
    }
}

impl<I: Clone + fmt::Debug + Eq + ops::AddAssign + crate::num::One> Iterator for Range<I> {
    type Item = Coord<I>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.current.clone();
        if self.finished {
            return None;
        }
        if self.current.x == self.end.x && self.current.y == self.end.y {
            self.finished = true;
        } else if self.current.x == self.end.x {
            self.current.x = self.start.x.clone();
            self.current.y += I::one();
        } else {
            self.current.x += I::one();
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_through() {
        let mut iter = Coord::at(0, 0).to(Coord::at(2, 2)).unwrap();

        assert_eq!(iter.next(), Some(Coord::at(0, 0)));
        assert_eq!(iter.next(), Some(Coord::at(1, 0)));
        assert_eq!(iter.next(), Some(Coord::at(2, 0)));
        assert_eq!(iter.next(), Some(Coord::at(0, 1)));
    }
}
