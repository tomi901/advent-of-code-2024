use anyhow::{self, Context};
use xmas::display_result;

fn main() -> anyhow::Result<()> {
    // part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = day_11::calculate_stone_count(&input, 25);
    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = day_11::calculate_stone_count(&input, 75);
    display_result(&result);
    Ok(())
}