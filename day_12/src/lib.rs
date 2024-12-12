use std::{collections::HashSet, str::FromStr};
use xmas::{direction::{QuarterRotation, DIRECTIONS}, map2d::{ByteMap, ParseMapError}, point2d::Point2D};

pub fn calculate_fence_costs(input: &str) -> Result<u64, ParseMapError> {
    let map = ByteMap::from_str(input)?;

    let mut checked_tiles = HashSet::new();
    Ok(map.iter_points()
        .flat_map(|p| check_cost_from(p, &map, &mut checked_tiles))
        .sum())
}

pub fn calculate_fence_costs_with_discount(input: &str) -> Result<u64, ParseMapError> {
    let map = ByteMap::from_str(input)?;

    let mut checked_tiles = HashSet::new();
    Ok(map.iter_points()
        .flat_map(|p| check_cost_from_with_discount(p, &map, &mut checked_tiles))
        .sum())
}

fn get_area_tiles(point: Point2D, map: &ByteMap, checked_tiles: &mut HashSet<Point2D>) -> Option<HashSet<Point2D>> {
    if checked_tiles.contains(&point) {
        return None;
    }

    let tile_to_match = *map.get_tile(point).unwrap();
    // unsafe { println!("Checking area: {}", char::from_u32_unchecked(tile_to_match as u32)) }

    let mut area_tiles = HashSet::new();
    let mut check_stack = vec![point];
    while let Some(candidate) = check_stack.pop() {
        if checked_tiles.contains(&candidate) {
            continue;
        }

        match map.get_tile(candidate) {
            Some(&t) if t == tile_to_match => (),
            _ => continue,
        }

        checked_tiles.insert(candidate);
        area_tiles.insert(candidate);

        for other_position in DIRECTIONS.map(|d| candidate + d.as_point()) {
            if !area_tiles.contains(&other_position) {
                check_stack.push(other_position);
            }
        }
    }

    Some(area_tiles)
}

fn check_cost_from(point: Point2D, map: &ByteMap, checked_tiles: &mut HashSet<Point2D>) -> Option<u64> {
    let area_tiles = match get_area_tiles(point, map, checked_tiles) {
        Some(t) => t,
        None => return None,
    };

    let perimeter = area_tiles.iter()
        .flat_map(|&p| DIRECTIONS.iter().map(move |d| p + d.as_point()))
        .filter(|p| !area_tiles.contains(p))
        .count() as u64;

    let area = area_tiles.len() as u64;
    Some(area * perimeter)
}

// I was about to fuse the perimeter edges, but counting corners is much more efficient and easier
fn check_cost_from_with_discount(point: Point2D, map: &ByteMap, checked_tiles: &mut HashSet<Point2D>) -> Option<u64> {
    let area_tiles = match get_area_tiles(point, map, checked_tiles) {
        Some(t) => t,
        None => return None,
    };

    let corners = area_tiles.iter()
        .flat_map(|&p| DIRECTIONS.iter().map(move |&d| (p, d)))
        .filter(|&(tile, dir)| {
            let a = tile + dir.as_point();
            let b = tile + dir.turn(QuarterRotation::Right).as_point();
            // Outer corners
            if !area_tiles.contains(&a) && !area_tiles.contains(&b) {
                return true;
            }

            // Innner corners
            let corner_point = tile + dir.as_point() + dir.turn(QuarterRotation::Right).as_point();
            area_tiles.contains(&a) && area_tiles.contains(&b) && !area_tiles.contains(&corner_point)
        })
        .count() as u64;

    let area = area_tiles.len() as u64;
    Some(area * corners)
}
