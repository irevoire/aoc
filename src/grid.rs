//! Define a [Grid] and all kind of operations on it.

use std::fmt::Display;

use crate::Coord;
use anyhow::Result;

/// A 2D [Grid] with a lot of fancy methods on it.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Create an empty [Grid].
    ///
    /// See also [Grid::from, Grid::with_capacity].
    /// # Example
    ///
    /// ```
    /// use aoc::Grid;
    /// let mut grid: Grid<usize> = Grid::new();
    /// assert!(grid.into_inner().is_empty());
    /// ```
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Create a [Grid] from a [Vec] of [Vec].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec![1, 2, 3, 4],
    ///     vec![5, 6, 7, 8],
    ///     vec![8, 7, 6, 5],
    ///     vec![4, 3, 2, 1],
    ///    ]);
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![1, 2, 3, 4],
    ///         vec![5, 6, 7, 8],
    ///         vec![8, 7, 6, 5],
    ///         vec![4, 3, 2, 1],
    ///     ],
    /// );
    /// ```
    pub fn from(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    /// Return the inner `Vec<Vec<_>>` of the [Grid].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec![1, 2, 3, 4],
    ///     vec![5, 6, 7, 8],
    ///     vec![8, 7, 6, 5],
    ///     vec![4, 3, 2, 1],
    ///    ]);
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![1, 2, 3, 4],
    ///         vec![5, 6, 7, 8],
    ///         vec![8, 7, 6, 5],
    ///         vec![4, 3, 2, 1],
    ///     ],
    /// );
    /// ```
    pub fn into_inner(self) -> Vec<Vec<T>> {
        self.data
    }

    /// Return the width of the [Grid].
    ///
    /// See also [Grid::height].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec![1, 2, 3, 4],
    ///     vec![5, 6, 7, 8],
    /// ]);
    /// assert_eq!(grid.width(), 4);
    /// ```
    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    /// Return the height of the [Grid].
    ///
    /// See also [Grid::width].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec![1, 2, 3, 4],
    ///     vec![5, 6, 7, 8],
    /// ]);
    /// assert_eq!(grid.height(), 2);
    /// ```
    pub fn height(&self) -> usize {
        self.data.len()
    }

    /// Return an [Iterator] on all the elements of the [Grid].
    ///
    /// See also [Grid::iter_mut], [Grid::enumerate] and [Grid::lines].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    /// let mut iter = grid.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|sub| sub.iter())
    }

    /// Return a mutable [Iterator] on all the elements of the [Grid].
    ///
    /// See also [Grid::iter], [Grid::enumerate_mut] and [Grid::lines_mut].
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    /// grid.iter_mut().for_each(|el| *el *= 2);
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![2, 4],
    ///         vec![6, 8],
    ///     ],
    /// );
    /// ```
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().flat_map(|sub| sub.iter_mut())
    }

    /// Creates an [Iterator] which gives the current iteration [Coord]inates as well as the next value.
    /// The iterator returned yields pairs `(coord, val)`, where `coord` is the current [Coord] of
    /// iteration and `val` is the value returned by the [Iterator].
    ///
    /// See also [Grid::enumerate_mut].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    /// let mut iter = grid.enumerate();
    /// assert_eq!(iter.next(), Some((aoc::Coord::at(0, 0), &1)));
    /// assert_eq!(iter.next(), Some((aoc::Coord::at(1, 0), &2)));
    /// assert_eq!(iter.next(), Some((aoc::Coord::at(0, 1), &3)));
    /// assert_eq!(iter.next(), Some((aoc::Coord::at(1, 1), &4)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn enumerate(&self) -> impl Iterator<Item = (Coord<usize>, &T)> {
        self.lines().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, el)| (Coord::at(x, y), el))
        })
    }

    /// Creates a mutable [Iterator] which gives the current iteration [Coord]inates as well as the next value.
    /// The iterator returned yields pairs `(coord, val)`, where `coord` is the current [Coord] of
    /// iteration and `val` is the value returned by the [Iterator].
    ///
    /// See also [Grid::enumerate].
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    /// grid.enumerate_mut().for_each(|(c, v)| *v = c.y);
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![0, 0],
    ///         vec![1, 1],
    ///     ],
    /// );
    /// ```
    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Coord<usize>, &mut T)> {
        self.lines_mut().enumerate().flat_map(|(y, line)| {
            line.iter_mut()
                .enumerate()
                .map(move |(x, el)| (Coord::at(x, y), el))
        })
    }

    /// Return an [Iterator] of all the lines of the [Grid].
    ///
    /// See also [Grid::rlines] and [Grid::lines_mut].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    /// let mut iter = grid.lines();
    /// assert_eq!(iter.next(), Some([1, 2].as_slice()));
    /// assert_eq!(iter.next(), Some([3, 4].as_slice()));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn lines(&self) -> impl Iterator<Item = &[T]> {
        self.data.iter().map(|v| v.as_slice())
    }

    /// Return a mutable [Iterator] on all the lines of the [Grid].
    ///
    /// See also [Grid::rlines_mut] and [Grid::lines].
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    /// grid.lines_mut().enumerate().for_each(|(i, line)| line.push(3 + i * 2));
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![1, 2, 3],
    ///         vec![3, 4, 5],
    ///     ],
    /// );
    /// ```
    pub fn lines_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
        self.data.iter_mut()
    }

    /// Return an [Iterator] on all the lines of the [Grid] from the bottom to the top.
    ///
    /// See also [Grid::lines] and [Grid::rlines_mut].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    /// let mut iter = grid.rlines();
    /// assert_eq!(iter.next(), Some([3, 4].as_slice()));
    /// assert_eq!(iter.next(), Some([1, 2].as_slice()));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn rlines(&self) -> impl Iterator<Item = &[T]> {
        self.data.iter().rev().map(|v| v.as_slice())
    }

    /// Return a mutable iterator on all the lines of the grid from the bottom to the top
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![1, 2],
    ///     vec![3, 4],
    /// ]);
    /// grid.rlines_mut().enumerate().for_each(|(i, line)| line.push(3 + i * 2));
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![1, 2, 5],
    ///         vec![3, 4, 3],
    ///     ],
    /// );
    /// ```
    pub fn rlines_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
        self.data.iter_mut().rev()
    }

    /// Return an [Iterator] of all the element in the [Grid] from one [Coord] to another.
    /// Can return an error if the starting position is situated *AFTER* the end position. See [Coord::to].
    ///
    /// See also [Grid::through_mut].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec!['a', 'b', 'c', 'd', 'e'],
    ///     vec!['f', 'g', 'h', 'i', 'j'],
    ///     vec!['k', 'l', 'm', 'n', 'o'],
    ///     vec!['p', 'q', 'r', 's', 't'],
    ///     vec!['u', 'v', 'w', 'x', 'y'],
    /// ]);
    /// let mut iter = grid.through(aoc::Coord::at(1, 1), aoc::Coord::at(3, 3)).unwrap();
    /// assert_eq!(iter.next(), Some(&'g'));
    /// assert_eq!(iter.next(), Some(&'h'));
    /// assert_eq!(iter.next(), Some(&'i'));
    /// assert_eq!(iter.next(), Some(&'l'));
    /// assert_eq!(iter.next(), Some(&'m'));
    /// assert_eq!(iter.next(), Some(&'n'));
    /// assert_eq!(iter.next(), Some(&'q'));
    /// assert_eq!(iter.next(), Some(&'r'));
    /// assert_eq!(iter.next(), Some(&'s'));
    /// assert_eq!(iter.next(), None);
    ///
    /// let result = grid.through(aoc::Coord::at(2, 2), aoc::Coord::at(1, 1));
    /// assert!(result.is_err());
    /// ```
    pub fn through(
        &self,
        from: Coord<usize>,
        to: Coord<usize>,
    ) -> Result<impl Iterator<Item = &T>> {
        Ok(from.to(to)?.map(move |coord| &self[coord]))
    }

    /// Return a mutable [Iterator] of all the elements in the grid from one [Coord] to another.
    /// Can return an error if the starting position is situated *AFTER* the end position. See [Coord::to].
    ///
    /// See also [Grid::through].
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec!['a', 'b', 'c', 'd', 'e'],
    ///     vec!['f', 'g', 'h', 'i', 'j'],
    ///     vec!['k', 'l', 'm', 'n', 'o'],
    ///     vec!['p', 'q', 'r', 's', 't'],
    ///     vec!['u', 'v', 'w', 'x', 'y'],
    /// ]);
    /// grid.through_mut(aoc::Coord::at(1, 1), aoc::Coord::at(3, 3)).unwrap().for_each(|el| *el = 'z');
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec!['a', 'b', 'c', 'd', 'e'],
    ///         vec!['f', 'z', 'z', 'z', 'j'],
    ///         vec!['k', 'z', 'z', 'z', 'o'],
    ///         vec!['p', 'z', 'z', 'z', 't'],
    ///         vec!['u', 'v', 'w', 'x', 'y'],
    ///     ],
    /// );
    /// ```
    pub fn through_mut(
        &mut self,
        from: Coord<usize>,
        to: Coord<usize>,
    ) -> Result<impl Iterator<Item = &mut T>> {
        Ok(from
            .to(to)?
            .map(move |coord| unsafe { std::mem::transmute(&mut self[coord]) }))
    }

    /// Return an [Iterator] over the borders of the [Grid].
    /// # Example
    ///
    /// ```
    /// let grid = aoc::Grid::from(vec![
    ///     vec![1,  2,  3],
    ///     vec![7,  0,  8],
    ///     vec![4,  5,  6],
    /// ]);
    ///
    /// let mut iter = grid.borders();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), Some(&4));
    /// assert_eq!(iter.next(), Some(&5));
    /// assert_eq!(iter.next(), Some(&6));
    /// assert_eq!(iter.next(), Some(&7));
    /// assert_eq!(iter.next(), Some(&8));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn borders(&self) -> impl Iterator<Item = &T> + '_ {
        match (self.height(), self.width()) {
            (0 | 1 | 2, _) | (_, 0 | 1) => Box::new(self.iter()) as Box<dyn Iterator<Item = &T>>,
            (height, width) => Box::new(
                self.through(Coord::at(0, 0), Coord::at(width - 1, 0))
                    .unwrap()
                    .chain(
                        self.through(Coord::at(0, height - 1), Coord::at(width - 1, height - 1))
                            .unwrap(),
                    )
                    .chain(
                        self.through(Coord::at(0, 1), Coord::at(0, height - 2))
                            .unwrap(),
                    )
                    .chain(
                        self.through(Coord::at(width - 1, 1), Coord::at(width - 1, height - 2))
                            .unwrap(),
                    ),
            ) as Box<dyn Iterator<Item = &T>>,
        }
    }

    /// Returns a [Grid] of the same size as self, with function f applied to each element from top to botom and left to right.
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![1, 2, 3, 4],
    ///     vec![5, 6, 7, 8],
    ///     vec![8, 7, 6, 5],
    ///     vec![4, 3, 2, 1],
    ///    ]);
    /// let grid = grid.map(|el| el.to_string());
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec!["1", "2", "3", "4"],
    ///         vec!["5", "6", "7", "8"],
    ///         vec!["8", "7", "6", "5"],
    ///         vec!["4", "3", "2", "1"],
    ///     ],
    /// );
    /// ```
    pub fn map<F, U>(self, mut f: F) -> Grid<U>
    where
        F: FnMut(T) -> U,
    {
        Grid::from(
            self.data
                .into_iter()
                .map(|line| line.into_iter().map(&mut f).collect())
                .collect(),
        )
    }

    /// Trim a grid from the left
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![0, 0, 1, 2, 0],
    ///     vec![0, 0, 0, 1, 0],
    ///     vec![0, 0, 1, 0, 0],
    ///     vec![0, 0, 0, 0, 0],
    ///    ]);
    /// grid.trim_left_matches(|&el| el == 0);
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![1, 2, 0],
    ///         vec![0, 1, 0],
    ///         vec![1, 0, 0],
    ///         vec![0, 0, 0],
    ///     ],
    /// );
    /// ```
    pub fn trim_left_matches(&mut self, to_trim: impl Fn(&T) -> bool) {
        let to_trim = self
            .lines()
            .map(|line| {
                line.iter()
                    .position(|el| !to_trim(el))
                    .unwrap_or_else(|| line.len())
            })
            .min()
            .unwrap_or_default();
        self.lines_mut()
            .for_each(|line| drop(line.drain(..to_trim)))
    }

    /// Trim a grid from the right
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![0, 0, 1, 2, 0],
    ///     vec![0, 0, 0, 1, 0],
    ///     vec![0, 0, 1, 0, 0],
    ///     vec![0, 0, 0, 0, 0],
    ///    ]);
    /// grid.trim_right_matches(|&el| el == 0);
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![0, 0, 1, 2],
    ///         vec![0, 0, 0, 1],
    ///         vec![0, 0, 1, 0],
    ///         vec![0, 0, 0, 0],
    ///     ],
    /// );
    /// ```
    pub fn trim_right_matches(&mut self, to_trim: impl Fn(&T) -> bool) {
        let to_trim = self
            .lines()
            .map(|line| {
                line.iter()
                    .rev()
                    .position(|el| !to_trim(el))
                    .unwrap_or_else(|| line.len())
            })
            .min()
            .unwrap_or_default();
        self.lines_mut()
            .for_each(|line| drop(line.drain(line.len() - to_trim..)))
    }

    /// Trim a grid from the top
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![0, 0, 0, 0, 0],
    ///     vec![0, 0, 0, 0, 0],
    ///     vec![0, 0, 1, 2, 0],
    ///     vec![0, 0, 0, 1, 0],
    ///     vec![0, 0, 1, 0, 0],
    ///     vec![0, 0, 0, 0, 0],
    ///    ]);
    /// grid.trim_top_matches(|&el| el == 0);
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![0, 0, 1, 2, 0],
    ///         vec![0, 0, 0, 1, 0],
    ///         vec![0, 0, 1, 0, 0],
    ///         vec![0, 0, 0, 0, 0],
    ///     ],
    /// );
    /// ```
    pub fn trim_top_matches(&mut self, to_trim: impl Fn(&T) -> bool) {
        let to_trim = self
            .lines()
            .position(|line| line.iter().any(|el| !to_trim(el)))
            .unwrap_or_default();
        self.data.drain(..to_trim);
    }

    /// Trim a grid from the bottom
    /// # Example
    ///
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![0, 0, 0, 0, 0],
    ///     vec![0, 0, 1, 2, 0],
    ///     vec![0, 0, 0, 1, 0],
    ///     vec![0, 0, 1, 0, 0],
    ///     vec![0, 0, 0, 0, 0],
    ///     vec![0, 0, 0, 0, 0],
    ///    ]);
    /// grid.trim_bottom_matches(|&el| el == 0);
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![0, 0, 0, 0, 0],
    ///         vec![0, 0, 1, 2, 0],
    ///         vec![0, 0, 0, 1, 0],
    ///         vec![0, 0, 1, 0, 0],
    ///     ],
    /// );
    /// ```
    pub fn trim_bottom_matches(&mut self, to_trim: impl Fn(&T) -> bool) {
        let to_trim = self
            .rlines()
            .position(|line| line.iter().any(|el| !to_trim(el)))
            .unwrap_or_default();
        self.data.drain(self.data.len() - to_trim..);
    }

    /// Trim a grid from all directions
    ///
    /// # Example
    /// ```
    /// let mut grid = aoc::Grid::from(vec![
    ///     vec![0, 0, 0, 0, 0],
    ///     vec![0, 0, 1, 2, 0],
    ///     vec![0, 0, 0, 1, 0],
    ///     vec![0, 0, 1, 0, 0],
    ///     vec![0, 0, 0, 0, 0],
    ///     vec![0, 0, 0, 0, 0],
    ///    ]);
    /// grid.trim_matches(|&el| el == 0);
    /// assert_eq!(
    ///     grid.into_inner(),
    ///     vec![
    ///         vec![1, 2],
    ///         vec![0, 1],
    ///         vec![1, 0],
    ///     ],
    /// );
    /// ```
    pub fn trim_matches(&mut self, to_trim: impl Fn(&T) -> bool) {
        self.trim_left_matches(&to_trim);
        self.trim_right_matches(&to_trim);
        self.trim_top_matches(&to_trim);
        self.trim_bottom_matches(&to_trim);
    }
}

