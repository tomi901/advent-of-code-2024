use anyhow::{self, Context};
use xmas::display_result;

fn main() -> anyhow::Result<()> {
    example_1()?;
    println!();
    // part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn example_1() -> anyhow::Result<()> {
    println!("Example 1:");
    let input = std::fs::read_to_string("./input_example.txt").context("Error reading input file.")?;

    let result = day_20::calculate_best_shortcuts(&input, 0, 2);
    display_result(&result);
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = day_20::calculate_best_shortcuts(&input, 100,  2);
    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}
