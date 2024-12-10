use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, str::FromStr};

use xmas::{direction::DIRECTIONS, keyed_ord::KeyedOrd, map2d::{ByteMap, ParseMapError}, point2d::Point2D};

#[derive(Debug, Clone)]
struct Breadcrumb {
    point: Point2D,
    value: u8,
}

impl Breadcrumb {
    fn to_priority(self) -> Reverse<KeyedOrd<Breadcrumb, u8>> {
        let key = self.value;
        Reverse(KeyedOrd::new(self, key))
    }
}

pub fn calculate_hiking_score(input: &str) -> Result<u64, ParseMapError> {
    let map = ByteMap::from_str(input)?;
    Ok(calculate_hiking_score_from_anywhere(&map, false))
}

pub fn calculate_hiking_ratings(input: &str) -> Result<u64, ParseMapError> {
    let map = ByteMap::from_str(input).unwrap();
    Ok(calculate_hiking_score_from_anywhere(&map, true))
}

fn calculate_hiking_score_from_anywhere(map: &ByteMap, allow_repeats: bool) -> u64 {
    map.iter_with_points()
        .filter(|&(_, t)| t == &b'0')
        .map(|(start, _)| calculate_hiking_score_from(&map, start, allow_repeats))
        .sum()
}

fn calculate_hiking_score_from(map: &ByteMap, start: Point2D, allow_repeats: bool) -> u64 {
    // println!("Calculating paths from {}...", start);

    let mut open_list = BinaryHeap::new();
    open_list.push(Breadcrumb { point: start, value: b'0' }.to_priority());

    let mut closed_list = HashSet::new();

    let mut score = 0;
    while let Some(cur_candidate) = open_list.pop() {
        let cur_candidate = cur_candidate.0.value;

        if !allow_repeats {
            if closed_list.contains(&cur_candidate.point) {
                continue;
            }
            closed_list.insert(cur_candidate.point);
        }
        // unsafe {
        //     println!("Candidate {} @ {}", char::from_u32_unchecked(cur_candidate.value as u32), cur_candidate.point);
        // }

        if cur_candidate.value == b'9' {
            score += 1;
            continue;
        }
        
        let next_value = cur_candidate.value + 1;
        let new_candidates = DIRECTIONS.iter()
            .map(|d| cur_candidate.point + d.as_point())
            .flat_map(|p| map.get_tile(p).cloned().map(|t| (p, t)))
            .filter(|&(_, t)| t == next_value)
            .map(|(point, value)| Breadcrumb { point, value }.to_priority());
        open_list.extend(new_candidates);
    }

    score
}
