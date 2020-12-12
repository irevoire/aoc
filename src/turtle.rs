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
        match (other, self.facing) {
            (Movement::North(n), _) => self.coord.y -= n,
            (Movement::West(n), _) => self.coord.x -= n,
            (Movement::East(n), _) => self.coord.x += n,
            (Movement::South(n), _) => self.coord.y += n,
            (Movement::Right(n), Direction::North)
            | (Movement::Left(n), Direction::South)
            | (Movement::Forward(n), Direction::East) => {
                self.facing = Direction::East;
                self.coord.x += n;
            }
            (Movement::Left(n), Direction::North)
            | (Movement::Right(n), Direction::South)
            | (Movement::Forward(n), Direction::West) => {
                self.facing = Direction::West;
                self.coord.x -= n;
            }
            (Movement::Left(n), Direction::East)
            | (Movement::Right(n), Direction::West)
            | (Movement::Forward(n), Direction::North) => {
                self.facing = Direction::North;
                self.coord.y += n;
            }
            (Movement::Left(n), Direction::West)
            | (Movement::Right(n), Direction::East)
            | (Movement::Forward(n), Direction::South) => {
                self.facing = Direction::South;
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
