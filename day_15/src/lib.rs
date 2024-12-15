use std::str::FromStr;
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

    warehouse.debug_display();
    for dir in movements {
        warehouse.move_robot(dir);
        // warehouse.debug_display();
    }

    warehouse.box_gps_sum()
}

struct Warehouse {
    map: CharMap,
    robot: Point2D,
}

impl Warehouse {
    pub fn new(map: CharMap) -> Self {
        let robot = map.iter_with_points()
            .find(|&(_, tile)| tile == &'@')
            .map(|(p, _)| p)
            .unwrap();
        Self { map, robot }
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
        Self::new(new_map)
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
        
        let mut push_pos = new_target_pos;
        let mut moved_boxes = 0;
        loop {
            match self.map.get_tile(push_pos) {
                Some('O') => {
                    moved_boxes += 1;
                    push_pos += dir.as_point();
                },
                // We cannot move
                Some('#') => return,
                Some(_) => break,
                None => unreachable!(),
            }
        }

        self.robot = new_target_pos;
        self.map.set_tile(start_pos, '.');
        self.map.set_tile(new_target_pos, '@');
        
        push_pos = new_target_pos + dir.as_point();
        for _ in 0..moved_boxes {
            self.map.set_tile(push_pos, 'O');
            push_pos += dir.as_point();
        }
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
