// ==== day 8

use crate::utils;
use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<&str> = s.trim().split(' ').collect();
        if parsed.len() != 2 {
            return Err(anyhow!("instruction parse error"));
        }
        let opcode = parsed[0];
        let operand = parsed[1].parse::<i32>()?;

        match opcode {
            "acc" => Ok(Instruction::Acc(operand)),
            "jmp" => Ok(Instruction::Jmp(operand)),
            "nop" => Ok(Instruction::Nop(operand)),
            _ => Err(anyhow!("unknown instruction")),
        }
    }
}

#[derive(Debug, Default)]
struct Console {
    pc: usize,                 // program counter
    hits: Vec<usize>,          // instruction hits
    accumulator: i32,          // accumulator (starts at 0)
    program: Vec<Instruction>, // program is just an array of instructions
}

impl FromStr for Console {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let memory: Vec<Instruction> = s
            .lines()
            .map(|l| Instruction::from_str(l).unwrap())
            .collect();
        Ok(Console {
            pc: 0,
            hits: vec![0; memory.len()],
            accumulator: 0,
            program: memory,
        })
    }
}

#[derive(PartialEq)]
enum Status {
    Running,
    Completed,
    Halted,
}

impl Console {
    fn reset(&mut self) {
        self.pc = 0;
        self.accumulator = 0;
        self.hits = vec![0; self.program.len()];
    }

    fn get_run_state(&self) -> Status {
        // check pc
        if !(0..self.program.len()).contains(&self.pc) {
            Status::Completed
        }
        // guard against infinite loops
        else if self.hits[self.pc] > 0 {
            Status::Halted
        } else {
            Status::Running
        }
    }

    fn step(&mut self) -> Status {
        // verify we're running
        if self.get_run_state() != Status::Running {
            return self.get_run_state();
        }

        // execute instruction at pc
        self.hits[self.pc] += 1;
        match self.program[self.pc] {
            Instruction::Acc(n) => {
                self.accumulator += n;
                self.pc += 1;
            }
            Instruction::Jmp(n) => {
                if n < 0 {
                    self.pc -= n.abs() as usize;
                } else {
                    self.pc += n as usize;
                }
            }
            Instruction::Nop(_) => {
                self.pc += 1;
            }
        }
        Status::Running
    }

    fn run(&mut self) -> i32 {
        self.reset();
        while self.step() == Status::Running {}
        self.accumulator
    }
}

pub fn run() -> Result<(i32, i32)> {
    let data = utils::read_file("data/input-08.txt")?;
    let answers = utils::read_i64s("data/output-08.txt")?;

    let mut console = Console::from_str(&data)?;
    println!("{:?}", console);

    let p1 = console.run();
    let p2 = 0;

    assert_eq!(p1, answers[0] as i32);
    assert_eq!(p2, answers[1] as i32);
    Ok((p1, p2))
}
