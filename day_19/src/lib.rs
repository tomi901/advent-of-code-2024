use std::{cell::RefCell, collections::HashMap, convert::Infallible, str::FromStr};

pub fn get_possible_design_count(input: &str) -> usize {
    let (patterns_s, designs) = input.split_once("\n\n").unwrap();
    let patterns = TowelPatterns::from_str(patterns_s).unwrap();

    designs.lines()
        .map(str::trim)
        .filter(|&design| patterns.design_is_possible_with_cache(design))
        .count()
}

struct TowelPatterns<'a> {
    patterns: Vec<String>,
    cache: RefCell<HashMap<&'a str, bool>>,
}

impl<'a> TowelPatterns<'a> {
    pub fn new(patterns: Vec<String>) -> Self {
        Self { patterns, cache: Default::default() }
    }

    pub fn design_is_possible_with_cache(&self, design: &'a str) -> bool {
        if let Some(&cached_result) = self.cache.borrow().get(design) {
            return cached_result;
        }

        let result = {
                for pattern in &self.patterns {
                if !design.starts_with(pattern) {
                    continue;
                }

                if pattern.len() == design.len() {
                    return true;
                }

                let any_children_matches = self.design_is_possible_with_cache(&design[pattern.len()..]);
                if any_children_matches {
                    return true;
                }
            }
            false
        };
        self.cache.borrow_mut().insert(design, result);
        result
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
