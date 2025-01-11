use std::fmt::Display;

struct Program {
    registers: Registers,
    instructions: Vec<u8>,
    instr_pointer: usize,
    output: Vec<u8>,
}
impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {}, B: {}, C: {}: P: {} -> {}",
            self.registers.a,
            self.registers.b,
            self.registers.c,
            self.instr_pointer,
            self.output
                .iter()
                .fold("".to_string(), |acc, i| acc + &i.to_string())
        )
    }
}
impl Program {
    fn new(a: u64, b: u64, c: u64, instructions: &[u8]) -> Program {
        Program {
            registers: Registers { a, b, c },
            instructions: instructions.to_vec(),
            instr_pointer: 0,
            output: vec![],
        }
    }

    fn step(&mut self) -> Option<&mut Self> {
        let new_inst_point = self.registers.apply(
            self.instructions.get(self.instr_pointer)?,
            self.instructions.get(self.instr_pointer + 1)?,
            &mut self.output,
        );
        if let Some(pointer) = new_inst_point {
            self.instr_pointer = pointer;
        } else {
            self.instr_pointer += 2;
        }
        // println!("{}", self);
        Some(self)
    }

    fn run(&mut self) -> Vec<u8> {
        while self.step().is_some() {}
        self.output.clone()
    }
}

struct Registers {
    a: u64,
    b: u64,
    c: u64,
}
impl Registers {
    fn combo_value(&self, combo: &u8) -> u64 {
        match combo {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unreachable!("Combo operator 7 should not appear in valid program"),
            _ => unreachable!(),
        }
    }

    fn apply(&mut self, instruction: &u8, arg: &u8, output: &mut Vec<u8>) -> Option<usize> {
        match instruction {
            0 => {
                self.a /= u64::pow(2, self.combo_value(arg) as u32);
                None
            }
            1 => {
                self.b ^= *arg as u64;
                None
            }
            2 => {
                self.b = self.combo_value(arg) % 8;
                None
            }
            3 => (self.a != 0).then_some(*arg as usize),
            4 => {
                self.b ^= self.c;
                None
            }
            5 => {
                output.push((self.combo_value(arg) % 8) as u8);
                None
            }
            6 => {
                self.b = self.a / u64::pow(2, self.combo_value(arg) as u32);
                None
            }
            7 => {
                self.c = self.a / u64::pow(2, self.combo_value(arg) as u32);
                None
            }
            _ => unreachable!("Invalid OP code"),
        }
    }
}

fn parse_input(input: &str) -> Program {
    let mut lines = input.lines();
    let a = lines.next().unwrap().split_at(12).1.parse::<u64>().unwrap();
    let b = lines.next().unwrap().split_at(12).1.parse::<u64>().unwrap();
    let c = lines.next().unwrap().split_at(12).1.parse::<u64>().unwrap();
    lines.next();
    let instructions: Vec<u8> = lines
        .next()
        .unwrap()
        .split_once(" ")
        .unwrap()
        .1
        .split(",")
        .map(|i| i.parse::<u8>().unwrap())
        .collect();
    Program::new(a, b, c, &instructions)
}

pub fn process_part1(input: &str) -> String {
    parse_input(input)
        .run()
        .into_iter()
        .map(|i| i.to_string())
        .reduce(|acc, c| acc + "," + &c)
        .unwrap()
}

pub fn process_part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_computer1() {
        assert_eq!(
            Program::new(0, 0, 9, &vec![2, 6])
                .step()
                .unwrap()
                .registers
                .b,
            1
        );
    }

    #[test]
    fn test_computer2() {
        assert_eq!(
            Program::new(10, 0, 0, &vec![5, 0, 5, 1, 5, 4]).run(),
            vec![0, 1, 2]
        );
    }
    #[test]
    fn test_computer3() {
        let mut program = Program::new(2024, 0, 0, &vec![0, 1, 5, 4, 3, 0]);
        assert_eq!(program.run(), vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(program.registers.a, 0);
    }
    #[test]
    fn test_computer4() {
        assert_eq!(
            Program::new(0, 29, 0, &vec![1, 7])
                .step()
                .unwrap()
                .registers
                .b,
            26
        );
    }
    #[test]
    fn test_computer5() {
        assert_eq!(
            Program::new(0, 2024, 43690, &vec![4, 0])
                .step()
                .unwrap()
                .registers
                .b,
            44354
        );
    }

    #[test]
    fn test_computer6() {
        let mut program = Program::new(2024, 0, 0, &vec![0, 1, 3, 0]);
        assert_eq!(program.step().unwrap().registers.a, 1012);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 506);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 253);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 126);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 63);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 31);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 15);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 7);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 3);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 1);
        program.step();
        assert_eq!(program.step().unwrap().registers.a, 0);
    }

    #[test]
    fn test_process_part1() {
        assert_eq!(process_part1(EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_process_part2() {
        assert_eq!(process_part2(EXAMPLE2), 117440);
    }
}
