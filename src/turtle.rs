use crate::{Coord, Direction, Movement};
use std::ops::{Add, AddAssign};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Turtle {
    pub coord: Coord<isize>,
    pub facing: Direction,
}

impl Turtle {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from(coord: Coord<isize>, facing: Direction) -> Self {
        Self { coord, facing }
    }

    pub fn distance_from(&self, coord: &Coord<isize>) -> isize {
        self.coord.distance_from(coord)
    }

    pub fn distance_from_base(&self) -> isize {
        self.coord.distance_from_base()
    }
}

impl Add<Movement> for Turtle {
    type Output = Self;

    fn add(mut self, other: Movement) -> Self {
        use Direction::*;
        use Movement::*;

        match (other, self.facing) {
            (Right(n), North) | (Left(n), South) | (Forward(n), East) => {
                self.facing = East;
                self.coord.x += n;
            }
            (Left(n), North) | (Right(n), South) | (Forward(n), West) => {
                self.facing = West;
                self.coord.x -= n;
            }
            (Left(n), East) | (Right(n), West) | (Forward(n), North) => {
                self.facing = North;
                self.coord.y += n;
            }
            (Left(n), West) | (Right(n), East) | (Forward(n), South) => {
                self.facing = South;
                self.coord.y -= n;
            }
        }
        self
    }
}

impl AddAssign<Movement> for Turtle {
    fn add_assign(&mut self, other: Movement) {
        *self = self.clone() + other;
    }
}
