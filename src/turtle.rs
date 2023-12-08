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
        self.coord.manhattan_distance_from(coord)
    }

    pub fn distance_from_base(&self) -> isize {
        self.coord.manhattan_distance_from_base()
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
            (Movement::Right(n), Direction::North | Direction::Up)
            | (Movement::Left(n), Direction::South | Direction::Down)
            | (Movement::Forward(n), Direction::East | Direction::Right) => {
                self.facing = Direction::East;
                self.coord.x += n;
            }
            (Movement::Left(n), Direction::North | Direction::Up)
            | (Movement::Right(n), Direction::South | Direction::Down)
            | (Movement::Forward(n), Direction::West | Direction::Left) => {
                self.facing = Direction::West;
                self.coord.x -= n;
            }
            (Movement::Left(n), Direction::East | Direction::Right)
            | (Movement::Right(n), Direction::West | Direction::Left)
            | (Movement::Forward(n), Direction::North | Direction::Up) => {
                self.facing = Direction::North;
                self.coord.y -= n;
            }
            (Movement::Left(n), Direction::West | Direction::Left)
            | (Movement::Right(n), Direction::East | Direction::Right)
            | (Movement::Forward(n), Direction::South | Direction::Down) => {
                self.facing = Direction::South;
                self.coord.y += n;
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
