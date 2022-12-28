use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

use crate::Coord;

#[derive(Debug)]
pub struct Shape {
    pub displayed_as: char,
    coords: HashSet<Coord<isize>>,
}

impl Default for Shape {
    fn default() -> Self {
        Self::new()
    }
}

impl Shape {
    pub fn new() -> Shape {
        Shape {
            displayed_as: '#',
            coords: HashSet::new(),
        }
    }

    pub fn from_coords(points: impl IntoIterator<Item = impl Into<Coord<isize>>>) -> Shape {
        Shape {
            coords: points.into_iter().map(|coord| coord.into()).collect(),
            ..Default::default()
        }
    }

    pub fn displayed_as(&mut self, displayed_as: char) {
        self.displayed_as = displayed_as;
    }

    pub fn push(&mut self, point: impl Into<Coord<isize>>) -> bool {
        self.coords.insert(point.into())
    }

    pub fn contains(&self, point: Coord<isize>) -> bool {
        self.coords.contains(&point)
    }

    pub fn is_empty(&self) -> bool {
        self.coords.is_empty()
    }

    pub fn collides_with(&self, other: &Shape) -> bool {
        !(self & other).is_empty()
    }

    pub fn min_x(&self) -> Option<isize> {
        self.coords.iter().map(|coord| coord.x).min()
    }

    pub fn min_y(&self) -> Option<isize> {
        self.coords.iter().map(|coord| coord.y).min()
    }

    pub fn min(&self) -> Option<Coord<isize>> {
        self.min_x().zip(self.min_y()).map(|(x, y)| Coord::at(x, y))
    }

    pub fn max_x(&self) -> Option<isize> {
        self.coords.iter().map(|coord| coord.x).max()
    }

    pub fn max_y(&self) -> Option<isize> {
        self.coords.iter().map(|coord| coord.y).max()
    }

    pub fn max(&self) -> Option<Coord<isize>> {
        self.max_x().zip(self.max_y()).map(|(x, y)| Coord::at(x, y))
    }
}

impl BitOr for &Shape {
    type Output = Shape;

    fn bitor(self, rhs: Self) -> Self::Output {
        Shape {
            coords: self.coords.union(&rhs.coords).copied().collect(),
            displayed_as: self.displayed_as,
        }
    }
}

impl BitOr for Shape {
    type Output = Shape;

    fn bitor(self, rhs: Self) -> Self::Output {
        Shape {
            coords: self.coords.union(&rhs.coords).copied().collect(),
            displayed_as: self.displayed_as,
        }
    }
}

impl BitOrAssign for Shape {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = std::mem::take(self) | rhs;
    }
}

impl BitAnd for &Shape {
    type Output = Shape;

    fn bitand(self, rhs: Self) -> Self::Output {
        Shape {
            coords: self.coords.intersection(&rhs.coords).copied().collect(),
            displayed_as: self.displayed_as,
        }
    }
}

impl BitAnd for Shape {
    type Output = Shape;

    fn bitand(self, rhs: Self) -> Self::Output {
        Shape {
            coords: self.coords.intersection(&rhs.coords).copied().collect(),
            displayed_as: self.displayed_as,
        }
    }
}

impl BitAndAssign for Shape {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = std::mem::take(self) | rhs;
    }
}

impl Add<Coord<isize>> for Shape {
    type Output = Shape;

    fn add(self, rhs: Coord<isize>) -> Self::Output {
        Shape {
            coords: self.coords.into_iter().map(|coord| coord + rhs).collect(),
            displayed_as: self.displayed_as,
        }
    }
}

impl AddAssign<Coord<isize>> for Shape {
    fn add_assign(&mut self, rhs: Coord<isize>) {
        *self = std::mem::take(self) + rhs;
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return Ok(());
        }
        let min = self.min().unwrap();
        let max = self.max().unwrap();

        write!(f, "  ")?;
        (min.x..=max.x).try_for_each(|x| {
            if x.is_negative() && x % 10 == 0 {
                write!(f, "-0")
            } else {
                write!(f, "{:>2}", x % 10)
            }
        })?;
        writeln!(f)?;

