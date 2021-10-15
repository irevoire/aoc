use std::fmt::Display;

use crate::Coord;
use anyhow::Result;

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// create an empty grid
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// create a grid from a Vec of Vec
    pub fn from(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    /// Return the inner Vec<Vec<_>>
    pub fn into_inner(self) -> Vec<Vec<T>> {
        self.data
    }

    /// Return the width of the grid
    pub fn width(&self) -> usize {
        self.data[0].len()
    }

    /// Return the height of the grid
    pub fn height(&self) -> usize {
        self.data.len()
    }

    /// Return an iterator on all the elements of the grid
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|sub| sub.iter())
    }

    /// Return a mutable iterator on all the elements of the grid
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().flat_map(|sub| sub.iter_mut())
    }

    /// Return an iterator on all the lines of the grid
    pub fn lines(&self) -> impl Iterator<Item = &[T]> {
        self.data.iter().map(|v| v.as_slice())
    }

    /// Return an iterator on all the lines of the grid from the bottom to the top
    pub fn rlines(&self) -> impl Iterator<Item = &[T]> {
        self.data.iter().rev().map(|v| v.as_slice())
    }

    /// Return a mutable iterator on all the lines of the grid
    pub fn lines_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
        self.data.iter_mut()
    }

    /// Return a mutable iterator on all the lines of the grid from the bottom to the top
    pub fn rlines_mut(&mut self) -> impl Iterator<Item = &mut Vec<T>> {
        self.data.iter_mut().rev()
    }

    /// Return an iterator of all the element in the grid from one point to another
    pub fn through(
        &self,
        from: Coord<usize>,
        to: Coord<usize>,
    ) -> Result<impl Iterator<Item = &T>> {
        Ok(from.to(to)?.map(move |coord| &self[coord]))
    }

    /// Return a mutable iterator of all the element in the grid from one point to another
    pub fn through_mut<'a>(
        &'a mut self,
        from: Coord<usize>,
        to: Coord<usize>,
    ) -> Result<impl Iterator<Item = &'a mut T>> {
        Ok(from
            .to(to)?
            .map(move |coord| unsafe { std::mem::transmute(&mut self[coord]) }))
    }

    /// Trim a grid from the left
    /// # Example
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
    ///        ],
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
    ///        ],
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
    ///        ],
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
    ///        ],
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
    ///        ],
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
    pub fn with_capacity(line: usize, col: usize) -> Self {
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
