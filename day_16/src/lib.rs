use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}, rc::Rc, str::FromStr};
use xmas::{direction::{Direction, QuarterRotation}, keyed_ord::KeyedOrd, map2d::CharMap, point2d::Point2D};

pub fn get_lowest_maze_cost(s: &str) -> u64 {
    let paths = get_best_paths_from_str(s);
    let info = paths.first().unwrap();
    // println!("{} turns * 1000 + {} steps = {}", info.turns, info.steps, info.cost());
    info.cost()
}

pub fn get_best_paths_tiles(s: &str) -> usize {
    let paths = get_best_paths_from_str(s);
    let positions: HashSet<_> = paths
        .iter()
        .flat_map(|bc| bc.backtrace())
        .map(|bc| bc.pos)
        .collect();

    positions.len()
}

fn get_best_paths_from_str(s: &str) -> Vec<Rc<Breadcrumb>> {
    let map = CharMap::from_str(s).unwrap();
    let start = map.iter_with_points()
        .find(|&(_, ch)| ch == &'S')
        .map(|(p, _)| p)
        .unwrap();
    let end = map.iter_with_points()
        .find(|&(_, ch)| ch == &'E')
        .map(|(p, _)| p)
        .unwrap();
    
    get_best_paths(map, start, end)
}

fn get_best_paths(map: CharMap, start: Point2D, end: Point2D) -> Vec<Rc<Breadcrumb>> {
    let mut open_list = BinaryHeap::new();
    open_list.push(Breadcrumb::new(start).as_priority());

    let mut best_scores = HashMap::new();

    let mut best_paths: Vec<Rc<Breadcrumb>> = vec![];
    let mut best_cost = u64::MAX;
    while let Some(candidate) = open_list.pop() {
        let candidate = Rc::new(candidate.value);
        let cost = candidate.cost();
        if candidate.pos == end {
            // println!("{cost} vs. {best_cost}");
            if cost < best_cost {
                best_paths.clear();
                best_cost = cost;
            }

            if cost == best_cost {
                best_paths.push(Rc::clone(&candidate));
                // println!("Best path found!");
            }
        }

        if cost > best_cost {
            continue;
        }

        if best_scores.get(&(candidate.pos, candidate.dir)).is_some_and(|&cur| cost > cur) {
            continue;
        }
        best_scores.insert((candidate.pos, candidate.dir), cost);

        let forward = candidate.pos + candidate.dir.as_point();

        let right = candidate.dir.turn(QuarterRotation::Right);
        let right_pos = candidate.pos + right.as_point();

        let left = candidate.dir.turn(QuarterRotation::Left);
        let left_pos = candidate.pos + left.as_point();

        // println!("{forward}");
        let previous_candidates = open_list.len();
        if map.get_tile(forward).is_some_and(|t| t != &'#') {
            // println!("Forward added");
            open_list.push(Breadcrumb {
                pos: forward,
                dir: candidate.dir,
                steps: candidate.steps + 1,
                turns: candidate.turns,
                previous: Some(Rc::clone(&candidate)),
            }.as_priority());
        }

        if map.get_tile(right_pos).is_some_and(|t| t != &'#') {
            // println!("Right added");
            open_list.push(Breadcrumb {
                pos: right_pos,
                dir: right,
                steps: candidate.steps + 1,
                turns: candidate.turns + 1,
                previous: Some(Rc::clone(&candidate)),
            }.as_priority());
        }

        if map.get_tile(left_pos).is_some_and(|t| t != &'#') {
            // println!("Left added");
            open_list.push(Breadcrumb {
                pos: left_pos,
                dir: left,
                steps: candidate.steps + 1,
                turns: candidate.turns + 1,
                previous: Some(Rc::clone(&candidate)),
            }.as_priority());
        }

        // let added = open_list.len() - previous_candidates;
        // if added == 0 {
        //     println!("Dead end!");
        // }
        // println!("{} candidate/s, {} added", open_list.len(), added);
        // println!("{:?}", open_list);
    }
    best_paths
}

#[derive(Debug, Clone)]
struct Breadcrumb {
    steps: u64,
    turns: u64,
    dir: Direction,
    pos: Point2D,
    previous: Option<Rc<Self>>,
}

impl Breadcrumb {
    pub fn new(pos: Point2D) -> Self {
        Self { steps: 0, turns: 0, dir: Direction::Right, pos, previous: None }
    }

    pub fn cost(&self) -> u64 {
        self.turns * 1000 + self.steps
    }

    pub fn as_priority(self) -> KeyedOrd<Self, Reverse<u64>> {
        let ord = Reverse(self.cost());
        KeyedOrd { value: self, key: ord }
    }

    pub fn backtrace(&self) -> impl Iterator<Item = &Self> {
        let mut cur = Some(self);
        std::iter::from_fn(move || {
            match cur {
                Some(bc) => {
                    cur = bc.previous.as_ref().map(|b| b.as_ref());
                    Some(bc)
                },
                None => None,
            }
        })
    }
}
