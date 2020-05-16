use crate::Coord;
use anyhow::Result;

#[derive(Debug, Clone, Default)]
pub struct Grid<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// create an empty grid
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// create a grid from a vec
    pub fn from(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flat_map(|sub| sub.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut().flat_map(|sub| sub.iter_mut())
    }

    pub fn lines(&self) -> impl Iterator<Item = &[T]> {
        self.data.iter().map(|v| v.as_slice())
    }

    pub fn lines_mut(&mut self) -> impl Iterator<Item = &mut [T]> {
        self.data.iter_mut().map(|v| v.as_mut_slice())
    }

    pub fn through(
        &self,
        from: Coord<usize>,
        to: Coord<usize>,
    ) -> Result<impl Iterator<Item = &T>> {
        Ok(from.to(to)?.map(move |coord| &self[coord]))
    }

    pub fn through_mut<'a>(
        &'a mut self,
        from: Coord<usize>,
        to: Coord<usize>,
    ) -> Result<impl Iterator<Item = &'a mut T>> {
        Ok(from
            .to(to)?
            .map(move |coord| unsafe { std::mem::transmute(&mut self[coord]) }))
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
