use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy)]
enum Register {
    RegA,
    RegB,
    RegC,
}

#[derive(Debug, Clone, Copy)]
struct LiteralOp {
    value: u8,
}

#[derive(Debug, Clone, Copy)]
enum ComboOp {
    Value(u8),
    RegisterValue(Register),
}

impl ComboOp {
    fn try_from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(ComboOp::Value(0)),
            1 => Ok(ComboOp::Value(1)),
            2 => Ok(ComboOp::Value(2)),
            3 => Ok(ComboOp::Value(3)),
            4 => Ok(ComboOp::RegisterValue(Register::RegA)),
            5 => Ok(ComboOp::RegisterValue(Register::RegB)),
            6 => Ok(ComboOp::RegisterValue(Register::RegC)),
            _ => bail!("Invalid combo operand value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Div { target: Register, operand: ComboOp },
    Bxl(LiteralOp),
    Bst(ComboOp),
    Jnz(LiteralOp),
    Bxc,
    Out(ComboOp),
}

impl Instruction {
    fn try_from_u8(instruction: u8, operand: u8) -> Result<Self> {
        let op = || ComboOp::try_from_u8(operand);
        let res = match instruction {
            0 => Instruction::Div {
                target: Register::RegA,
                operand: op()?,
            },
            1 => Instruction::Bxl(LiteralOp { value: operand }),
            2 => Instruction::Bst(op()?),
            3 => Instruction::Jnz(LiteralOp { value: operand }),
            4 => Instruction::Bxc,
            5 => Instruction::Out(op()?),
            6 => Instruction::Div {
                target: Register::RegB,
                operand: op()?,
            },
            7 => Instruction::Div {
                target: Register::RegC,
                operand: op()?,
            },
            _ => bail!("Invalid instruction value {}", instruction),
        };

        Ok(res)
    }
}

#[derive(Clone)]
struct Rom {
    reg_a: u128,
    reg_b: u128,
    reg_c: u128,
    program: Vec<u8>,
}

struct Emulator {
    pc: usize,
    reg_a: u128,
    reg_b: u128,
    reg_c: u128,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Emulator {
    fn from_rom(rom: Rom) -> Self {
        Self {
            pc: 0,
            reg_a: rom.reg_a,
            reg_b: rom.reg_b,
            reg_c: rom.reg_c,
            program: rom.program,
            output: Vec::new(),
        }
    }

    fn run(&mut self) -> Result<Vec<u8>> {
        loop {
            if self.pc >= self.program.len() {
                break;
            }
            let instruction = self.read_instruction()?;
            self.pc += 2;

            self.handle_instruction(instruction);
        }

        Ok(self.output.clone())
    }

    fn read_instruction(&self) -> Result<Instruction> {
        let raw_inst = self.program[self.pc];
        let raw_op = self.program[self.pc + 1];

        Instruction::try_from_u8(raw_inst, raw_op)
    }

    fn handle_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Div { target, operand } => self.handle_div(target, operand),
            Instruction::Bxl(operand) => self.bxl(operand),
            Instruction::Bst(operand) => self.bst(operand),
            Instruction::Jnz(operand) => self.jnz(operand),
            Instruction::Bxc => self.bxc(),
            Instruction::Out(operand) => self.out(operand),
        }
    }

    fn handle_div(&mut self, target: Register, operand: ComboOp) {
        let op_value = self.get_operand_value(operand);
        let denominator = 2_u128.pow(op_value.try_into().unwrap());
        let value = self.reg_a / denominator;
        match target {
            Register::RegA => self.reg_a = value,
            Register::RegB => self.reg_b = value,
            Register::RegC => self.reg_c = value,
        }
    }

    fn bxl(&mut self, op: LiteralOp) {
        let op_value = op.value as u128;
        self.reg_b ^= op_value;
    }

    fn bst(&mut self, op: ComboOp) {
        let op_value = self.get_operand_value(op);
        self.reg_b = op_value % 8;
    }

    fn jnz(&mut self, op: LiteralOp) {
        if self.reg_a == 0 {
            return;
        }

        self.pc = op.value as usize;
    }

    fn bxc(&mut self) {
        self.reg_b ^= self.reg_c;
    }

