use pathfinding::directed::astar;
use xmas::{direction::DIRECTIONS, map2d::CharMap, point2d::Point2D};

pub fn calculate_path_after_bytes(input: &str, map_size: Point2D, bytes: usize) -> u64 {
    let map = {
        let mut map = CharMap::new_filled(map_size, '.');
        let corrupt_at = input.lines()
            .map(parse_point)
            .take(bytes);

        for point in corrupt_at {
            // println!("{}", point);
            map.set_tile(point, '#');
        }
        map
    };
    // println!("Map:\n{}", map);

    let (_, cost) = try_get_path(&map).unwrap();

    // println!("Path: {:?}", path);

    cost
}

pub fn get_cutting_byte(input: &str, map_size: Point2D, skip: usize) -> Point2D {
    let mut map = CharMap::new_filled(map_size, '.');
    let corrupt_at = input.lines().map(parse_point);

    let mut i = 0;
    for point in corrupt_at {
        map.set_tile(point, '#');
        i += 1;
        if i <= skip {
            continue;
        }

        let result = try_get_path(&map);
        if result.is_none() {
            return point;
        }
    }

    panic!("Not found!");
}

fn try_get_path(map: &CharMap) -> Option<(Vec<Point2D>, u64)> {
    let target = map.size() - Point2D(1, 1);
    astar::astar(
        &Point2D::ZERO,
        |&from| DIRECTIONS.iter()
            .map(move |dir| from + dir.as_point())
            .filter(|&p| map.get_tile(p).is_some_and(|t| t == &'.'))
            .map(|point| (point, 1)),
        |_| 0,
        |&bc| bc == target,
    )
}

fn parse_point(s: &str) -> Point2D {
    let (x, y) = s.split_once(',').unwrap();
    Point2D(x.parse().unwrap(), y.parse().unwrap())
}
