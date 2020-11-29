//! Define a Coordinate and all kind of operation.
//! **Be really cautious when using this module, we are only working on Manhattan distance**

use crate::{direction, num, range};
use anyhow::Result;
use std::str::FromStr;
use std::{cmp, fmt, ops};

/// Define a 2D `Coord`inate. You need to specify the type you need.
/// Be cautious, if you use an unsigned type you won't be able to use negative coordinate
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coord<I> {
    pub x: I,
    pub y: I,
}

/// Define a coordinate at the origin.
/// Similar to `Coord::at(0, 0)`
/// ```
/// use aoc::Coord;
///
/// let base = Coord::<isize>::at(0, 0);
/// let current: Coord<isize> = Coord::default();
///
/// assert_eq!(base, current);
/// ```
impl<I: Default> Coord<I> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// Create a coordinate at the position you want
/// ```
/// use aoc::Coord;
///
/// let coord = Coord::<isize>::at(42, 35);
///
/// assert_eq!(coord.x, 42);
/// assert_eq!(coord.y, 35);
/// ```
impl<I> Coord<I> {
    pub fn at(x: I, y: I) -> Self {
        Self { x, y }
    }
}

impl<I> Coord<I>
where
    I: ops::Sub<Output = I> + ops::Add<Output = I> + Ord + Copy + Default,
{
    /// Compute the distance between two coordinates.
    ///
    /// **Be cautious, we are working on Manhattan distance**
    /// ```
    /// use aoc::Coord;
    ///
    /// let orig = Coord::default();
    /// let coord = Coord::<isize>::at(1, 1);
    /// let coord2 = Coord::<isize>::at(-1, -1);
    ///
    /// assert_eq!(orig.distance_from(&coord), 2);
    /// assert_eq!(orig.distance_from(&coord2), 2);
    /// ```
    pub fn distance_from(&self, other: &Self) -> I {
        let x = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let y = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        x + y
    }

    /// Compute the distance between from the origin.
    ///
    /// **Be cautious, we are working on Manhattan distance**
    /// ```
    /// use aoc::Coord;
    ///
    /// let coord = Coord::<isize>::at(1, 1);
    /// let coord2 = Coord::<isize>::at(-1, -1);
    ///
    /// assert_eq!(coord.distance_from_base(), 2);
    /// assert_eq!(coord2.distance_from_base(), 2);
    /// ```
    pub fn distance_from_base(&self) -> I {
        Coord::default().distance_from(&self)
    }
}

impl<I: Ord + Clone + fmt::Debug> Coord<I> {
    /// Generate an iterator from a point to another.
    /// The fonction will return an error if the starting point is before the ending point.
    /// ```
    /// use aoc::Coord;
    ///
    /// let start = Coord::<isize>::at(-1, -1);
    /// let end = Coord::<isize>::at(1, 1);
    /// let mut iter = start.to(end).unwrap();
    ///
    /// assert_eq!(iter.next(), Some(Coord::at(-1, -1)));
    /// assert_eq!(iter.next(), Some(Coord::at(0, -1)));
    /// assert_eq!(iter.next(), Some(Coord::at(1, -1)));
    /// assert_eq!(iter.next(), Some(Coord::at(-1, 0)));
    /// assert_eq!(iter.next(), Some(Coord::at(0 , 0)));
    /// assert_eq!(iter.next(), Some(Coord::at(1, 0)));
    /// assert_eq!(iter.next(), Some(Coord::at(-1, 1)));
    /// assert_eq!(iter.next(), Some(Coord::at(0 , 1)));
    /// assert_eq!(iter.next(), Some(Coord::at(1, 1)));
    ///
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn to(self, end: Self) -> Result<range::Range<I>> {
        range::Range::new(self, end)
    }
}

impl<I: Ord> PartialOrd for Coord<I> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<I: Ord> Ord for Coord<I> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl<I: std::ops::Add<Output = I>> std::ops::Add for Coord<I> {
    type Output = Self;

