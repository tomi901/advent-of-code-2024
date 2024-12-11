use std::{collections::HashMap, num::ParseIntError};

// Cache was added on part 2

#[derive(Debug, Clone, Default)]
struct StoneCache {
    step_results: HashMap<u64, StoneResult>,
    /// Key = (num, steps)
    count_result_after_steps: HashMap<(u64, u64), usize>,
}

impl StoneCache {
    fn get_or_create_count_after(&mut self, num: u64, steps: u64) -> usize {
        if steps == 0 {
            return 1;
        }

        if let Some(&count) = self.count_result_after_steps.get(&(num, steps)) {
            // println!("Using cache!");
            return count;
        }

        let result = self.get_or_create_step_result(num).clone();
        let count = match result {
            StoneResult::Change(new_num) => {
                self.get_or_create_count_after(new_num, steps - 1)
            },
            StoneResult::Split(num1, num2) => {
                self.get_or_create_count_after(num1, steps - 1)
                + self.get_or_create_count_after(num2, steps - 1)
            },
        };
        self.count_result_after_steps.insert((num, steps), count);

        count
    }

    fn get_or_create_step_result(&mut self, num: u64) -> &StoneResult {
        self.step_results.entry(num).or_insert_with(|| {
            if num == 0 {
                return StoneResult::Change(1);
            }
    
            let stone_s = num.to_string();
            if stone_s.len() % 2 != 0 {
                return StoneResult::Change(num * 2024);
            }
    
            let split = stone_s.len() / 2;
            let stone_1 = stone_s[..split].parse().unwrap();
            let stone_2 = stone_s[split..].parse().unwrap();
            StoneResult::Split(stone_1, stone_2)
        })
    }
}

#[derive(Debug, Clone)]
enum StoneResult {
    Change(u64),
    Split(u64, u64),
}

pub fn calculate_stone_count(input: &str, blinks: u64) -> Result<usize, ParseIntError> {
    let stones = input.trim()
        .split_whitespace()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut cache = StoneCache::default();
    let mut count = 0;
    for stone in &stones {
        count += cache.get_or_create_count_after(*stone, blinks);
    }
    Ok(count)
}
