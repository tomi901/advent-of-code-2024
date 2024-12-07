
pub fn get_calibration_sum_1(input: &str) -> i64 {
    input.lines()
        .map(|l| {
            let (expected, nums) = l.split_once(':').unwrap();
            let nums = nums.trim()
                .split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            (expected.parse::<i64>().unwrap(), nums)
        })
        .filter(|(expected, nums)| can_be_solved(*expected, nums))
        .map(|(expected, _)| expected)
        .sum()
}

fn can_be_solved(expected: i64, nums: &[i64]) -> bool {
    let first = *nums.first().unwrap();
    can_be_solved_recursive(expected, first, &nums[1..])
}

fn can_be_solved_recursive(expected: i64, already_calculated: i64, nums: &[i64]) -> bool {
    let next = match nums.first().cloned() {
        Some(n) => n,
        None => return expected == already_calculated,
    };

    let rest = &nums[1..];
    can_be_solved_recursive(expected, already_calculated + next, rest)
        || can_be_solved_recursive(expected, already_calculated * next, rest)
        || can_be_solved_recursive(expected, concatenate_nums(already_calculated, next), rest)
}

fn can_be_solved_2(expected: i64, nums: &[i64]) -> bool {
    let first = *nums.first().unwrap();
    can_be_solved_recursive(expected, first, &nums[1..])
}

fn can_be_solved_recursive_2(expected: i64, already_calculated: i64, nums: &[i64]) -> bool {
    let next = match nums.first().cloned() {
        Some(n) => n,
        None => return expected == already_calculated,
    };

    let rest = &nums[1..];
    can_be_solved_recursive_2(expected, already_calculated + next, rest)
        || can_be_solved_recursive_2(expected, already_calculated * next, rest)
        || can_be_solved_recursive_2(expected, concatenate_nums(already_calculated, next), rest)
}

fn concatenate_nums(lhs: i64, rhs: i64) -> i64 {
    let s = format!("{lhs}{rhs}");
    s.parse().unwrap()
}