    /// Compute the distance between from the origin.
    ///
    /// ```
    /// use aoc::Coord;
    ///
    /// let base = Coord::default();
    /// let coord = Coord::<isize>::at(1, 1);
    ///
    /// assert_eq!(base + coord, Coord::at(1, 1));
    /// assert_eq!(base + coord + coord, Coord::at(2, 2));
    /// ```
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> From<(T, T)> for Coord<T> {
    /// Create a `Coord` from a tuple in the form of `(x, y)`
    /// ```
    /// use aoc::Coord;
    ///
    /// let coord: Coord<usize> = (0, 0).into();
    ///
    /// assert_eq!(coord, Coord::at(0, 0));
    /// ```
    fn from(t: (T, T)) -> Self {
        Coord::at(t.0, t.1)
    }
}

impl<T> From<&(T, T)> for &Coord<T> {
    /// Create a `Coord` from a tuple in the form of `(x, y)`
    /// ```
    /// use aoc::Coord;
    ///
    /// let coord = &(1_usize, 2_usize);
    /// let coord: &Coord<usize> = coord.into();
    ///
    /// assert_eq!(coord, &Coord::at(1, 2));
    /// ```
    fn from(t: &(T, T)) -> Self {
        unsafe { std::mem::transmute(t) }
    }
}

impl<T> std::ops::Deref for Coord<T> {
    type Target = (T, T);

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> PartialEq<(T, T)> for Coord<T>
where
    T: PartialEq,
{
    /// Allow to makes comparison between `Coord` and tuple.
    ///
    /// *Since it's not possible to implements already existings traits to already existing types in
    /// rust, you'll need to always put the Coord on the left part of the equality.*
    /// ```
    /// use aoc::Coord;
    ///
    /// let tuple = (1_usize, 2_usize);
    /// let coord1 = Coord::at(1, 2);
    /// let coord2 = Coord::at(2, 2);
    ///
    /// assert!(coord1 == tuple);
    /// assert!(coord2 != tuple);
    ///
    /// // assert!(tuple == coord1); // This will not compile because the tuple is on the left side
    /// ```
    fn eq(&self, other: &(T, T)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl<I> std::ops::Add<direction::Direction> for Coord<I>
where
    I: num::One + ops::Add<Output = I> + ops::Sub<Output = I>,
{
    type Output = Self;

    /// Allow to add [Direction](crate::Direction)s to `Coord`.
    ///
    /// ```
    /// use aoc::{Coord, Direction};
    ///
    /// let coord = Coord::default();
    ///
    /// assert_eq!(coord + Direction::North, Coord::at(0, -1));
    /// assert_eq!(coord + Direction::West, Coord::at(-1, 0));
    /// assert_eq!(coord + Direction::East, Coord::at(1, 0));
    /// assert_eq!(coord + Direction::South, Coord::at(0, 1));
    /// ```
    fn add(self, dir: direction::Direction) -> Self {
        use direction::Direction::*;
        match dir {
            West => Self {
                x: self.x - I::one(),
                ..self
            },
            East => Self {
                x: self.x + I::one(),
                ..self
            },
            North => Self {
                y: self.y - I::one(),
                ..self
            },
            South => Self {
                y: self.y + I::one(),
                ..self
            },
        }
    }
}

impl<I, T> std::ops::AddAssign<T> for Coord<I>
where
    Self: std::ops::Add<T, Output = Self> + Clone,
{
    fn add_assign(&mut self, other: T) {
        *self = self.clone() + other
    }
}

impl<I> std::str::FromStr for Coord<I>
where
    I: Eq + Clone + FromStr,
    <I as std::str::FromStr>::Err: std::error::Error + Sync + Send + 'static,
{
    type Err = anyhow::Error;

    /// Parse a `Coord` in the form of x, y.
    /// Whitespaces and parenthesis on the start and end are ignored, the comma is mandatory
    /// though.
    /// ```
    /// use aoc::Coord;
    ///
    /// assert_eq!(Coord::at(0, 0), "0, 0".parse::<Coord<usize>>().unwrap());
    /// assert_eq!(Coord::at(-1, 0), "-1, 0".parse::<Coord<isize>>().unwrap());
    /// assert_eq!(Coord::at(12, 5), "12,5".parse::<Coord<isize>>().unwrap());
    /// assert_eq!(Coord::at(12, 5), "(12,5)".parse::<Coord<isize>>().unwrap());
    /// assert_eq!(Coord::at(12, 5), "  (  12  ,  5  )  ".parse::<Coord<isize>>().unwrap());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .split(',')
            .map(|s| s.trim_matches(|c: char| c.is_whitespace() || c == '(' || c == ')'))
            .collect();

        let x = coords[0].parse::<I>()?;
        let y = coords[1].parse::<I>()?;

        Ok(Coord { x, y })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref() {
        let mut c = Coord::at(0, 0);

        assert_eq!(*c, (0, 0));
        c.x = 2;
        c.y = 3;
        assert_eq!(*c, (2, 3));
    }

    #[test]
    fn test_from() {
        let mut c = Coord::at(0_u32, 0_u32);

        let res: &(u32, u32) = &(0_u32, 0_u32);
        let res: &Coord<u32> = res.into();
        assert_eq!(&c, res);
        c.x = 2;
        c.y = 3;
        assert_eq!(c, Into::<Coord<u32>>::into((2_u32, 3_u32)));
    }
}
