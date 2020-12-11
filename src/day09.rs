// ==== day 9

use crate::utils;
use anyhow::{anyhow, Result};

fn is_bad(data: &Vec<i64>, n: usize, i: usize) -> Result<i64> {
    let prev_n = &data[i - n..i];
    if !utils::combinations(2, prev_n.len()).any(|c| data[i] == prev_n[c[0]] + prev_n[c[1]]) {
        Ok(data[i])
    } else {
        Err(anyhow!("no result found"))
    }
}

fn find_first_bad(data: &Vec<i64>, n: usize, start: usize) -> Result<i64> {
    for i in start..data.len() {
        let ret = is_bad(data, n, i);
        if ret.is_ok() {
            return ret;
        }
    }
    Err(anyhow!("no result found"))
}

fn find_contiguous_set(data: &Vec<i64>, x: i64) -> Result<i64> {
    for set_size in 2..data.len() - 1 {
        for i in 0..data.len() - set_size {
            if data[i..i + set_size].iter().sum::<i64>() == x {
                // TODO: this can be optimized by using a rolling window, but for now this is fast enough
                let min = data[i..i + set_size].iter().min().unwrap();
                let max = data[i..i + set_size].iter().max().unwrap();
                return Ok(min + max);
            }
        }
    }
    Err(anyhow!("no result found"))
}

pub fn run() -> Result<(i64, i64)> {
    let data = utils::read_i64s("data/input-09.txt")?;
    let answers = utils::read_i64s("data/output-09.txt")?;

    let p1 = find_first_bad(&data, 25, 25)?;
    let p2 = find_contiguous_set(&data, 756008079)?;

    assert_eq!(p1, answers[0]);
    assert_eq!(p2, answers[1]);
    Ok((p1, p2))
}
