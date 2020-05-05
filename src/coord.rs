use crate::{direction, num, range};
use anyhow::Result;
use std::str::FromStr;
use std::{cmp, fmt, ops};

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coord<I> {
    pub x: I,
    pub y: I,
}

impl<I: Default> Coord<I> {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<I> Coord<I> {
    pub fn at(x: I, y: I) -> Self {
        Self { x, y }
    }
}

impl<I> Coord<I>
where
    I: ops::Sub<Output = I> + ops::Add<Output = I> + Ord + Copy,
{
    pub fn distance_from(&self, other: &Self) -> I {
        let x = if self.x < other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let y = if self.y < other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        x + y
    }
}

impl<I> Coord<I>
where
    I: ops::Add<Output = I> + Copy,
{
    pub fn distance_from_base(&self) -> I {
        self.x + self.y
    }
}

impl<I: Ord + Clone + fmt::Debug> Coord<I> {
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

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<I> std::ops::Add<direction::Direction> for Coord<I>
where
    I: num::One + ops::Add<Output = I> + ops::Sub<Output = I>,
{
    type Output = Self;

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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();

        let x = coords[0].parse::<I>()?;
        let y = coords[1].parse::<I>()?;

        Ok(Coord { x, y })
    }
}