        for y in min.y..=max.y {
            write!(f, "{:<2} ", y % 10)?;
            for x in min.x..=max.x {
                if self.contains(Coord::at(x, y)) {
                    write!(f, "{} ", self.displayed_as)?;
                } else {
                    write!(f, ". ")?;
                }
            }

            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let shape = Shape::new();
        insta::assert_display_snapshot!(shape, @"");

        let mut shape = Shape::from_coords([(0, 0)]);
        insta::assert_display_snapshot!(shape, @r###"
           0
        0  # 
        "###);
        shape.displayed_as('o');
        insta::assert_display_snapshot!(shape, @r###"
           0
        0  o 
        "###);

        let shape = Shape::from_coords([(0, 0), (2, 0)]);
        insta::assert_display_snapshot!(shape, @r###"
           0 1 2
        0  # . # 
        "###);

        let shape = Shape::from_coords([(0, 0), (0, 2)]);
        insta::assert_display_snapshot!(shape, @r###"
           0
        0  # 
        1  . 
        2  # 
        "###);

        let shape = Shape::from_coords([(0, 0), (2, 2)]);
        insta::assert_display_snapshot!(shape, @r###"
           0 1 2
        0  # . . 
        1  . . . 
        2  . . # 
        "###);

        let shape = Shape::from_coords([(2, 2), (4, 4)]);
        insta::assert_display_snapshot!(shape, @r###"
           2 3 4
        2  # . . 
        3  . . . 
        4  . . # 
        "###);

        let shape = Shape::from_coords([(-2, -2), (4, 4)]);
        insta::assert_display_snapshot!(shape, @r###"
          -2-1 0 1 2 3 4
        -2 # . . . . . . 
        -1 . . . . . . . 
        0  . . . . . . . 
        1  . . . . . . . 
        2  . . . . . . . 
        3  . . . . . . . 
        4  . . . . . . # 
        "###);

        let shape = Shape::from_coords([(-22, 0), (22, 0)]);
        insta::assert_display_snapshot!(shape, @r###"
          -2-1-0-9-8-7-6-5-4-3-2-1-0-9-8-7-6-5-4-3-2-1 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2
        0  # . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . # 
        "###);
    }

    #[test]
    fn translate() {
        let shape = Shape::from_coords([(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)]);
        insta::assert_display_snapshot!(shape, @r###"
          -1 0 1
        -1 . # . 
        0  # # # 
        1  . # . 
        "###);
        let mut shape = shape + Coord::at(5, 5);
        insta::assert_display_snapshot!(shape, @r###"
           4 5 6
        4  . # . 
        5  # # # 
        6  . # . 
        "###);
        shape += Coord::at(-5, -5);
        insta::assert_display_snapshot!(shape, @r###"
          -1 0 1
        -1 . # . 
        0  # # # 
        1  . # . 
        "###);
    }

    #[test]
    fn merge_shape() {
        let top_left = Shape::from_coords([(0, -1), (-1, 0)]);
        let bottom_right = Shape::from_coords([(0, 1), (1, 0)]);
        let middle = Shape::from_coords([(0, 0)]);
        let final_shape = top_left;
        insta::assert_display_snapshot!(final_shape, @r###"
          -1 0
        -1 . # 
        0  # . 
        "###);
        let mut final_shape = final_shape | bottom_right;
        insta::assert_display_snapshot!(final_shape, @r###"
          -1 0 1
        -1 . # . 
        0  # . # 
        1  . # . 
        "###);
        final_shape |= middle;
        insta::assert_display_snapshot!(final_shape, @r###"
          -1 0 1
        -1 . # . 
        0  # # # 
        1  . # . 
        "###);
    }

    #[test]
    fn collide_with() {
        let top_left = Shape::from_coords([(0, -1), (-1, 0)]);
        let bottom_right = Shape::from_coords([(0, 1), (1, 0)]);
        let middle = Shape::from_coords([(0, 0)]);

        assert!(!top_left.collides_with(&bottom_right));
        assert!(!bottom_right.collides_with(&top_left));
        let shape = &top_left | &bottom_right;
        assert!(shape.collides_with(&top_left));
        assert!(shape.collides_with(&bottom_right));
        assert!(!shape.collides_with(&middle));
    }
}
