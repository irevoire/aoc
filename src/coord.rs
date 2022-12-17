//! Define a Coordinate and all kind of operation.
//! **Be really cautious when using this module, we are only working on Manhattan distance**

use crate::{direction, num, range};
use anyhow::Result;
use std::cmp::Reverse;
use std::collections::HashSet;
use std::str::FromStr;
use std::{cmp, fmt, ops};

/// Define a 2D `Coord`inate. You need to specify the type you need.
/// Be cautious, if you use an unsigned type you won't be able to use negative coordinate
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coord<I = usize> {
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
    /// See also [Coord::chebyshev_distance_from], [Coord::manhattan_distance_from_base].
    /// # Example
    /// ```
    /// use aoc::Coord;
    ///
    /// let orig = Coord::default();
    /// let coord = Coord::<isize>::at(1, 1);
    /// let coord2 = Coord::<isize>::at(-1, -1);
    ///
    /// assert_eq!(orig.manhattan_distance_from(&coord), 2);
    /// assert_eq!(orig.manhattan_distance_from(&coord2), 2);
    /// ```
    pub fn manhattan_distance_from(&self, other: &Self) -> I {
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
    /// See also [Coord::manhattan_distance_from], [Coord::chebyshev_distance_from_base].
    /// # Example
    /// ```
    /// use aoc::Coord;
    ///
    /// let coord = Coord::<isize>::at(1, 1);
    /// let coord2 = Coord::<isize>::at(-1, -1);
    ///
    /// assert_eq!(coord.manhattan_distance_from_base(), 2);
    /// assert_eq!(coord2.manhattan_distance_from_base(), 2);
    /// ```
    pub fn manhattan_distance_from_base(&self) -> I {
        Coord::default().manhattan_distance_from(self)
    }

    /// Compute the distance between two coordinates.
    ///
    /// See also [Coord::manhattan_distance_from], [Coord::chebyshev_distance_from_base].
    /// # Example
    /// ```
    /// use aoc::Coord;
    ///
    /// let orig = Coord::default();
    /// let coord = Coord::<isize>::at(1, 1);
    /// let coord2 = Coord::<isize>::at(-1, -1);
    ///
    /// assert_eq!(orig.chebyshev_distance_from(&coord), 1);
    /// assert_eq!(orig.chebyshev_distance_from(&coord2), 1);
    /// assert_eq!(coord.chebyshev_distance_from(&coord2), 2);
    /// ```
    pub fn chebyshev_distance_from(&self, other: &Self) -> I {
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

        x.max(y)
    }

    /// Compute the distance between from the origin.
    ///
    /// See also [Coord::chebyshev_distance_from], [Coord::manhattan_distance_from_base].
    /// # Example
    /// ```
    /// use aoc::Coord;
    ///
    /// let coord = Coord::<isize>::at(1, 1);
    /// let coord2 = Coord::<isize>::at(-1, -1);
    ///
    /// assert_eq!(coord.chebyshev_distance_from_base(), 2);
    /// assert_eq!(coord2.chebyshev_distance_from_base(), 2);
    /// ```
    pub fn chebyshev_distance_from_base(&self) -> I {
        Coord::default().manhattan_distance_from(self)
    }
}

impl<I> Coord<I>
where
    I: ops::Sub<Output = I> + ops::Add<Output = I> + num::One + Ord + Copy + Default,
{
    /// Move toward the other coord in one Manhattan step.
    ///
    /// If the coord is in the `o` position it can move on any `#` position.
    ///
    /// ```no_rust
    /// .........
    /// ...###...
    /// ...#o#...
    /// ...###...
    /// .........
    /// ```
    ///
    /// ```
    /// use aoc::Coord;
    ///
    /// let mut coord = Coord::<isize>::at(0, 0);
    ///
    /// coord.move_toward(&Coord::at(0, 0));
    /// assert_eq!(coord, Coord::at(0, 0));
    ///
    /// let mut coord = Coord::default();
    /// coord.move_toward(&Coord::at(-10, -10));
    /// assert_eq!(coord, Coord::at(-1, -1));
    /// ```
    pub fn move_toward(&mut self, target: &Self) {
        if self.x < target.x {
            self.x = self.x + I::one();
        } else if self.x > target.x {
            self.x = self.x - I::one();
        }

        if self.y < target.y {
            self.y = self.y + I::one();
        } else if self.y > target.y {
            self.y = self.y - I::one();
        }
    }

    /// Indicate if two points are adjacent with a manhattan distance.
    ///
    /// See also [Coord::manhattan_distance_from], [Coord::chebyshev_adjacent].
    /// # Example
    /// ```
    /// use aoc::Coord;
    ///
    /// let coord = Coord::<isize>::at(1, 0);
    /// let coord2 = Coord::<isize>::at(-1, 0);
    ///
    /// assert_eq!(coord.is_manhattan_adjacent(&Coord::default()), true);
    /// assert_eq!(coord2.is_manhattan_adjacent(&Coord::default()), true);
    /// assert_eq!(coord.is_manhattan_adjacent(&coord2), false);
    /// ```
    pub fn is_manhattan_adjacent(&self, other: &Self) -> bool {
        self.manhattan_distance_from(other) == I::one()
    }

    /// Indicate if two points are adjacent with a chebyshev distance.
    ///
    /// See also [Coord::chebyshev_distance_from], [Coord::manhattan_adjacent].
    /// # Example
    /// ```
    /// use aoc::Coord;
    ///
    /// let coord = Coord::<isize>::at(1, 1);
    /// let coord2 = Coord::<isize>::at(-1, -1);
    ///
    /// assert_eq!(coord.is_chebyshev_adjacent(&Coord::default()), true);
    /// assert_eq!(coord2.is_chebyshev_adjacent(&Coord::default()), true);
    /// assert_eq!(coord.is_chebyshev_adjacent(&coord2), false);
    /// ```
    pub fn is_chebyshev_adjacent(&self, other: &Self) -> bool {
        self.chebyshev_distance_from(other) == I::one()
    }
}

impl<I> Coord<I>
where
    I: ops::Sub<Output = I> + ops::Add<Output = I> + num::One + num::CheckedOp + Copy,
{
    /// Returns an iterator over all the adjacent position.
    ///
    /// See also [Coord::is_manhattan_adjacent], [Coord::chebyshev_adjacent].
    /// # Example
    /// ```
    /// use aoc::Coord;
    ///
    /// let mut coords = Coord::<isize>::at(0, 0).manhattan_adjacent();
    ///
    /// assert_eq!(coords.next(), Some(Coord::at(-1, 0)));
    /// assert_eq!(coords.next(), Some(Coord::at(1, 0)));
    /// assert_eq!(coords.next(), Some(Coord::at(0, -1)));
    /// assert_eq!(coords.next(), Some(Coord::at(0, 1)));
    /// assert_eq!(coords.next(), None);
    /// ```
    pub fn manhattan_adjacent(&self) -> impl Iterator<Item = Coord<I>> {
        let ret = [
            self.x.checked_sub(I::one()).map(|x| Coord::at(x, self.y)),
            self.x.checked_add(I::one()).map(|x| Coord::at(x, self.y)),
            self.y.checked_sub(I::one()).map(|y| Coord::at(self.x, y)),
            self.y.checked_add(I::one()).map(|y| Coord::at(self.x, y)),
        ];

        ret.into_iter().filter_map(|s| s)
    }

    /// Returns an iterator over all the adjacent position.
    ///
    /// See also [Coord::is_manhattan_adjacent], [Coord::chebyshev_adjacent].
    /// # Example
    /// ```
    /// use aoc::Coord;
    ///
    /// let mut coords = Coord::<isize>::at(0, 0).chebyshev_adjacent();
    ///
    /// // top row
    /// assert_eq!(coords.next(), Some(Coord::at(-1, 1)));
    /// assert_eq!(coords.next(), Some(Coord::at(0, 1)));
    /// assert_eq!(coords.next(), Some(Coord::at(1, 1)));
    /// // middle row
    /// assert_eq!(coords.next(), Some(Coord::at(-1, 0)));
    /// assert_eq!(coords.next(), Some(Coord::at(1, 0)));
    /// // bottom row
    /// assert_eq!(coords.next(), Some(Coord::at(-1, -1)));
    /// assert_eq!(coords.next(), Some(Coord::at(0, -1)));
    /// assert_eq!(coords.next(), Some(Coord::at(1, -1)));
    /// assert_eq!(coords.next(), None);
    /// ```
    pub fn chebyshev_adjacent(&self) -> impl Iterator<Item = Coord<I>> {
        let ret = [
            // top row
            self.x
                .checked_sub(I::one())
                .zip(self.y.checked_add(I::one()))
                .map(|(x, y)| Coord::at(x, y)),
            self.y.checked_add(I::one()).map(|y| Coord::at(self.x, y)),
            self.x
                .checked_add(I::one())
                .zip(self.y.checked_add(I::one()))
                .map(|(x, y)| Coord::at(x, y)),
            // middle row
            self.x.checked_sub(I::one()).map(|x| Coord::at(x, self.y)),
            self.x.checked_add(I::one()).map(|x| Coord::at(x, self.y)),
            // top bottom
            self.x
                .checked_sub(I::one())
                .zip(self.y.checked_sub(I::one()))
                .map(|(x, y)| Coord::at(x, y)),
            self.y.checked_sub(I::one()).map(|y| Coord::at(self.x, y)),
            self.x
                .checked_add(I::one())
                .zip(self.y.checked_sub(I::one()))
                .map(|(x, y)| Coord::at(x, y)),
        ];

        ret.into_iter().filter_map(|s| s)
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

impl<I> Coord<I>
where
    I: ops::Sub<Output = I>
        + ops::Add<Output = I>
        + std::hash::Hash
        + num::Zero
        + num::One
        + num::CheckedOp
        + Eq
        + Ord
        + Default
        + Copy,
{
    /// Returns a `Vec` of `Coord` at a distance of exactly `distance` from the starting point.
    ///
    /// If `self` is `S`, then, with a distance of 2, this function returns all the point in the `#` coordinates.
    /// ```text
    /// . . . . . . .
    /// . . . # . . .
    /// . . # . # . .
    /// . # . S . # .
    /// . . # . # . .
    /// . . . # . . .
    /// . . . . . . .
    /// ```
    pub fn manhattan_coords_at_distance(&self, distance: I) -> Vec<Coord<I>> {
        let mut ret = Vec::new();
        let mut explored = HashSet::new();
        let mut to_explore = vec![(self.clone(), I::zero())];

        loop {
            to_explore.sort_by(|(_, left), (_, right)| Reverse(left).cmp(&Reverse(right)));

            if let Some((current, curr_dist)) = to_explore.pop() {
                explored.insert(current);
                if curr_dist == distance {
                    ret.push(current);
                }
                to_explore.extend(
                    current
                        .manhattan_adjacent()
                        .filter(|coord| !explored.contains(coord))
                        .map(|c| (c, curr_dist + I::one()))
                        .filter(|(_, d)| *d <= distance),
                );
            } else {
                break;
            }
        }

        ret
    }

    /// Returns a `Vec` of `Coord` at a distance of exactly `distance` from the starting point.
    ///
    /// If `self` is `S`, then, with a distance of 2, this function returns all the point in the `#` coordinates.
    /// ```text
    /// . . . . . . .
    /// . # # # # # .
    /// . # . . . # .
    /// . # . . . # .
    /// . # . . . # .
    /// . # # # # # .
    /// . . . . . . .
    /// ```
    pub fn chebyshev_coords_at_distance(&self, distance: I) -> Vec<Coord<I>> {
        let mut ret = Vec::new();
        let mut explored = HashSet::new();
        let mut to_explore = vec![(self.clone(), I::zero())];

        loop {
            to_explore.sort_by(|(_, left), (_, right)| Reverse(left).cmp(&Reverse(right)));

            if let Some((current, curr_dist)) = to_explore.pop() {
                explored.insert(current);
                if curr_dist == distance {
                    ret.push(current);
                }
                to_explore.extend(
                    current
                        .chebyshev_adjacent()
                        .filter(|coord| !explored.contains(coord))
                        .map(|c| (c, curr_dist + I::one()))
                        .filter(|(_, d)| *d <= distance),
                );
            } else {
                break;
            }
        }

        ret.into_iter()
            .filter(|coord| !(self.chebyshev_distance_from(coord) != distance))
            .collect()
    }
}

impl<I> Coord<I>
where
    I: ops::Neg<Output = I> + Clone,
{
    /// Rotate the coordinate clockwise around the origin
    /// ```
    /// use aoc::Coord;
    ///
    /// let base = Coord::<isize>::at(3, 1);
    ///
    /// assert_eq!(base.rotate_clockwise(), Coord::at(-1, 3));
    /// assert_eq!(base.rotate_clockwise().rotate_clockwise(), Coord::at(-3, -1));
    /// assert_eq!(base.rotate_clockwise().rotate_clockwise().rotate_clockwise(), Coord::at(1 , -3));
    /// assert_eq!(base.rotate_clockwise().rotate_clockwise().rotate_clockwise().rotate_clockwise(), base);
    /// ```
    pub fn rotate_clockwise(&self) -> Self {
        Self::at(-self.y.clone(), self.x.clone())
    }

    /// Rotate the coordinate clockwise around the origin n times
    /// ```
    /// use aoc::Coord;
    ///
    /// let base = Coord::<isize>::at(3, 1);
    ///
    /// assert_eq!(base.rotate_clockwise_n(0), base);
    /// assert_eq!(base.rotate_clockwise_n(1), Coord::at(-1, 3));
    /// assert_eq!(base.rotate_clockwise_n(2), Coord::at(-3, -1));
    /// assert_eq!(base.rotate_clockwise_n(3), Coord::at(1, -3));
    /// assert_eq!(base.rotate_clockwise_n(4), base);
    /// ```
    pub fn rotate_clockwise_n(&self, n: usize) -> Self {
        (0..n).fold(self.clone(), |coord, _| coord.rotate_clockwise())
    }

    /// Rotate the coordinate clockwise around the origin
    /// ```
    /// use aoc::Coord;
    ///
    /// let base = Coord::<isize>::at(3, 1);
    ///
    /// assert_eq!(base.rotate_counter_clockwise(), Coord::at(1, -3));
    /// assert_eq!(base.rotate_counter_clockwise().rotate_counter_clockwise(), Coord::at(-3, -1));
    /// assert_eq!(base.rotate_counter_clockwise().rotate_counter_clockwise().rotate_counter_clockwise(), Coord::at(-1 , 3));
    /// assert_eq!(base.rotate_counter_clockwise().rotate_counter_clockwise().rotate_counter_clockwise().rotate_counter_clockwise(), base);
    /// ```
    pub fn rotate_counter_clockwise(&self) -> Self {
        Self::at(self.y.clone(), -self.x.clone())
    }

    /// Rotate the coordinate clockwise around the origin n times
    /// ```
    /// use aoc::Coord;
    ///
    /// let base = Coord::<isize>::at(3, 1);
    ///
    /// assert_eq!(base.rotate_counter_clockwise_n(0), base);
    /// assert_eq!(base.rotate_counter_clockwise_n(1), Coord::at(1, -3));
    /// assert_eq!(base.rotate_counter_clockwise_n(2), Coord::at(-3, -1));
    /// assert_eq!(base.rotate_counter_clockwise_n(3), Coord::at(-1, 3));
    /// assert_eq!(base.rotate_counter_clockwise_n(4), base);
    /// ```
    pub fn rotate_counter_clockwise_n(&self, n: usize) -> Self {
        (0..n).fold(self.clone(), |coord, _| coord.rotate_counter_clockwise())
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
    /// let base = Coord::<isize>::default();
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

impl<T> From<&(T, T)> for Coord<T>
where
    T: Clone,
{
    fn from(t: &(T, T)) -> Self {
        unsafe { std::mem::transmute::<_, &Coord<T>>(t) }.clone()
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

impl Coord<usize> {
    /// Makes a checked addition between a [Direction](crate::Direction)s and a `Coord<usize>`.
    ///
    /// ```
    /// use aoc::{Coord, Direction};
    ///
    /// let coord: Coord<usize> = Coord::default();
    ///
    /// assert_eq!(coord.checked_add(Direction::North), None);
    /// assert_eq!(coord.checked_add(Direction::West), None);
    /// assert_eq!(coord.checked_add(Direction::East), Some(Coord::at(1, 0)));
    /// assert_eq!(coord.checked_add(Direction::South), Some(Coord::at(0, 1)));
    /// assert_eq!(Coord::at(5, 0).checked_add(Direction::North), None);
    /// assert_eq!(Coord::at(0, 5).checked_add(Direction::West), None);
    /// assert_eq!(Coord::at(5, 5).checked_add(Direction::North), Some(Coord::at(5, 4)));
    /// assert_eq!(Coord::at(5, 5).checked_add(Direction::West), Some(Coord::at(4, 5)));
    /// assert_eq!(Coord::at(5, 5).checked_add(Direction::South), Some(Coord::at(5, 6)));
    /// assert_eq!(Coord::at(5, 5).checked_add(Direction::East), Some(Coord::at(6, 5)));
    /// ```
    pub fn checked_add(self, dir: direction::Direction) -> Option<Self> {
        use direction::Direction::*;

        match self {
            Self { x: 0, y: 0 } if dir == North || dir == West => None,
            Self { x: 0, .. } if dir == West => None,
            Self { y: 0, .. } if dir == North => None,
            Self { .. } => Some(self + dir),
        }
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

impl<I> std::ops::Add<crate::Movement> for Coord<I>
where
    I: num::One + ops::Add<Output = I> + ops::Sub<Output = I>,
{
    type Output = Self;

    /// Allow to add [Direction](crate::Direction)s to `Coord`.
    ///
    /// ```
    /// use aoc::{Coord, Movement};
    ///
    /// let coord = Coord::default();
    ///
    /// assert_eq!(coord + Movement::North(5), Coord::at(0, -5));
    /// assert_eq!(coord + Movement::West(3), Coord::at(-3, 0));
    /// assert_eq!(coord + Movement::East(2), Coord::at(2, 0));
    /// assert_eq!(coord + Movement::South(15), Coord::at(0, 15));
    /// ```
    ///
    /// Be cautious when using this function, it can panic if you try to convert a `Movement` that
    /// can't be converted to a `Direction`:
    /// ```should_panic
    /// use aoc::{Coord, Movement};
    /// Coord::<usize>::default() + Movement::Right(5);
    /// Coord::<usize>::default() + Movement::Left(5);
    /// Coord::<usize>::default() + Movement::Forward(5);
    /// ```
    fn add(self, dir: crate::Movement) -> Self {
        let (dir, n) = dir.to_dir_val().unwrap();

        (0..n).fold(self, |coord, _| coord + dir)
    }
}

impl<I> std::ops::Add<I> for Coord<I>
where
    I: ops::Add<Output = I> + Clone,
{
    type Output = Self;

    /// ```
    /// use aoc::{Coord, Direction};
    ///
    /// let coord = Coord::default();
    ///
    /// assert_eq!(coord + 2, Coord::at(2, 2));
    /// assert_eq!(coord + -2, Coord::at(-2, -2));
    /// ```
    fn add(self, n: I) -> Self {
        Coord::at(self.x + n.clone(), self.y + n)
    }
}

impl<I> std::ops::Sub<I> for Coord<I>
where
    I: ops::Sub<Output = I> + Clone,
{
    type Output = Self;

    /// ```
    /// use aoc::{Coord, Direction};
    ///
    /// let coord = Coord::default();
    ///
    /// assert_eq!(coord - 2, Coord::at(-2, -2));
    /// assert_eq!(coord - 0, Coord::at(0, 0));
    /// ```
    fn sub(self, n: I) -> Self {
        Coord::at(self.x - n.clone(), self.y - n)
    }
}

impl<I> std::ops::Mul<I> for Coord<I>
where
    I: ops::Mul<Output = I> + Clone,
{
    type Output = Self;

    /// ```
    /// use aoc::{Coord, Direction};
    ///
    /// let coord = Coord::at(1, 1);
    ///
    /// assert_eq!(coord * 2, Coord::at(2, 2));
    /// assert_eq!(coord * -2, Coord::at(-2, -2));
    /// ```
    fn mul(self, n: I) -> Self {
        Coord::at(self.x * n.clone(), self.y * n)
    }
}

impl<I> std::ops::Div<I> for Coord<I>
where
    I: ops::Div<Output = I> + Clone,
{
    type Output = Self;

    /// ```
    /// use aoc::{Coord, Direction};
    ///
    /// let coord = Coord::at(10, 5);
    ///
    /// assert_eq!(coord / 2, Coord::at(5, 2));
    /// assert_eq!(coord / -2, Coord::at(-5, -2));
    /// ```
    fn div(self, n: I) -> Self {
        Coord::at(self.x / n.clone(), self.y / n)
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

impl<I, T> std::ops::SubAssign<T> for Coord<I>
where
    Self: std::ops::Sub<T, Output = Self> + Clone,
{
    fn sub_assign(&mut self, other: T) {
        *self = self.clone() - other
    }
}

impl<I, T> std::ops::MulAssign<T> for Coord<I>
where
    Self: std::ops::Mul<T, Output = Self> + Clone,
{
    fn mul_assign(&mut self, other: T) {
        *self = self.clone() * other
    }
}

impl<I, T> std::ops::DivAssign<T> for Coord<I>
where
    Self: std::ops::Div<T, Output = Self> + Clone,
{
    fn div_assign(&mut self, other: T) {
        *self = self.clone() / other
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
    use crate::Grid;

    use super::*;

    fn visualize(coords: impl IntoIterator<Item = Coord<isize>>) -> String {
        let mut coords = coords.into_iter().collect::<Vec<_>>();

        let min_x = coords.iter().map(|coord| coord.x).min().unwrap();
        let min_y = coords.iter().map(|coord| coord.y).min().unwrap();

        if min_x <= 0 {
            let inc = min_x.abs();
            coords.iter_mut().for_each(|coord| coord.x += inc + 1);
        }
        if min_y <= 0 {
            let inc = min_y.abs();
            coords.iter_mut().for_each(|coord| coord.y += inc + 1);
        }
        let coords = coords
            .into_iter()
            .map(|Coord { x, y }| Coord::at(x as usize, y as usize))
            .collect::<Vec<_>>();

        let max_x = coords.iter().map(|coord| coord.x).max().unwrap();
        let max_y = coords.iter().map(|coord| coord.y).max().unwrap();

        let grid = Grid::<char>::with_dimension(max_x + 2, max_y + 2);
        let mut grid = grid.map(|_| '.');

        for coord in coords {
            grid[coord] = '#';
        }

        format!("{}", grid)
    }

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

    #[test]
    fn test_move_toward() {
        for coord in Coord::at(-1, -1).to(Coord::at(1, 1)).unwrap() {
            let mut base = Coord::default();

            base.move_toward(&coord);
            assert_eq!(base, coord);
        }

        // does it work with a more than 1 distance
        let mut coord = Coord::default();
        coord.move_toward(&Coord::at(10, 0));
        assert_eq!(coord, Coord::at(1, 0));

        let mut coord = Coord::default();
        coord.move_toward(&Coord::at(-10, 0));
        assert_eq!(coord, Coord::at(-1, 0));

        let mut coord = Coord::default();
        coord.move_toward(&Coord::at(-10, -10));
        assert_eq!(coord, Coord::at(-1, -1));

        let mut coord = Coord::default();
        coord.move_toward(&Coord::at(10, 10));
        assert_eq!(coord, Coord::at(1, 1));
    }

    #[test]
    fn test_manhattan_coords_at_distance() {
        let coord = Coord::at(0, 0);

        insta::assert_snapshot!(visualize(coord.manhattan_coords_at_distance(0)), @r###"
        . . . 
        . # . 
        . . . 
        "###);

        insta::assert_snapshot!(visualize(coord.manhattan_coords_at_distance(1)), @r###"
        . . . . . 
        . . # . . 
        . # . # . 
        . . # . . 
        . . . . . 
        "###);

        insta::assert_snapshot!(visualize(coord.manhattan_coords_at_distance(2)), @r###"
        . . . . . . . 
        . . . # . . . 
        . . # . # . . 
        . # . . . # . 
        . . # . # . . 
        . . . # . . . 
        . . . . . . . 
        "###);

        insta::assert_snapshot!(visualize(coord.manhattan_coords_at_distance(3)), @r###"
        . . . . . . . . . 
        . . . . # . . . . 
        . . . # . # . . . 
        . . # . . . # . . 
        . # . . . . . # . 
        . . # . . . # . . 
        . . . # . # . . . 
        . . . . # . . . . 
        . . . . . . . . . 
        "###);
    }

    #[test]
    fn test_chebyshev_coords_at_distance() {
        let coord = Coord::at(0, 0);

        insta::assert_snapshot!(visualize(coord.chebyshev_coords_at_distance(0)), @r###"
        . . . 
        . # . 
        . . . 
        "###);

        insta::assert_snapshot!(visualize(coord.chebyshev_coords_at_distance(1)), @r###"
        . . . . . 
        . # # # . 
        . # . # . 
        . # # # . 
        . . . . . 
        "###);

        insta::assert_snapshot!(visualize(coord.chebyshev_coords_at_distance(2)), @r###"
        . . . . . . . 
        . # # # # # . 
        . # . . . # . 
        . # . . . # . 
        . # . . . # . 
        . # # # # # . 
        . . . . . . . 
        "###);

        insta::assert_snapshot!(visualize(coord.chebyshev_coords_at_distance(3)), @r###"
        . . . . . . . . . 
        . # # # # # # # . 
        . # . . . . . # . 
        . # . . . . . # . 
        . # . . . . . # . 
        . # . . . . . # . 
        . # . . . . . # . 
        . # # # # # # # . 
        . . . . . . . . . 
        "###);
    }
}
