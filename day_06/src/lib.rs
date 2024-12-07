use std::{collections::HashSet, str::FromStr, time::Instant};

use anyhow::Context;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use xmas::{direction::{Direction, QuarterRotation}, map2d::ByteMap, point2d::Point2D};


pub fn get_patrol_visited_count(input: &str) -> Result<usize, anyhow::Error> {
    let map = ByteMap::from_str(input)?;
    let start = find_start(&map).context("No starting point")?;

    let visited = get_visited_tiles(&map, start);
    println!("Visited {}/{}", visited.len(), map.width() * map.height());

    Ok(visited.len())
}

pub fn find_loop_count(input: &str) -> Result<usize, anyhow::Error> {
    let map = ByteMap::from_str(input)?;
    let start = find_start(&map).context("No starting point")?;

    let original_visited = get_visited_tiles(&map, start);
    let start_time = Instant::now();
    let count = original_visited.into_par_iter()
        .filter(|&new_obstacle| {
            if new_obstacle == start {
                return false;
            }
            let mut cur_pos = start;
            let mut cur_dir = Direction::Up;

            let mut path = HashSet::new();

            loop {
                if path.contains(&(cur_pos, cur_dir)) {
                    // println!("Found loop");
                    return true;
                }
                path.insert((cur_pos, cur_dir));

                let next_pos = cur_pos + cur_dir.as_point();
                match map.get_tile(next_pos) {
                    Some(_) if next_pos == new_obstacle => cur_dir = cur_dir.turn(QuarterRotation::Right),
                    Some(b'#') => cur_dir = cur_dir.turn(QuarterRotation::Right),
                    Some(_) => cur_pos = next_pos,
                    None => break,
                }
            }
            false
        })
        .count();
    let elapsed = start_time.elapsed();
    println!("Calculated in {:?}", elapsed);
    Ok(count)
}

fn get_visited_tiles(map: &ByteMap, start: Point2D) -> HashSet<Point2D> {
    let mut visited = HashSet::new();

    let mut cur_pos = start;
    let mut cur_dir = Direction::Up;
    loop {
        visited.insert(cur_pos);
        let next_pos = cur_pos + cur_dir.as_point();
        match map.get_tile(next_pos) {
            Some(b'#') => cur_dir = cur_dir.turn(QuarterRotation::Right),
            Some(_) => cur_pos = next_pos,
            None => break,
        }
    }

    visited
}

fn find_start(map: &ByteMap) -> Option<Point2D> {
    map.iter_with_points()
        .find(|&(_, tile)| tile == &b'^')
        .map(|(point, _)| point)
}
