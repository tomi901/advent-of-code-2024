use std::str::FromStr;

use xmas::point2d::Point2D;
use regex_static::{once_cell::sync::Lazy, Regex, lazy_regex};

const DIGIT_REGEX: Lazy<Regex> = lazy_regex!(r"\d+");

const A_COST: u64 = 3;
const B_COST: u64 = 1;
const PRESS_LIMIT: u64 = 100;

#[derive(Debug, Clone)]
struct Game {
    button_a: Point2D,
    button_b: Point2D,
    prize: Point2D,
}

impl Game {
    fn many_from_str(s: &str) -> Result<Vec<Self>, anyhow::Error> {
        s.split("\n\n")
            .map(Self::from_str)
            .collect()
    }

    fn lowest_token_cost(&self) -> Option<u64> {
        // println!("{:?}", self);
        self.lowest_button_presses()
            .map(|(a, b)| a * A_COST + b * B_COST)
    }

    fn lowest_button_presses(&self) -> Option<(u64, u64)> {
        for a in 0..=PRESS_LIMIT {
            let a_position = self.button_a * a as isize;
            let diff = self.prize - a_position;
            if diff.0 < 0 || diff.1 < 0 {
                // Moved too far away
                return None;
            }

            if diff == Point2D(0, 0) {
                return Some((a, 0));
            }

            if diff.0 % self.button_b.0 != 0 || diff.1 % self.button_b.1 != 0 {
                continue;
            }

            let move_amount = diff.0 / self.button_b.0;
            if self.button_b.1 * move_amount != diff.1 {
                continue;
            }

            return Some((a, move_amount as u64))
        }
        None
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let button_a = parse_coords(lines.next());
        let button_b = parse_coords(lines.next());
        let prize = parse_coords(lines.next());
        Ok(Game { button_a, button_b, prize })
    }
}

fn parse_coords(s: Option<&str>) -> Point2D {
    let s = s.unwrap();
    // println!("Parsing: {}", s);
    let binding = DIGIT_REGEX;
    let mut matches = binding.find_iter(s);
    let x = matches.next().unwrap().as_str().parse().unwrap();
    let y = matches.next().unwrap().as_str().parse().unwrap();
    Point2D(x, y)
}

pub fn lowest_token_cost(input: &str) -> u64 {
    let games = Game::many_from_str(input).unwrap();

    // println!("Games: {:?}", games);
    games.iter()
        .map(|g| g.lowest_token_cost().unwrap_or_default())
        .sum()
}
