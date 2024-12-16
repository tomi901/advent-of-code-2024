use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}, str::FromStr};

use xmas::{direction::{Direction, QuarterRotation}, keyed_ord::KeyedOrd, map2d::CharMap, point2d::Point2D};


pub fn get_lowest_maze_cost(s: &str) -> u64 {
    let map = CharMap::from_str(s).unwrap();
    let start = map.iter_with_points()
        .find(|&(_, ch)| ch == &'S')
        .map(|(p, _)| p)
        .unwrap();
    let end = map.iter_with_points()
        .find(|&(_, ch)| ch == &'E')
        .map(|(p, _)| p)
        .unwrap();

    let info = get_lowest_maze_path_cost(map, start, end).unwrap();
    println!("{} turns * 1000 + {} steps = {}", info.turns, info.steps, info.cost());
    info.cost()
}

fn get_lowest_maze_path_cost(map: CharMap, start: Point2D, end: Point2D) -> Option<Breadcrumb> {
    let mut open_list = BinaryHeap::new();
    open_list.push(Breadcrumb::new(start).as_priority());

    let mut closed_list = HashSet::new();

    while let Some(candidate) = open_list.pop() {
        let candidate = candidate.value;
        if candidate.pos == end {
            return Some(candidate);
        }

        if closed_list.contains(&(candidate.pos, candidate.dir)) {
            continue;
        }
        closed_list.insert((candidate.pos, candidate.dir));

        let forward = candidate.pos + candidate.dir.as_point();
        // println!("{forward}");
        let next = map.get_tile(forward)
            .filter(|&t| t != &'#')
            .map(|_| Breadcrumb { pos: forward, steps: candidate.steps + 1, ..candidate.clone() })
            .into_iter()
            .chain([candidate.turn(QuarterRotation::Left), candidate.turn(QuarterRotation::Right)])
            .map(|c| c.as_priority());

        open_list.extend(next);
        // println!("{} candidate/s", open_list.len());
        // println!("{:?}", open_list);
    }
    None
}

#[derive(Debug, Clone)]
struct Breadcrumb {
    steps: u64,
    turns: u64,
    dir: Direction,
    pos: Point2D,
}

impl Breadcrumb {
    pub fn new(pos: Point2D) -> Self {
        Self { steps: 0, turns: 0, dir: Direction::Right, pos }
    }

    pub fn turn(&self, rot: QuarterRotation) -> Self {
        Self { dir: self.dir.turn(rot), turns: self.turns + 1, ..self.clone() }
    }

    pub fn cost(&self) -> u64 {
        self.turns * 1000 + self.steps
    }

    pub fn as_priority(self) -> KeyedOrd<Self, Reverse<u64>> {
        let ord = Reverse(self.cost());
        KeyedOrd { value: self, key: ord }
    }
}
