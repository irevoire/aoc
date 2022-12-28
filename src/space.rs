use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::{shape::Shape, Coord};

type Id = usize;

#[derive(Default, Debug)]
pub struct Space {
    shapes: Vec<Option<Shape>>,
}

impl Space {
    pub fn new() -> Space {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.shapes.iter().all(|shape| shape.is_none())
    }

    pub fn push(&mut self, shape: Shape) -> Id {
        if let Some((id, cell)) = self
            .shapes
            .iter_mut()
            .enumerate()
            .find(|(_id, shape)| shape.is_none())
        {
            *cell = Some(shape);
            id
        } else {
            let id = self.shapes.len();
            self.shapes.push(Some(shape));
            id
        }
    }

    pub fn min_x(&self) -> Option<isize> {
        self.shapes
            .iter()
            .filter_map(|shape| shape.as_ref())
            .filter_map(|shape| shape.min_x())
            .min()
    }

    pub fn min_y(&self) -> Option<isize> {
        self.shapes
            .iter()
            .filter_map(|shape| shape.as_ref())
            .filter_map(|shape| shape.min_y())
            .min()
    }

    pub fn min(&self) -> Option<Coord<isize>> {
        self.min_x().zip(self.min_y()).map(|(x, y)| Coord::at(x, y))
    }

    pub fn max_x(&self) -> Option<isize> {
        self.shapes
            .iter()
            .filter_map(|shape| shape.as_ref())
            .filter_map(|shape| shape.max_x())
            .max()
    }

    pub fn max_y(&self) -> Option<isize> {
        self.shapes
            .iter()
            .filter_map(|shape| shape.as_ref())
            .filter_map(|shape| shape.max_y())
            .max()
    }

    pub fn max(&self) -> Option<Coord<isize>> {
        self.max_x().zip(self.max_y()).map(|(x, y)| Coord::at(x, y))
    }
}

impl Index<&Id> for Space {
    type Output = Shape;

    fn index(&self, index: &Id) -> &Self::Output {
        self.shapes[*index].as_ref().unwrap()
    }
}

impl IndexMut<&Id> for Space {
    fn index_mut(&mut self, index: &Id) -> &mut Self::Output {
        self.shapes[*index].as_mut().unwrap()
    }
}

impl Display for Space {
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
                if let Some(shape) = self
                    .shapes
                    .iter()
                    .filter_map(|shape| shape.as_ref())
                    .find(|shape| shape.contains(Coord::at(x, y)))
                {
                    write!(f, "{} ", shape.displayed_as)?;
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
        let mut space = Space::new();
        let shape = Shape::from_coords([(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)]);
        space.push(shape);
        insta::assert_display_snapshot!(space, @r###"
          -1 0 1
        -1 . # . 
        0  # # # 
        1  . # . 
        "###);
        let mut shape = Shape::from_coords([(-1, -1), (1, -1), (-1, 1), (1, 1)]);
        shape.displayed_as('o');
        space.push(shape);
        insta::assert_display_snapshot!(space, @r###"
          -1 0 1
        -1 o # o 
        0  # # # 
        1  o # o 
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
}
