use anyhow::{Error, Result};
use std::str::FromStr;

/// Describe a movement in one direction with a certain length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    Right(isize),
    Left(isize),
    Forward(isize),
}
use Movement::*;

impl Movement {
    /// Increment the number of deplacement by one
    pub fn incr(self) -> Self {
        match self {
            Right(n) => Right(n + 1),
            Left(n) => Left(n + 1),
            Forward(n) => Forward(n + 1),
        }
    }

    /// Decrement the number of deplacement by one
    pub fn decr(self) -> Self {
        match self {
            Right(n) => Right(n - 1),
            Left(n) => Left(n - 1),
            Forward(n) => Forward(n - 1),
        }
    }

    /// Extract the number of deplacement
    pub fn value(self) -> isize {
        match self {
            Right(n) => n,
            Left(n) => n,
            Forward(n) => n,
        }
    }

    /// Get the direction with a deplacement of 1, 0 or -1 depending of the sign of the deplacement
    /// ```
    /// use aoc::Movement;
    ///
    /// assert_eq!(Movement::Right(42).unit(), Movement::Right(1));
    /// assert_eq!(Movement::Forward(0).unit(), Movement::Forward(0));
    /// ```
    pub fn unit(self) -> Self {
        match self {
            Right(n) if n > 0 => Right(1),
            Right(n) if n < 0 => Right(-1),
            Left(n) if n > 0 => Left(1),
            Left(n) if n < 0 => Left(-1),
            Forward(n) if n > 0 => Forward(1),
            Forward(n) if n < 0 => Forward(-1),
            el @ Right(0) | el @ Left(0) | el @ Forward(0) => el,
            el => panic!("This should not happens: {:?}", el),
        }
    }

    /// Generate an iterator from one movement by exploding the Movement(n) into Movement(1)
    /// followed by n times Forward(1)
    ///
    /// ```
    /// use aoc::Movement;
    ///
    /// assert_eq!(Movement::Right(2).explode().collect::<Vec<_>>(), &[Movement::Right(1), Movement::Forward(1)]);
    /// assert_eq!(Movement::Right(0).explode().collect::<Vec<_>>(), &[Movement::Right(0)]);
    /// ```
    pub fn explode(self) -> impl Iterator<Item = Self> {
        std::iter::once(self.unit()).chain(
            std::iter::repeat(Forward(1)).take((self.value().abs() as usize).saturating_sub(1)),
        )
    }
}

impl FromStr for Movement {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (left, right) = s.split_at(1);
        Ok(match left {
            "R" => Right(right.parse::<isize>()?),
            "L" => Left(right.parse::<isize>()?),
            "F" => Forward(right.parse::<isize>()?),
            _ => panic!("unexpected string"),
        })
    }
}
