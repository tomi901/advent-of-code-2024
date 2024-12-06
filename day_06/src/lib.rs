use std::{collections::HashSet, str::FromStr};

use xmas::{direction::Direction, map2d::ByteMap};


pub fn get_patrol_visited_count(input: &str) -> usize {
    let map = ByteMap::from_str(input).unwrap();
    let (start, _) = map.iter_with_points()
        .find(|&(_, tile)| tile == &b'^')
        .unwrap();

    let mut visited = HashSet::new();

    let mut cur_pos = start;
    let mut cur_dir = Direction::Up;
    loop {
        visited.insert(cur_pos);
        let next_pos = cur_pos + cur_dir.as_point();
        match map.get_tile(next_pos) {
            Some(b'#') => cur_dir = cur_dir.turn(xmas::direction::QuarterRotation::Right),
            Some(_) => cur_pos = next_pos,
            None => break,
        }
    }
    visited.len()
}
