use anyhow::{self, Context};
use xmas::{display_result, point2d::Point2D};

fn main() -> anyhow::Result<()> {
    example_1()?;
    println!();
    part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn example_1() -> anyhow::Result<()> {
    println!("Example 1:");
    let input = std::fs::read_to_string("./input_example.txt").context("Error reading input file.")?;

    let result = day_18::calculate_path_after_bytes(&input, Point2D(7, 7), 12);
    display_result(&result);
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = day_18::calculate_path_after_bytes(&input, Point2D(71, 71), 1024);
    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = day_18::get_cutting_byte(&input, Point2D(71, 71), 1024);
    display_result(&result);
    Ok(())
}
