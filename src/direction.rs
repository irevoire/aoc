use anyhow::{bail, Error, Result};
use std::str::FromStr;

/// Represent a direction.
/// Be cautious with this type because a lot of variant are equivalent:
/// * Left = West
/// * Right = East
/// * Up = Top = North
/// * Down = Bottom = South
#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    North,
    West,
    East,
    South,
}

impl Default for Direction {
    fn default() -> Self {
        Self::North
    }
}

impl FromStr for Direction {
    type Err = Error;

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
