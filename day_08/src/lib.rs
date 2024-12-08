use std::{collections::{HashMap, HashSet}, str::FromStr};
use xmas::map2d::{ByteMap, ParseMapError};


pub fn calculate_antinodes_locations(input: &str) -> Result<usize, ParseMapError> {
    let map = ByteMap::from_str(input)?;
    let mut antenas_by_id = HashMap::new();
    for (point, tile) in map.iter_with_points().filter(|&(_, t)| t != &b'.' && t != &b'#') {
        let entry = antenas_by_id.entry(tile).or_insert_with(|| vec![]);
        entry.push(point);
    }

    let mut antinodes = HashSet::new();
    for (_frequency, points) in antenas_by_id {
        let point_and_other_points = points.iter()
            .enumerate()
            .map(|(i, point)| (point, &points[(i + 1)..]));
        for (point, other_points) in point_and_other_points {
            for other in other_points {
                // println!("{}: {} <-> {}", char::from_u32(*frequency as u32).unwrap(), point, other);
                let diff = *other - *point;
                antinodes.insert(*other + diff);
                antinodes.insert(*point - diff);
            }
        }
    }
    antinodes.retain(|p| map.is_inside(*p));
    // println!("{:?}", antinodes);

    Ok(antinodes.len())
}

pub fn calculate_antinodes_locations_with_resonance(input: &str) -> Result<usize, ParseMapError> {
    let map = ByteMap::from_str(input)?;
    let mut antenas_by_id = HashMap::new();
    for (point, tile) in map.iter_with_points().filter(|&(_, t)| t != &b'.' && t != &b'#') {
        let entry = antenas_by_id.entry(tile).or_insert_with(|| vec![]);
        entry.push(point);
    }

    let mut antinodes = HashSet::new();
    for (_frequency, points) in antenas_by_id {
        let point_and_other_points = points.iter()
            .enumerate()
            .map(|(i, point)| (point, &points[(i + 1)..]));
        for (point, other_points) in point_and_other_points {
            for other in other_points {
                // println!("{}: {} <-> {}", char::from_u32(*frequency as u32).unwrap(), point, other);
                let diff = *other - *point;
                let mut ab_resonance_point = *other;
                while map.is_inside(ab_resonance_point) {
                    antinodes.insert(ab_resonance_point);
                    ab_resonance_point += diff;
                }
                let mut ba_resonance_point = *point;
                while map.is_inside(ba_resonance_point) {
                    antinodes.insert(ba_resonance_point);
                    ba_resonance_point -= diff;
                }
            }
        }
    }

    Ok(antinodes.len())
}
