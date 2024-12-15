use std::{cell::RefCell, collections::HashSet, str::FromStr};
use xmas::{direction::Direction, map2d::CharMap, point2d::Point2D};

pub fn box_gps_sum(input: &str) -> isize {
    let (warehouse_s, movements_s) = input.split_once("\n\n").unwrap();
    let mut warehouse = Warehouse::from_str(warehouse_s).unwrap();
    let movements = parse_directions(movements_s);

    // warehouse.debug_display();
    for dir in movements {
        warehouse.move_robot(dir);
        // warehouse.debug_display();
    }

    warehouse.box_gps_sum()
}

pub fn box_gps_sum_wide(input: &str) -> isize {
    let (warehouse_s, movements_s) = input.split_once("\n\n").unwrap();
    let mut warehouse = Warehouse::from_str(warehouse_s).unwrap().clone_wide_version();
    // warehouse.debug_display();
    let movements = parse_directions(movements_s);

    // warehouse.debug_display();
    for dir in movements {
        warehouse.move_robot(dir);
        // println!("Moved: {:?}", dir);
        // warehouse.debug_display();
    }

    warehouse.box_gps_sum()
}

struct Warehouse {
    map: CharMap,
    robot: Point2D,
    wide: bool,
}

impl Warehouse {
    pub fn new(map: CharMap) -> Self {
        let robot = map.iter_with_points()
            .find(|&(_, tile)| tile == &'@')
            .map(|(p, _)| p)
            .unwrap();
        Self { map, robot, wide: false }
    }

    pub fn clone_wide_version(&self) -> Self {
        let new_size = Point2D(self.map.width() as isize * 2, self.map.height() as isize);
        let mut new_map = CharMap::new_filled(new_size, '.');
        for (point, tile) in self.map.iter_with_points() {
            let new_tiles = match tile {
                '#' => ['#', '#'],
                '.' => ['.', '.'],
                '@' => ['@', '.'],
                'O' => ['[', ']'],
                _ => unreachable!(),
            };

            let new_point = Point2D(point.0 * 2, point.1);
            new_map.set_tile(new_point, new_tiles[0]);
            new_map.set_tile(new_point + Point2D(1, 0), new_tiles[1]);
        }
        Self { wide: true, ..Self::new(new_map) }
    }

    pub fn box_gps_sum(&self) -> isize {
        self.map.iter_with_points()
            .filter_map(|(p, t)| matches!(t, 'O' | '[').then_some(p))
            .map(|p| p.0 + (p.1 * 100))
            .sum()
    }

    fn move_robot(&mut self, dir: Direction) {
        let start_pos = self.robot;
        let new_target_pos = start_pos + dir.as_point();
        
        let push_boxes = RefCell::new(vec![]);
        let mut already_pushing = HashSet::new();
        // True if movement is valid, false if cancelled
        let mut try_push = |point: Point2D| -> bool {
            let pushing_box = match self.map.get_tile(point) {
                Some('O' | '[') => point,
                Some(']') => point + Point2D(-1, 0),
                Some('#') => {
                    // println!("Hit wall!");
                    return false;
                }
                Some(_) => return true,
                None => unreachable!(),
            };

            if already_pushing.contains(&pushing_box) {
                return true;
            }
            // println!("Pushing: {}", pushing_box);

            push_boxes.borrow_mut().push(pushing_box);
            already_pushing.insert(pushing_box);
            return true;
        };

        if !try_push(new_target_pos) {
            return;
        }

        let mut i = 0;
        while i < push_boxes.borrow().len() {
            let box_pos = push_boxes.borrow()[i];
            let new_box_pos = box_pos + dir.as_point();
            if !try_push(new_box_pos) {
                // println!("Chained hit wall!");
                return;
            }

            if self.wide {
                let right_point = new_box_pos + Point2D(1, 0);
                // println!("Checking right side {right_point}...");
                if !try_push(right_point) {
                    // println!("Chained hit wall on the right!");
                    return;
                }
            }

            // println!("Iter {i}, list length is: {}", push_boxes.borrow().len());
            i += 1;
        }

        for &pushed_box in &already_pushing {
            self.map.set_tile(pushed_box, '.');
            if self.wide {
                self.map.set_tile(pushed_box + Point2D(1, 0), '.');
            }
        }

        for &pushed_box in &already_pushing {
            let new_point = pushed_box + dir.as_point();
            if self.wide {
                self.map.set_tile(new_point, '[');
                self.map.set_tile(new_point + Point2D(1, 0), ']');
            } else {
                self.map.set_tile(new_point, 'O');
            }
        }

        self.robot = new_target_pos;
        self.map.set_tile(start_pos, '.');
        self.map.set_tile(new_target_pos, '@');
    }

    fn debug_display(&mut self) {
        println!("{}", self.map);
    }
}

impl FromStr for Warehouse {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map = CharMap::from_str(s)?;
        Ok(Self::new(map))
    }
}

/*
struct BoxPushes<'a> {
    map: &'a CharMap,
    dir: Direction,
    pushing_boxes: Vec<Point2D>,
    already_pushing: HashSet<Point2D>,
}
*/

fn parse_directions(s: &str) -> Vec<Direction> {
    s.chars()
        .filter(|ch| ch != &'\n')
        .map(|ch| match ch {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!(),
        })
        .collect()
}
