use anyhow::{self, Context};
use day_02::Count;
use xmas::display_result;

fn main() -> anyhow::Result<()> {
    part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = day_02::count_safe(&input)?;
    display_count_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = day_02::count_safe_with_tolerance(&input)?;
    display_count_result(&result);
    Ok(())
}

fn display_count_result(count: &Count) {
    println!();
    println!("Safe: {}/{}", count.count, count.total);
    display_result(&count.count);
}
