use std::str::FromStr;
use genawaiter::{rc::gen, yield_};
use regex_static::{once_cell::sync::Lazy, Regex, lazy_regex};

const DIGIT_REGEX: Lazy<Regex> = lazy_regex!(r"\d+");

type TinyByte = u8;
pub type Register = u64;

pub const INS_ADV: TinyByte = 0;
pub const INS_BXL: TinyByte = 1;
pub const INS_BST: TinyByte = 2;
pub const INS_JNZ: TinyByte = 3;
pub const INS_BXC: TinyByte = 4;
pub const INS_OUT: TinyByte = 5;
pub const INS_BDV: TinyByte = 6;
pub const INS_CDV: TinyByte = 7;

pub fn execute_all_instructions(input: &str) -> Vec<TinyByte> {
    let (mut computer, program) = parse_input(input);
    computer.execute(&program).collect()
}

pub fn calculate_required_a_value(input: &str) -> Register {
    let (_, program) = parse_input(input);
    println!("Target output = {:?}", program);

    try_to_find_valid_a(0, &program).unwrap()
}

// WON'T work with example input, use brute force from previous commit
pub fn try_to_find_valid_a(a_subset: Register, program: &[TinyByte]) -> Option<Register> {
    // println!("Testing: {}", a_subset);
    let subset_min = if a_subset == 0 { 1 } else { a_subset };
    for test_a in subset_min..(a_subset + 8) {
        let output: Vec<_> = Computer::with_a(test_a).execute(program).collect();
        let last_portion = &program[(program.len() - output.len())..];
        // println!("{} Match: {:?} matches with last portion of {:?}", test_a, output, last_portion);
        if last_portion != output {
            continue;
        }

        if program.len() == output.len() {
            return Some(test_a);
        } else if let Some(val) = try_to_find_valid_a(test_a * 8, program) {
            return Some(val);
        }
    }
    None
}

fn parse_input(input: &str) -> (Computer, Vec<TinyByte>) {
    let (computer_s, program_s) = input.split_once("\n\n").unwrap();
    let computer = Computer::from_str(computer_s).unwrap();
    let program: Vec<TinyByte> = program_s.trim_start_matches("Program:")
        .trim()
        .split(',')
        .map(|ins| ins.parse::<TinyByte>())
        .collect::<Result<_, _>>()
        .unwrap();

    (computer, program)
}

pub fn output_result(result: &[TinyByte]) {
    let mut first = true;
    for n in result {
        if !first {
            print!(",");
        } else {
            first = false;
        }
        print!("{n}");
    }
    println!();
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: Register,
    register_b: Register,
    register_c: Register,
}

impl Computer {
    pub fn new(register_a: Register, register_b: Register, register_c: Register) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
        }
    }

    pub fn with_a(register_a: Register) -> Self {
        Self::new(register_a, 0, 0)
    }

    pub fn combo_value(&self, val: TinyByte) -> Register {
        match val {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => unreachable!(),
            _ => val as Register,
        }
    }

    fn dv_op(&self, operand: TinyByte) -> Register {
        let power = self.combo_value(operand) as u32;
        let result = self.register_a.checked_shr(power).unwrap_or(0);
        // println!("{} / 2 ^ {} = {}", self.register_a, power, result);
        result
    }

    pub fn execute<'a>(&'a mut self, instructions: &'a [TinyByte]) -> impl Iterator<Item = TinyByte> + 'a {
        gen!({
            let mut program_counter: usize = 0;
            while program_counter < instructions.len() {
                let operator = instructions[program_counter];
                let operand = instructions.get(program_counter + 1).cloned().unwrap_or(0);
                match (operator, operand) {
                    (INS_ADV, operand) => self.register_a = self.dv_op(operand),
                    (INS_BXL, operand) => self.register_b ^= operand as u64,
                    (INS_BST, operand) => self.register_b = self.combo_value(operand) % 8,
                    (INS_JNZ, operand) => {
                        if self.register_a != 0 {
                            program_counter = operand as usize;
                            continue;
                        }
                    }
                    (INS_BXC, _) => self.register_b ^= self.register_c,
                    (INS_OUT, operand) => {
                        let val = self.combo_value(operand) as u8 % 8;
                        yield_!(val);
                        // println!("out: {:?}", output);
                    },
                    (INS_BDV, operand) => self.register_b = self.dv_op(operand),
                    (INS_CDV, operand) => self.register_c = self.dv_op(operand),
                    (ins, _) => unreachable!("Instruction {ins} not supported"),
                }
                program_counter += 2;
            }
        })
        .into_iter()
    }
}

impl FromStr for Computer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines()
            .map(|l| DIGIT_REGEX.find(l).unwrap().as_str().parse::<Register>().unwrap());
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();
        Ok(Self::new(a, b, c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut computer = Computer::new(0, 0, 9);

        let _ = computer.execute(&[2, 6]).collect::<Vec<_>>();

        assert_eq!(computer.register_b, 1);
    }

    #[test]
    fn example_2() {
        let mut computer = Computer::new(10, 0, 0);

        let output: Vec<_> = computer.execute(&[5, 0, 1, 5, 4]).collect();

        assert_eq!(output, [0, 1, 2]);
    }

    #[test]
    fn example_3() {
        let mut computer = Computer::new(2024, 0, 0);

        let output: Vec<_> = computer.execute(&[0, 1, 5, 4, 3, 0]).collect();

        assert_eq!(output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    }

    #[test]
    fn example_4() {
        let mut computer = Computer::new(0, 29, 0);

        let _ = computer.execute(&[1,7]).collect::<Vec<_>>();

        assert_eq!(computer.register_b, 26);
    }

    #[test]
    fn example_5() {
        let mut computer = Computer::new(0, 2024, 43690);

        let _ = computer.execute(&[4,0]).collect::<Vec<_>>();

        assert_eq!(computer.register_b, 44354);
    }
}
