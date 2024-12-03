use std::str::FromStr;

use anyhow::Context;
use regex_static::{Regex, lazy_regex, once_cell::sync::Lazy};

// TODO: In the future consider using nom instead?
const INSTRUCTION_REGEX: Lazy<Regex> = lazy_regex!(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)");

pub fn execute_instructions(input: &str) -> Result<i64, anyhow::Error> {
    let instructions = Instruction::many_from_str(input).unwrap();
    Ok(instructions.iter()
        .map(Instruction::value)
        .sum())
}

pub fn execute_instructions_conditionally(input: &str) -> Result<i64, anyhow::Error> {
    let instructions = Instruction::many_from_str(input).unwrap();
    let mut enabled = true;
    Ok(instructions.iter()
        .map(|i| i.execute_conditionally(&mut enabled))
        .sum())
}


pub enum Instruction {
    Mul(i64, i64),
    Do,
    Dont,
}

impl Instruction {
    pub fn value(&self) -> i64 {
        match self {
            Instruction::Mul(a, b) => a * b,
            _ => 0
        }
    }

    pub fn execute_conditionally(&self, enabled: &mut bool) -> i64 {
        // dbg!(*enabled);
        match self {
            Instruction::Do => {
                *enabled = true;
                0
            },
            Instruction::Dont => {
                *enabled = false;
                // println!("Disabled!");
                0
            },
            _ => if *enabled { self.value() } else { 0 }
        }
    }

    pub fn many_from_str(s: &str) -> Result<Vec<Self>, anyhow::Error> {
        INSTRUCTION_REGEX
            .find_iter(s)
            .map(|m| Self::from_str(m.as_str()))
            .collect()
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse = INSTRUCTION_REGEX
            .captures(s)
            .context("Not an instruction")?;
        Ok(match parse.get(0).unwrap().as_str() {
            ins if ins.starts_with("mul") => {
                Self::Mul(
                    parse.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    parse.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                )
            },
            ins if ins.starts_with("don't") => Self::Dont,
            ins if ins.starts_with("do") => Self::Do,
            _ => unreachable!(),
        })
    }
}
