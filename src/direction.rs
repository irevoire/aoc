//! Enum to represent a direction on a grid

use anyhow::{bail, Error, Result};
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

impl FromStr for Direction {
    type Err = Error;

    /// Generate a `Direction` from a string. The following string are accepted for each
    /// directions:
    /// - `North`: "^" | "u" | "n" | "up" | "north" | "top"
    /// - `East`: ">" | "r" | "e" | "right" | "east"
    /// - `South`: "v" | "d" | "s" | "down" | "south" | "bottom"
    /// - `West`: "<" | "l" | "w" | "left" | "west"
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