impl<T: Default + Clone> Grid<T> {
    /// Create an empty [Grid] with specific dimension.
    ///
    /// See also [Grid::from, Grid::new].
    /// # Example
    ///
    /// ```
    /// use aoc::Grid;
    /// let mut grid: Grid<usize> = Grid::with_dimension(3, 2);
    /// assert_eq!(grid.into_inner(), vec![
    ///    vec![0, 0, 0],
    ///    vec![0, 0, 0],
    /// ]);
    /// ```
    pub fn with_dimension(col: usize, line: usize) -> Self {
        Self {
            data: vec![vec![T::default(); col]; line],
        }
    }
}

impl<T> std::ops::Index<&Coord<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: &Coord<usize>) -> &Self::Output {
        &self.data[index.y][index.x]
    }
}

impl<T> std::ops::IndexMut<&Coord<usize>> for Grid<T> {
    fn index_mut(&mut self, index: &Coord<usize>) -> &mut Self::Output {
        &mut self.data[index.y][index.x]
    }
}

impl<T> std::ops::Index<Coord<usize>> for Grid<T> {
    type Output = T;

    fn index(&self, index: Coord<usize>) -> &Self::Output {
        &self.data[index.y][index.x]
    }
}

impl<T> std::ops::IndexMut<Coord<usize>> for Grid<T> {
    fn index_mut(&mut self, index: Coord<usize>) -> &mut Self::Output {
        &mut self.data[index.y][index.x]
    }
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1][index.0]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.1][index.0]
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let largest_string = self
            .iter()
            .map(|el| el.to_string().chars().count())
            .max()
            .unwrap_or_default(); // if there was no element we wont enter in the next for_each so the value is not important
        self.lines().try_for_each(|line| {
            line.iter()
                .try_for_each(|el| write!(f, "{:>1$} ", el, largest_string))?;
            writeln!(f)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borders() {
        #[rustfmt::skip]
        let grids = vec![
            (Grid::from(vec![
                vec![1,  2,  3,  4],
                vec![9,  0,  0,  11],
                vec![10, 0,  0,  12],
                vec![5,  6,  7,  8],
            ]), 12),
            (Grid::from(vec![
                vec![1, 2, 3],
                vec![7, 0, 8],
                vec![4, 5, 6]
            ]), 8),
            (Grid::from(vec![
                vec![1, 2],
                vec![5, 6],
                vec![3, 4]
            ]), 6),
            (Grid::from(vec![
                vec![1],
                vec![2],
                vec![3],
            ]), 3),
            (Grid::from(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
            ]), 8),
            (Grid::from(vec![
                vec![1, 2, 3, 4],
            ]), 4),
            (Grid::from(vec![
                vec![1],
            ]), 1),
            (Grid::from(vec![
                vec![],
            ]), 0),
        ];

        for (grid, len) in grids {
            let borders: Vec<_> = grid.borders().collect();
            assert!(
                borders.len() == len && borders.windows(2).all(|slice| slice[0] < slice[1]),
                "Borders failed for the following grid: {}. Got {:?}",
                grid,
                borders,
            );
        }
    }
}
