// ==== day 1

use crate::utils;
use anyhow::{anyhow, Result};

fn day01_part1(data: &[i64]) -> Result<i64> {
    for i in 0..data.len() - 2 {
        for j in i + 1..data.len() - 1 {
            if data[i] + data[j] == 2020 {
                return Ok(data[i] * data[j]);
            }
        }
    }
    Err(anyhow!("no result found"))
}

fn day01_part2(data: &[i64]) -> Result<i64> {
    for i in 0..data.len() - 3 {
        for j in i + 1..data.len() - 2 {
            for k in j + 1..data.len() - 1 {
                if data[i] + data[j] + data[k] == 2020 {
                    return Ok(data[i] * data[j] * data[k]);
                }
            }
        }
    }
    Err(anyhow!("no result found"))
}

pub fn run() -> Result<(i64, i64)> {
    let data = utils::read_i64s("data/input-01.txt")?;
    let answers = utils::read_i64s("data/output-01.txt")?;

    let p1 = day01_part1(&data)?;
    assert_eq!(p1, answers[0]);

    let p2 = day01_part2(&data)?;
    assert_eq!(p2, answers[1]);

    Ok((p1, p2))
}
