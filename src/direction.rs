//! Enum to represent a direction on a grid

use anyhow::{bail, Error, Result};
use std::ops::Add;
use std::str::FromStr;

/// Represent a direction.
#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    North,
    West,
    East,
    South,
}

/// The default direction is the `North`
impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

use std::convert::TryFrom;

use crate::Movement;

impl Add<isize> for Direction {
    type Output = Movement;

    fn add(self, n: isize) -> Self::Output {
        match self {
            Direction::North => Movement::North(n),
            Direction::West => Movement::West(n),
            Direction::East => Movement::East(n),
            Direction::South => Movement::South(n),
        }
    }
}

impl TryFrom<Movement> for Direction {
    type Error = anyhow::Error;

    /// Generate a `Direction` from a [`Movement`](crate::Movement).
    /// The movement needs to be either:
    /// - `North`
    /// - `East`
    /// - `South`
    /// - `West`
    /// An error will be throw if it's either of the following variation:
    /// - `Forward`
    /// - `Right`
    /// - `Left`
    ///
    /// You'll probably want to call `TryInto` instead of `TryFrom` in your code
    /// ```
    /// use std::convert::TryFrom;
    /// use aoc::{Direction, Movement};
    ///
    /// assert_eq!(Direction::try_from(Movement::North(15)).unwrap(), Direction::North);
    /// assert_eq!(Direction::try_from(Movement::West(-5)).unwrap(), Direction::West);
    /// assert_eq!(Direction::try_from(Movement::East(0)).unwrap(), Direction::East);
    /// assert_eq!(Direction::try_from(Movement::South(53)).unwrap(), Direction::South);
    ///
    /// assert!(Direction::try_from(Movement::Forward(5)).is_err());
    /// assert!(Direction::try_from(Movement::Left(50)).is_err());
    /// assert!(Direction::try_from(Movement::Right(0)).is_err());
    /// ```
    fn try_from(movement: Movement) -> Result<Self, Self::Error> {
        Ok(match movement {
            Movement::North(_) => Direction::North,
            Movement::East(_) => Direction::East,
            Movement::South(_) => Direction::South,
            Movement::West(_) => Direction::West,
            m => anyhow::bail!("Can't convert {:?} into a Direction", m),
        })
    }
}

impl FromStr for Direction {
    type Err = Error;

    /// Generate a `Direction` from a string. The following string are accepted for each
    /// directions:
    /// - `North`: "^" | "u" | "n" | "up" | "north" | "top"
    /// - `East`: ">" | "r" | "e" | "right" | "east"
    /// - `South`: "v" | "d" | "s" | "down" | "south" | "bottom"
    /// - `West`: "<" | "l" | "w" | "left" | "west"
    /// ```
    /// use aoc::Direction;
    ///
    /// assert_eq!("^".parse::<Direction>().unwrap(), Direction::North);
    /// assert_eq!("<".parse::<Direction>().unwrap(), Direction::West);
    /// assert_eq!("right".parse::<Direction>().unwrap(), Direction::East);
    /// assert_eq!("S".parse::<Direction>().unwrap(), Direction::South);
    /// ```
    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.to_lowercase().trim() {
            "^" | "u" | "n" | "up" | "north" | "top" => Self::North,
            ">" | "r" | "e" | "right" | "east" => Self::East,
            "v" | "d" | "s" | "down" | "south" | "bottom" => Self::South,
            "<" | "l" | "w" | "left" | "west" => Self::West,
            s => bail!("can’t convert {} as a direction", s),
        })
    }
}
