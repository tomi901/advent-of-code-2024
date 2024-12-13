use std::str::FromStr;

use num::Rational64;
use xmas::point2d::Point2D;
use regex_static::{once_cell::sync::Lazy, Regex, lazy_regex};

const DIGIT_REGEX: Lazy<Regex> = lazy_regex!(r"\d+");

const A_COST: u64 = 3;
const B_COST: u64 = 1;

#[derive(Debug, Clone, Copy)]
struct Line {
    slope: Rational64,
    bias: Rational64,
}

impl Line {
    fn eval(&self, x: Rational64) -> Rational64 {
        x * self.slope + self.bias
    }

    fn intersection_with(&self, other: &Self) -> Option<Rational64> {
        let new_slope = self.slope - other.slope;
        if new_slope != Rational64::ZERO {
            let x = (other.bias - self.bias) / new_slope;
            // println!("Found intersection at {}", x);
            Some(x)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    button_a: Point2D,
    button_b: Point2D,
    prize: Point2D,
}

impl Game {
    fn many_from_str(s: &str, prize_adder: u64) -> Result<Vec<Self>, anyhow::Error> {
        let prize_adder = prize_adder as isize;
        s.split("\n\n")
            .map(Self::from_str)
            .map(|r|
                r.map(|g| Game { prize: g.prize + Point2D(prize_adder, prize_adder), ..g }))
            .collect()
    }

    fn lowest_token_cost(&self) -> Option<u64> {
        // println!("{:?}", self);
        self.lowest_button_presses()
            .map(|(a, b)| a * A_COST + b * B_COST)
    }

    fn lowest_button_presses(&self) -> Option<(u64, u64)> {
        let x_line = Self::line_for_prize(self.button_a.0 as u64, self.button_b.0 as u64, self.prize.0 as u64);
        let y_line = Self::line_for_prize(self.button_a.1 as u64, self.button_b.1 as u64, self.prize.1 as u64);
        // dbg!((&x_line, &y_line));

        let a = match x_line.intersection_with(&y_line) {
            Some(x) if x.is_integer() => x,
            _ => return None,
        };

        let b = match x_line.eval(a) {
            y if y.is_integer() => y.to_integer() as u64,
            _ => return None,
        };

        Some((a.to_integer() as u64, b))
    }

    fn line_for_prize(a: u64, b: u64, p: u64) -> Line {
        let slope = -Rational64::new(a as i64, b as i64);
        let bias = Rational64::new(p as i64, b as i64);
        Line { slope, bias }
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

pub fn lowest_token_cost(input: &str, prize_adder: u64) -> u64 {
    let games = Game::many_from_str(input, prize_adder).unwrap();

    // println!("Games: {:?}", games);
    games.iter()
        .map(|g| g.lowest_token_cost().unwrap_or_default())
        .sum()
}
