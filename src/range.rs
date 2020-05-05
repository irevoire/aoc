use crate::coord::Coord;
use anyhow::{ensure, Result};
use std::fmt;
use std::ops;

#[derive(Debug)]
pub struct Range<I> {
    pub start: Coord<I>,
    current: Coord<I>,
    finished: bool,
    pub end: Coord<I>,
}

impl<I: fmt::Debug + Clone + Ord> Range<I> {
    pub fn new(start: Coord<I>, end: Coord<I>) -> Result<Self> {
        ensure!(
            start < end,
            "You can’t create a range with starting after the end: {:?} {:?}",
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
