// ==== day 12

use crate::utils;
use anyhow::Result;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Mask {
        exes_mask: u64,
        ones_mask: u64,
        zeroes_mask: u64,
    },
    Write {
        address: u64,
        value: u64,
    },
}

fn parse_mask(s: &str) -> Instruction {
    let maskstr = &s[s.find(" = ").unwrap() + 3..];
    let ones_str = maskstr.replace('X', "0");
    let zeroes_str = maskstr
        .replace('1', "X")
        .replace('0', "1")
        .replace('X', "0");
    let exes_str = maskstr.replace('1', "0").replace('X', "1");
    Instruction::Mask {
        exes_mask: u64::from_str_radix(&exes_str, 2).unwrap(),
        ones_mask: u64::from_str_radix(&ones_str, 2).unwrap(),
        zeroes_mask: u64::from_str_radix(&zeroes_str, 2).unwrap(),
    }
}

fn parse_write(s: &str) -> Instruction {
    Instruction::Write {
        address: s[4..s.find(']').unwrap()].parse().unwrap(),
        value: s[s.find(" = ").unwrap() + 3..].parse().unwrap(),
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| l.trim())
        .map(|l| match &l[0..4] {
            "mask" => parse_mask(l),
            "mem[" => parse_write(l),
            _ => unreachable!("bad input"),
        })
        .collect()
}

fn simulate1(program: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let (mut cur_ones, mut cur_zeroes) = (0, 0);
    program.iter().for_each(|i| {
        match i {
            Instruction::Mask {
                ones_mask,
                zeroes_mask,
                ..
            } => {
                cur_ones = *ones_mask;
                cur_zeroes = *zeroes_mask;
            }
            Instruction::Write { address, value } => {
                memory.insert(*address, (value | cur_ones) & !cur_zeroes);
            }
        };
    });

    memory.values().sum()
}

#[inline(always)]
fn set_bit(n: u64, bit: usize, value: bool) -> u64 {
    if value {
        n | (1 << bit)
    } else {
        n & !(1 << bit)
    }
}

fn address_masks_from_float_mask(float_mask: u64) -> impl Iterator<Item = u64> {
    let possibilities = 2_u64.pow(float_mask.count_ones());
    let one_positions: Vec<usize> = (0..36).filter(|i| float_mask & (1 << i) != 0).collect();
    (0..possibilities).map(move |i| {
        one_positions
            .iter()
            .enumerate()
            .fold(0, |result, (j, pos)| {
                set_bit(result, *pos, (i & (1 << j)) != 0)
            })
    })
}

fn simulate2(program: &[Instruction]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let (mut cur_exes, mut cur_ones, mut cur_zeroes) = (0, 0, 0);
    program.iter().for_each(|i| {
        match i {
            Instruction::Mask {
                exes_mask,
                ones_mask,
                zeroes_mask,
            } => {
                cur_exes = *exes_mask;
                cur_ones = *ones_mask;
                cur_zeroes = *zeroes_mask;
            }
            Instruction::Write { address, value } => {
                let base_address = (*address | cur_ones) & !cur_exes;
                address_masks_from_float_mask(cur_exes)
                    .map(|a| a | base_address)
                    .for_each(|a| {
                        memory.insert(a, *value);
                    });
            }
        };
    });

    memory.values().sum()
}

// main
pub fn run() -> Result<(u64, u64)> {
    let data = utils::read_file("data/input-14.txt")?;
    let answers = utils::read_i64s("data/output-14.txt")?;

    let parsed = parse(&data);

    let p1 = simulate1(&parsed);
    let p2 = simulate2(&parsed);

    assert_eq!(p1, answers[0] as u64);
    assert_eq!(p2, answers[1] as u64);
    Ok((p1, p2))
}
