use anyhow::{Error, Result};
use std::str::FromStr;

/// Describe a movement in one direction with a certain length
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
    North(isize),
    East(isize),
    West(isize),
    South(isize),
    Right(isize),
    Left(isize),
    Forward(isize),
}
use Movement::*;

impl Movement {
    /// Increment the number of deplacement by one
    pub fn incr(self) -> Self {
        match self {
            North(n) => North(n + 1),
            East(n) => East(n + 1),
            West(n) => West(n + 1),
            South(n) => South(n + 1),
            Right(n) => Right(n + 1),
            Left(n) => Left(n + 1),
            Forward(n) => Forward(n + 1),
        }
    }

    /// Decrement the number of deplacement by one
    pub fn decr(self) -> Self {
        match self {
            North(n) => North(n - 1),
            East(n) => East(n - 1),
            West(n) => West(n - 1),
            South(n) => South(n - 1),
            Right(n) => Right(n - 1),
            Left(n) => Left(n - 1),
            Forward(n) => Forward(n - 1),
        }
    }

    /// Extract the number of deplacement
    pub fn value(self) -> isize {
        match self {
            North(n) => n,
            East(n) => n,
            West(n) => n,
            South(n) => n,
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
            North(n) if n > 0 => North(1),
            North(n) if n < 0 => North(-1),
            East(n) if n > 0 => East(1),
            East(n) if n < 0 => East(-1),
            West(n) if n > 0 => West(1),
            West(n) if n < 0 => West(-1),
            South(n) if n > 0 => South(1),
            South(n) if n < 0 => South(-1),
            Right(n) if n > 0 => Right(1),
            Right(n) if n < 0 => Right(-1),
            Left(n) if n > 0 => Left(1),
            Left(n) if n < 0 => Left(-1),
            Forward(n) if n > 0 => Forward(1),
            Forward(n) if n < 0 => Forward(-1),
            el @ North(0)
            | el @ East(0)
            | el @ West(0)
            | el @ South(0)
            | el @ Right(0)
            | el @ Left(0)
            | el @ Forward(0) => el,
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
            "N" => North(right.parse::<isize>()?),
            "W" => West(right.parse::<isize>()?),
            "E" => East(right.parse::<isize>()?),
            "S" => South(right.parse::<isize>()?),
            "F" => Forward(right.parse::<isize>()?),
            "R" => Right(right.parse::<isize>()?),
            "L" => Left(right.parse::<isize>()?),
            _ => panic!("unexpected string"),
        })
    }
}
