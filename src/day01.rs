// ==== day 1

use crate::utils;
use anyhow::{anyhow, Result};

fn part1(data: &[i64]) -> Result<i64> {
    for c in utils::combinations(2, data.len()) {
        let test = vec![data[c[0]], data[c[1]]];
        if test.iter().sum::<i64>() == 2020 {
            return Ok(test.iter().product());
        }
    }
    Err(anyhow!("no result found"))
}

fn part2(data: &[i64]) -> Result<i64> {
    for c in utils::combinations(3, data.len()) {
        let test = vec![data[c[0]], data[c[1]], data[c[2]]];
        if test.iter().sum::<i64>() == 2020 {
            return Ok(test.iter().product());
        }
    }
    Err(anyhow!("no result found"))
}

pub fn run() -> Result<(i64, i64)> {
    let data = utils::read_i64s("data/input-01.txt")?;
    let answers = utils::read_i64s("data/output-01.txt")?;

    let p1 = part1(&data)?;
    assert_eq!(p1, answers[0]);

    let p2 = part2(&data)?;
    assert_eq!(p2, answers[1]);

    Ok((p1, p2))
}
