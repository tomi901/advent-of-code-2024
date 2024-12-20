use std::{collections::HashMap, hash::Hash};
use xmas::{direction::DIRECTIONS, map2d::CharMap, point2d::Point2D};
use pathfinding::directed::dijkstra;

pub fn calculate_best_shortcuts(input: &str, shortcut_required_saving: u64, cheat_time: u64) -> usize {
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

    let cheats = path.iter()
        .map(|bc| bc.as_cheat(cheat_time))
        .filter_map(|start| dijkstra::dijkstra(
            &start,
            |bc| {
                bc.next_in(&map).map(|next| {
                    let cost = base_cost
                        - time_per_tile.get(&next.point).cloned().unwrap_or(base_cost)
                        + 1;
                    (next, cost)
                })
            },
            |bc| bc.cheat_time == 0 && time_per_tile.contains_key(&bc.point),
        ))
        .filter_map(|(path, _)| {
            let last_bc = path.last().unwrap();
            let point = last_bc.point;
            let original_time = *time_per_tile.get(&point).unwrap();
            let new_time = last_bc.time;
            if new_time >= original_time {
                return None;
            }
            
            let save_time = original_time - new_time;
            if save_time < shortcut_required_saving {
                return None
            }
            println!("Cheat {}: saved {} picoseconds", path.first().unwrap().point, save_time);
            Some(save_time)
        });

    let mut cheat_count = HashMap::new();
    for cheat in cheats {
        *cheat_count.entry(cheat).or_insert(0) += 1;
    }

    let mut cheat_count_vec: Vec<_> = cheat_count.iter().collect();
    cheat_count_vec.sort_by_key(|kvp| kvp.0);
    for (save_time, count) in cheat_count_vec {
        println!("- There are {count} cheat/s that save {save_time} picoseconds.");
    }

    // println!("{:#?}", cheat_count);
    cheat_count.values()
        .sum()
}

#[derive(Debug, Clone)]
struct Breadcrumb {
    point: Point2D,
    time: u64,
    cheat_time: u64,
    just_enabled_cheat: bool,
}

impl Breadcrumb {
    pub fn new(point: Point2D) -> Self {
        Breadcrumb { point, time: 0, cheat_time: 0, just_enabled_cheat: false }
    }

    pub fn next_in<'a>(&self, map: &'a CharMap) -> impl Iterator<Item = Self> + 'a {
        let from = self.point;
        let next_time = self.time + 1;
        let can_cheat = self.cheat_time > 0;
        let just_enabled_cheat = self.just_enabled_cheat;
        // if can_cheat {
        //     println!("{}", self.cheat_time);
        // }
        let next_cheating_time = if can_cheat { self.cheat_time - 1 } else { 0 };
        DIRECTIONS.iter()
            .map(move |dir| from + dir.as_point())
            .filter(move |point| map.get_tile(*point).is_some_and(|t| {
                if just_enabled_cheat {
                    t == &'#'
                } else {
                    can_cheat || t != &'#'
                }
            }))
            .map(move |point| Breadcrumb {
                point,
                time: next_time,
                cheat_time: next_cheating_time,
                just_enabled_cheat: false,
            })
    }

    pub fn as_cheat(&self, cheat_time: u64) -> Self {
        Self {
            cheat_time,
            just_enabled_cheat: true,
            ..self.clone()
        }
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
