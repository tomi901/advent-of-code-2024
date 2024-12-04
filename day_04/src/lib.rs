use std::str::FromStr;

use xmas::{direction::DIRECTIONS_8, map2d::{Map2D, ParseMapError}};

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
