use std::str::FromStr;
use xmas::{point2d::Point2D, wrap_val};
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