    fn out(&mut self, op: ComboOp) {
        let op_value = self.get_operand_value(op);
        let res = op_value % 8;
        self.output.push(res as u8);
    }

    fn get_register(&self, reg: Register) -> u128 {
        match reg {
            Register::RegA => self.reg_a,
            Register::RegB => self.reg_b,
            Register::RegC => self.reg_c,
        }
    }

    fn get_operand_value(&self, op: ComboOp) -> u128 {
        match op {
            ComboOp::Value(i) => i as u128,
            ComboOp::RegisterValue(register) => self.get_register(register),
        }
    }
}

fn parse_input(input: &str) -> Result<Rom> {
    let blocks = input.split("\r\n\r\n").collect_vec();
    if blocks.len() != 2 {
        bail!("exepcted 2 blocks");
    }

    let values = blocks[0]
        .lines()
        .map(parse_register)
        .collect::<Result<Vec<_>>>()?;

    let program = parse_program(blocks[1])?;

    let rom = Rom {
        reg_a: values[0],
        reg_b: values[1],
        reg_c: values[2],
        program,
    };

    Ok(rom)
}

fn parse_register(line: &str) -> Result<u128> {
    let reg: Lazy<Regex> = Lazy::new(|| Regex::new(r"Register .: (\d+)").unwrap());
    let caputure = reg
        .captures(line)
        .ok_or(anyhow!("Invalid register input"))?;

    Ok(caputure[1].parse::<u128>()?)
}

fn parse_program(line: &str) -> Result<Vec<u8>> {
    let line = line.replace("Program: ", "");
    let instructions = line
        .split(',')
        .map(|str| str.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::{Emulator, Rom};

    #[test]
    fn test_bst_instruction() {
        let rom = Rom {
            reg_a: 0,
            reg_b: 0,
            reg_c: 9,
            program: vec![2, 6],
        };
        let mut emu = Emulator::from_rom(rom);
        emu.run().unwrap();
        assert_eq!(emu.reg_b, 1)
    }

    #[test]
    fn test_bxl_instruction() {
        let rom = Rom {
            reg_a: 0,
            reg_b: 29,
            reg_c: 0,
            program: vec![1, 7],
        };
        let mut emu = Emulator::from_rom(rom);
        emu.run().unwrap();
        assert_eq!(emu.reg_b, 26)
    }

    #[test]
    fn test_bxc_instruction() {
        let rom = Rom {
            reg_a: 0,
            reg_b: 2024,
            reg_c: 43690,
            program: vec![4, 0],
        };
        let mut emu = Emulator::from_rom(rom);
        emu.run().unwrap();
        assert_eq!(emu.reg_b, 44354)
    }

    #[test]
    fn test_simple_output() {
        let rom = Rom {
            reg_a: 10,
            reg_b: 0,
            reg_c: 0,
            program: vec![5, 0, 5, 1, 5, 4],
        };
        let mut emu = Emulator::from_rom(rom);
        let out = emu.run().unwrap();
        assert_eq!(out, [0, 1, 2]);
    }

    #[test]
    fn test_long_output() {
        let rom = Rom {
            reg_a: 2024,
            reg_b: 0,
            reg_c: 0,
            program: vec![0, 1, 5, 4, 3, 0],
        };
        let mut emu = Emulator::from_rom(rom);
        let out = emu.run().unwrap();
        assert_eq!(out, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(emu.reg_a, 0);
    }

    #[test]
    fn test_translation() {
        let a: u64 = 30553366 / 8 / 8 / 8;

        let b = ((((a % 8) ^ 1) ^ (a / 2_u64.pow(((a % 8_u64) ^ 1) as u32))) ^ 4) % 8;

        // ((((a % 8) ^ 1) ^ (a / 2_u64.pow(((a % 8_u64) ^ 1) as u32))) ^ 4) % 8;
        // ((((a % 8) ^ 1) ^ (a / 2_u64.pow(((a % 8_u64) ^ 1) as u32))) ^ 4) % 8;
        // ((((a % 8) ^ 1) ^ (a / 2_u64.pow(((a % 8_u64) ^ 1) as u32))) ^ 4) % 8;

        // (1,3,7,4,6,4,2,3,5);
        assert_eq!(b, 4);
    }
}
