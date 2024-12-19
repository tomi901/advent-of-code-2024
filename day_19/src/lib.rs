use std::{cell::RefCell, collections::HashMap, convert::Infallible, str::FromStr};

pub fn get_possible_design_count(input: &str) -> usize {
    let (patterns_s, designs) = input.split_once("\n\n").unwrap();
    let patterns = TowelPatterns::from_str(patterns_s).unwrap();

    designs.lines()
        .map(str::trim)
        .filter(|&design| patterns.get_possible_designs(design) > 1)
        .count()
}

pub fn get_total_amount_of_combinations(input: &str) -> usize {
    let (patterns_s, designs) = input.split_once("\n\n").unwrap();
    let patterns = TowelPatterns::from_str(patterns_s).unwrap();

    designs.lines()
        .map(str::trim)
        .map(|design| patterns.get_possible_designs(design))
        .sum()
}

struct TowelPatterns<'a> {
    patterns: Vec<String>,
    cache: RefCell<HashMap<&'a str, usize>>,
}

impl<'a> TowelPatterns<'a> {
    pub fn new(patterns: Vec<String>) -> Self {
        Self { patterns, cache: Default::default() }
    }

    pub fn get_possible_designs(&self, design: &'a str) -> usize {
        if let Some(&cached_result) = self.cache.borrow().get(design) {
            return cached_result;
        }

        let mut possible_designs = 0;
        for pattern in &self.patterns {
            if !design.starts_with(pattern) {
                continue;
            }

            if pattern.len() == design.len() {
                possible_designs += 1;
                continue;
            }

            let possible_rest_designs = self.get_possible_designs(&design[pattern.len()..]);
            possible_designs += possible_rest_designs;
        }
        self.cache.borrow_mut().insert(design, possible_designs);
        possible_designs
    }
}

impl<'a> FromStr for TowelPatterns<'a> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let patterns = s.split(',')
            .map(str::trim)
            .map(str::to_string)
            .collect();
        Ok(Self::new(patterns))
    }
}
