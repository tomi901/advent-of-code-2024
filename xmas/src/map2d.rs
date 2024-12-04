use core::str;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

use crate::point2d::Point2D;

pub type ByteMap = Map2D<u8>;
pub type CharMap = Map2D<char>;

#[derive(Debug, Clone, PartialEq)]
pub struct Map2D<Tile = u8> {
    map: Vec<Tile>,
    width: usize,
    height: usize,
}

impl<T: Clone> Map2D<T> {
    pub fn new_filled(size: Point2D, tile: T) -> Self {
        let width = size.0 as usize;
        let height = size.1 as usize;
        let map = vec![tile; width * height];
        Self {
            map,
            width,
            height,
        }
    }
}

impl<T: Default> Map2D<T> {
    pub fn new_with_default_tiles(size: Point2D) -> Self {
        let width = size.0 as usize;
        let height = size.1 as usize;
        let mut map = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            map.push(T::default());
        }
        Self {
            map,
            width,
            height,
        }
    }
}

impl<T> Map2D<T> {
    pub fn from_str_with_parser<'a, Iter, Parser>(
        s: &'a str,
        parser: &mut Parser,
    ) -> Result<Self, ParseMapError>
        where Iter: Iterator<Item = T>,
        Parser: FnMut(&'a str) -> Iter
    {
        if s.is_empty() {
            return Err(ParseMapError::EmptyString);
        }

        let map = Vec::with_capacity(s.len());
        let mut lines = s.lines();
        
        let first_line = lines.next().unwrap();

        let mut map = Self { map, width: 0, height: 0 };
        map.parse_and_add_row(first_line, parser)?;
        for line in lines {
            map.parse_and_add_row(line, parser)?;
        }

        Ok(map)
    }

    pub fn parse_and_add_row<'a, Iter, Parser>(
        &mut self,
        line: &'a str,
        parser: &mut Parser,
    ) -> Result<(), ParseMapError>
        where Iter: Iterator<Item = T>,
        Parser: FnMut(&'a str) -> Iter
    {
        self.add_row(parser(line))
    }

    pub fn add_row(&mut self, row: impl Iterator<Item = T>) -> Result<(), ParseMapError> {
        let tiles = row.collect::<Vec<T>>();
        if self.height == 0 {
            self.width = tiles.len();
        } else if tiles.len() != self.width {
            return Err(ParseMapError::InconsistentRowSize { current: tiles.len(), expected: self.width });
        }
        self.map.extend(tiles.into_iter());
        self.height += 1;
        Ok(())
    }

    pub fn is_inside(&self, point: Point2D) -> bool {
        point.0 >= 0 && point.1 >= 0 && (point.0 as usize) < self.width && (point.1 as usize) < self.height
    }

    pub fn set_tile(&mut self, point: Point2D, tile: T) -> bool {
        if let Some(index) = self.get_index(point) {
            self.map[index] = tile;
            true
        } else {
            false
        }
    }

    pub fn get_tile(&self, point: Point2D) -> Option<&T> {
        self.get_index(point).and_then(|i| self.map.get(i))
    }

    pub fn get_tile_mut(&mut self, point: Point2D) -> Option<&mut T> {
        self.get_index(point).and_then(|i| self.map.get_mut(i))
    }

    pub fn get_index(&self, point: Point2D) -> Option<usize> {
        self.is_inside(point).then(|| point.0 as usize + (point.1 as usize * self.width))
    }

    pub fn iter_points(&self) -> impl Iterator<Item = Point2D> + '_ {
        (0..(self.height as isize))
            .flat_map(|y| (0..(self.width as isize)).map(move |x| Point2D(x, y)))
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.map.iter()
    }

    pub fn iter_with_points(&self) -> impl Iterator<Item = (Point2D, &T)> + '_ {
        (0..(self.height as isize))
            .flat_map(|y| (0..(self.width as isize)).map(move |x| Point2D(x, y)))
            .map(|p| (p, self.get_tile(p).unwrap()))
    }

    pub fn row(&self, index: usize) -> &[T] {
        let start = index * self.width;
        let end = start + self.width;
        &self.map[start..end]
    }

    pub fn rows_iter(&self) -> impl Iterator<Item = &[T]> {
        (0..self.height).map(|y| self.row(y))
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> Point2D {
        Point2D(self.width as isize, self.height as isize)
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseMapError {
    #[error("Can't parse an empty string to a Map2D")]
    EmptyString,
    #[error("Inconsistent row size. Current: {current} Expected: {expected}")]
    InconsistentRowSize { current: usize, expected: usize },
}

impl FromStr for Map2D<u8> {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_with_parser(s, &mut str::bytes)
    }
}

impl Display for Map2D<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.rows_iter() {
            writeln!(f, "{}", String::from_utf8_lossy(line))?;
        }
        Ok(())
    }
}

impl FromStr for Map2D<char> {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_with_parser(s, &mut str::chars)
    }
}

impl Display for Map2D<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.rows_iter() {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn builds_map_correctly() {
        let map = ByteMap::new_with_default_tiles(Point2D(20, 10));

        assert_eq!(map.width, 20);
        assert_eq!(map.height, 10);
        assert_eq!(map.map.len(), 20 * 10);
    }

    #[rstest]
    #[case(Point2D(20, 10), Point2D(0, 0), Some(0))]
    #[case(Point2D(20, 10), Point2D(4, 5), Some(104))]
    #[case(Point2D(20, 10), Point2D(-1, 0), None)]
    #[case(Point2D(20, 10), Point2D(0, -1), None)]
    #[case(Point2D(20, 10), Point2D(20, 0), None)]
    #[case(Point2D(20, 10), Point2D(0, 10), None)]
    fn index_is_equal_to_expected(
        #[case] map_size: Point2D,
        #[case] point: Point2D,
        #[case] expected: Option<usize>,
    ) {
        let map = ByteMap::new_with_default_tiles(map_size);
        let index = map.get_index(point);

        assert_eq!(index, expected);
    }

    #[test]
    fn parses_map_correctly() {
        const MAP: &str = concat!(
            "0123\n",
            "4567\n",
            "89AB\n",
        );

        let map = ByteMap::from_str(MAP).unwrap();
        assert_eq!(map.width, 4);
        assert_eq!(map.height, 3);
    }

    #[test]
    fn parse_map_returns_empty_error() {
        let result = ByteMap::from_str("");
        assert_eq!(result, Err(ParseMapError::EmptyString));
    }

    #[test]
    fn parse_map_returns_inconsistent_lines_error() {
        const MAP: &str = concat!(
            "0123\n",
            "457\n",
            "89AB\n",
        );

        let result = ByteMap::from_str(MAP);
        assert_eq!(result, Err(ParseMapError::InconsistentRowSize { current: 3, expected: 4 }))
    }
}
