use std::{collections::HashSet, str::FromStr};
use xmas::{direction::DIRECTIONS, map2d::ByteMap, point2d::Point2D};

pub fn calculate_fence_costs(input: &str) -> u64 {
    let map = ByteMap::from_str(input).unwrap();

    let mut checked_tiles = HashSet::new();
    let mut total_cost = 0;
    for point in map.iter_points() {
        total_cost += check_cost_from(point, &map, &mut checked_tiles);
    }

    total_cost
}

fn check_cost_from(point: Point2D, map: &ByteMap, checked_tiles: &mut HashSet<Point2D>) -> u64 {
    if checked_tiles.contains(&point) {
        return 0;
    }

    let tile_to_match = *map.get_tile(point).unwrap();
    // unsafe { println!("Checking area: {}", char::from_u32_unchecked(tile_to_match as u32)) }

    let mut area_tiles = HashSet::new();
    let mut perimeter = 0;
    let mut check_stack = vec![point];
    while let Some(candidate) = check_stack.pop() {
        if checked_tiles.contains(&candidate) {
            continue;
        }

        match map.get_tile(candidate) {
            Some(&t) if t == tile_to_match => (),
            _ => continue,
        }

        // println!("Checking point: {}", candidate);
        checked_tiles.insert(candidate);
        area_tiles.insert(candidate);
        perimeter += 4;
        // println!("Perimeter: {}", perimeter);
        // println!("A: {}", area_tiles.len());

        for other_position in DIRECTIONS.map(|d| candidate + d.as_point()) {
            if area_tiles.contains(&other_position) {
                // println!("{} and {} adjacent, substracting 2 to perimeter {}", candidate, other_position, perimeter);
                perimeter -= 2;
            } else {
                // println!("Queueing: {}", other_position);
                check_stack.push(other_position);
            }
        }
    }

    let area = area_tiles.len() as u64;
    // println!("A: {}, P: {}", area, perimeter);
    let result = area * perimeter;
    // unsafe {
    //     let ch = char::from_u32_unchecked(tile_to_match as u32);
    //     println!("{}: Area {} * Perimeter {} = Cost {}", ch, area, perimeter, result);
    // }
    result
}
