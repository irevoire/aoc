use anyhow::{bail, Error, Result};
use std::str::FromStr;

/// Represent a direction.
/// Be cautious with this type because a lot of variant are equivalent:
/// * Left = West
/// * Right = East
/// * Up = Top = North
/// * Down = Bottom = South
#[derive(Debug, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Top,
    Bottom,

    West,
    East,
    North,
    South,
}

impl Default for Direction {
    fn default() -> Self {
        Self::Up
    }
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s.to_lowercase().trim() {
            "^" | "u" | "n" | "up" | "north" | "top" => Self::Up,
            ">" | "r" | "e" | "right" | "east" => Self::Right,
            "v" | "d" | "s" | "down" | "south" | "bottom" => Self::Down,
            "<" | "l" | "w" | "left" | "west" => Self::Left,
            s => bail!("can’t convert {} as a direction", s),
        })
    }
}
