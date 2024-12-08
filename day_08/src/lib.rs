use std::{collections::{HashMap, HashSet}, str::FromStr};
use xmas::{map2d::{ByteMap, ParseMapError}, point2d::Point2D};

type Frequency = u8;

#[derive(Debug)]
struct AntennaMap {
    map: ByteMap,
    antennas_by_id: HashMap<Frequency, Vec<Point2D>>,
}

impl AntennaMap {
    /// Returns an iterator which contains all of the antenna pairs of the same frequency.
    /// They don't repeat so it should contain `a!` combinations per frequency where `a`
    /// is the amount of antennas in that frequency.
    pub fn find_unique_pairs(&self) -> impl Iterator<Item = (Frequency, Point2D, Point2D)> + '_ {
        self.antennas_by_id.iter()
            .flat_map(|(freq, points)| points.iter()
                .enumerate()
                .map(move |(i, p)| (freq, *p, &points[i + 1..])))
            .flat_map(|(freq, point, others)| others.iter()
                .map(move |o| (*freq, point, *o)))
    }

    pub fn is_inside(&self, point: Point2D) -> bool {
        self.map.is_inside(point)
    }
}

impl FromStr for AntennaMap {
    type Err = ParseMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = ByteMap::from_str(s)?;
        let mut antennas_by_id = HashMap::new();
        for (point, tile) in map.iter_with_points().filter(|&(_, t)| t != &b'.' && t != &b'#') {
            let entry = antennas_by_id.entry(*tile).or_insert_with(|| vec![]);
            entry.push(point);
        }
        Ok(Self { map, antennas_by_id })
    }
}

pub fn calculate_antinodes_locations(input: &str) -> Result<usize, ParseMapError> {
    let map = AntennaMap::from_str(&input)?;

    let mut antinodes = HashSet::new();
    for (_frequency, a, b) in map.find_unique_pairs() {
        // println!("{}: {} <-> {}", char::from_u32(*frequency as u32).unwrap(), point, other);
        let diff = b - a;
        antinodes.insert(b + diff);
        antinodes.insert(a - diff);
    }
    antinodes.retain(|p| map.is_inside(*p));
    // println!("{:?}", antinodes);

    Ok(antinodes.len())
}

pub fn calculate_antinodes_locations_with_resonance(input: &str) -> Result<usize, ParseMapError> {
    let map = AntennaMap::from_str(&input)?;

    let mut antinodes = HashSet::new();
    for (_frequency, a, b) in map.find_unique_pairs() {
        let diff = b - a;
        let mut ab_resonance_point = b;
        while map.is_inside(ab_resonance_point) {
            antinodes.insert(ab_resonance_point);
            ab_resonance_point += diff;
        }
        let mut ba_resonance_point = a;
        while map.is_inside(ba_resonance_point) {
            antinodes.insert(ba_resonance_point);
            ba_resonance_point -= diff;
        }
    }

    Ok(antinodes.len())
}
