use std::{collections::HashSet, str::FromStr};
use crossterm::{event::KeyCode, execute};
use xmas::{direction::DIRECTIONS_8, map2d::{ByteMap, CharMap}, point2d::Point2D, wrap_val};
use regex_static::{once_cell::sync::Lazy, Regex, lazy_regex};

const DIGIT_REGEX: Lazy<Regex> = lazy_regex!(r"-?\d+");

pub fn calculate_safety_factor(input: &str, seconds: isize, space: Point2D) -> usize {
    let robots: Vec<_> = input.lines()
        .map(Robot::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    let mut quadrants = [0; 4];
    let middle = Point2D(space.0 / 2, space.1 / 2);
    for robot in &robots {
        let position = robot.predict_position(seconds, space);
        match position {
            Point2D(x, y) if x < middle.0 && y < middle.1 => quadrants[0] += 1,
            Point2D(x, y) if x > middle.0 && y < middle.1 => quadrants[1] += 1,
            Point2D(x, y) if x < middle.0 && y > middle.1 => quadrants[2] += 1,
            Point2D(x, y) if x > middle.0 && y > middle.1 => quadrants[3] += 1,
            _ => (),
        }
    }

    quadrants.into_iter()
        .reduce(|a, b| a * b)
        .unwrap_or(0)
}

pub fn find_christmas_tree_time(input: &str, space: Point2D) -> isize {
    let robots: Vec<_> = input.lines()
        .map(Robot::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    'outer: for seconds in 1.. {
        let mut positions = HashSet::new();
        for robot in &robots {
            let position = robot.predict_position(seconds, space);
            positions.insert(position);
        }

        for &position in &positions {
            if DIRECTIONS_8.map(|dir| position + dir).iter().any(|p| !positions.contains(p)) {
                // We found a lone position, probably not a picture
                continue 'outer;
            }
        }

        let mut map = CharMap::new_filled(space, '.');
        for &position in &positions {
            map.set_tile(position, '#');
        }

        println!("After {} second/s:", seconds);
        println!("{}", map);
    }

    todo!()
}

pub fn simulate_step_by_step(input: &str, space: Point2D) {
    let robots: Vec<_> = input.lines()
        .map(Robot::from_str)
        .collect::<Result<_, _>>()
        .unwrap();

    execute!(
        std::io::stdout(),
    ).unwrap();

    for seconds in 1.. {
        let mut map = CharMap::new_filled(space, '.');
        for robot in &robots {
            let position = robot.predict_position(seconds, space);
            map.set_tile(position, '#');
        }

        println!("After {} second/s:", seconds);
        println!("{}", map);

        /*
        println!("Press enter to continue...");
        loop {
            match crossterm::event::read().unwrap() {
                crossterm::event::Event::Key(key_event) => {
                    if key_event.code == KeyCode::Enter {
                        break;
                    }
                },
                _ => (),
            }
        }
        */
    }
}

struct Robot {
    start: Point2D,
    velocity: Point2D,
}

impl Robot {
    fn predict_position(&self, seconds: isize, space: Point2D) -> Point2D {
        let position = self.start + (self.velocity * seconds);
        let wrapped_position = Point2D(
            wrap_val(position.0, space.0),
            wrap_val(position.1, space.1),
        );
        wrapped_position
    }
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let binding = DIGIT_REGEX;
        let mut nums = binding.find_iter(s)
            .map(|m| m.as_str().parse::<isize>().unwrap());
        
        let start = Point2D(nums.next().unwrap(), nums.next().unwrap());
        let velocity = Point2D(nums.next().unwrap(), nums.next().unwrap());
        Ok(Self { start, velocity })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_around() {
        let robot = Robot { start: Point2D(4, 1), velocity: Point2D(2, -3) };

        let position = robot.predict_position(1, Point2D(11, 7));

        assert_eq!(position, Point2D(6, 5));
    }
}
