use std::str::FromStr;

use xmas::{direction::{QuarterRotation, DIRECTIONS, DIRECTIONS_8}, map2d::{Map2D, ParseMapError}, point2d::Point2D};

pub fn find_xmas_count(input: &str) -> Result<usize, ParseMapError> {
    let map = LetterMap::from_str(input)?;
    const WORD: &str = "XMAS";
    Ok(map.map.iter_points()
        .flat_map(|p| DIRECTIONS_8.iter().map(move |&dir| (p, dir)))
        .filter(|&(point, dir)| {
            let mut cur = point;
            for letter in WORD.bytes() {
                let tile = map.map.get_tile(cur);
                if tile.is_none() || tile.is_some_and(|&t| t != letter) {
                    return false;
                }
                cur += dir;
            }
            true
        })
        .count())
}

pub fn find_crossed_mas_count(input: &str) -> Result<usize, ParseMapError> {
    let map = LetterMap::from_str(input)?;
    let tile_is = |point: Point2D, expected: u8| -> bool {
        match map.map.get_tile(point) {
            Some(&t) => t == expected,
            None => false,
        }
    };

    Ok(map.map.iter_with_points()
        .filter(|&(_, t)| t == &b'A')
        .filter(|&(point, _)| {
            DIRECTIONS.iter()
                .any(|&dir| {
                    let m_left = point + dir.combined(dir.turn(QuarterRotation::Left));
                    if !tile_is(m_left, b'M') {
                        return false;
                    }
                    let m_right = point + dir.combined(dir.turn(QuarterRotation::Right));
                    if !tile_is(m_right, b'M') {
                        return false;
                    }

                    let inverse = dir.inverse();
                    let s_left = point + inverse.combined(dir.turn(QuarterRotation::Left));
                    if !tile_is(s_left, b'S') {
                        return false;
                    }
                    let s_right = point + inverse.combined(dir.turn(QuarterRotation::Right));
                    if !tile_is(s_right, b'S') {
                        return false;
                    }
                    true
                })
        })
        .count())
}

struct LetterMap {
    map: Map2D,
}

impl FromStr for LetterMap {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = Map2D::from_str(s)?;
        Ok(Self { map })
    }
}
