use std::{collections::{HashMap, HashSet}, hash::Hash};
use xmas::{direction::DIRECTIONS, map2d::CharMap, point2d::Point2D};
use pathfinding::directed::dijkstra;

pub fn calculate_best_shortcuts(input: &str, shortcut_required_saving: u64, cheat_time: u64, debug: bool) -> usize {
    let map = input.parse::<CharMap>().unwrap();
    let start = map.find(&'S').unwrap();
    let end = map.find(&'E').unwrap();

    let calculate_path = |start: Breadcrumb| {
        // println!("Calculating path from: {:?}", start);
        dijkstra::dijkstra(
            &start,
            |bc| bc.next_in(&map).map(move |bc| (bc, 1)),
            |bc| bc.point == end,
        )
    };

    let (path, base_cost) = calculate_path(Breadcrumb::new(start)).unwrap();
    println!("Base path calculated with cost: {}", base_cost);

    let time_per_tile: HashMap<Point2D, u64> = path.iter()
        .map(|bc| (bc.point, bc.time))
        .collect();

    let mut cheats = vec![];
    for bc in &path {
        let from: Point2D = bc.point;
        let cur_time = bc.time;

        let mut points_to_check = vec![from];
        let mut checked_points = HashSet::new();
        while let Some(next_candidate) = points_to_check.pop() {
            if checked_points.contains(&next_candidate) {
                continue;
            }
            checked_points.insert(next_candidate);
            let distance = next_candidate.manhattan_distance(from) as u64;
            // println!("Checking {} -> {} (distance: {})", from, next_candidate, distance);
            if distance > cheat_time {
                continue;
            }

            points_to_check.extend(DIRECTIONS.iter()
                .map(|dir| next_candidate + dir.as_point())
                .filter(|p| !checked_points.contains(p)));

            let time_with_cheat = cur_time + distance;
            let other_time = match time_per_tile.get(&next_candidate) {
                Some(time) => *time,
                None => continue,
            };

            if time_with_cheat >= other_time {
                continue;
            }

            let save_time = other_time - time_with_cheat;
            if save_time >= shortcut_required_saving {
                cheats.push(save_time);
            }
        }
    }

    let mut cheat_count = HashMap::new();
    for cheat in cheats {
        *cheat_count.entry(cheat).or_insert(0) += 1;
    }

    if debug {
        let mut cheat_count_vec: Vec<_> = cheat_count.iter().collect();
        cheat_count_vec.sort_by_key(|kvp| kvp.0);
        for (save_time, count) in cheat_count_vec {
            println!("- There are {count} cheat/s that save {save_time} picoseconds.");
        }
    }

    // println!("{:#?}", cheat_count);
    cheat_count.values()
        .sum()
}

#[derive(Debug, Clone)]
struct Breadcrumb {
    point: Point2D,
    time: u64,
}

impl Breadcrumb {
    pub fn new(point: Point2D) -> Self {
        Breadcrumb { point, time: 0 }
    }

    pub fn next_in<'a>(&self, map: &'a CharMap) -> impl Iterator<Item = Self> + 'a {
        let from = self.point;
        let next_time = self.time + 1;
        DIRECTIONS.iter()
            .map(move |dir| from + dir.as_point())
            .filter(move |point| map.get_tile(*point).is_some_and(|t| t != &'#'))
            .map(move |point| Breadcrumb {
                point,
                time: next_time,
            })
    }
}

impl PartialEq for Breadcrumb {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

impl Eq for Breadcrumb {
}

impl Hash for Breadcrumb {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.point.hash(state)
    }
}
