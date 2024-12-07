use std::ops::{Add, Mul};

type OperatorFns<'a> = [&'a dyn Fn(u64, u64) -> u64];

pub fn get_calibration_sum_1(input: &str) -> u64 {
    get_calibration_result(input, &[&Add::add, &Mul::mul])
}

pub fn get_calibration_sum_2(input: &str) -> u64 {
    get_calibration_result(input, &[&Add::add, &Mul::mul, &concatenate_nums])
}

fn get_calibration_result(input: &str, operators: &OperatorFns) -> u64 {
    input.lines()
        .map(|l| {
            let (expected, nums) = l.split_once(':').unwrap();
            let nums = nums.trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (expected.parse::<u64>().unwrap(), nums)
        })
        .filter(|(expected, nums)| can_be_solved(*expected, nums, operators))
        .map(|(expected, _)| expected)
        .sum()
}

fn can_be_solved(expected: u64, nums: &[u64], operators: &OperatorFns) -> bool {
    let first = *nums.first().unwrap();
    can_be_solved_recursive(expected, first, &nums[1..], operators)
}

fn can_be_solved_recursive(expected: u64, already_calculated: u64, nums: &[u64], operators: &OperatorFns) -> bool {
    let next = match nums.first().cloned() {
        Some(n) => n,
        None => return expected == already_calculated,
    };

    let rest = &nums[1..];
    operators.iter()
        .any(|op| {
            let calculated = op(already_calculated, next);
            if calculated > expected {
                return false;
            }
            can_be_solved_recursive(expected, calculated, rest, operators)
        })
}

fn concatenate_nums(lhs: u64, rhs: u64) -> u64 {
    let digits = rhs.ilog10();
    let mult = 10u64.pow(digits + 1);
    lhs * mult + rhs
}
