// ==== day 15

use crate::utils;
use anyhow::Result;
use std::collections::BTreeMap;

fn play_speaking_game(input: &[i64], n: usize) -> i64 {
    let mut seen: BTreeMap<i64, usize> = BTreeMap::new();
    let mut last_insert: Option<usize> = None;

    input.iter().enumerate().for_each(|(i, n)| {
        last_insert = seen.insert(*n, i);
    });

    (seen.len()..n).fold(*input.last().unwrap(), |_, step| {
        let num: i64 = match last_insert {
            Some(prev_step) => (step - 1 - prev_step) as i64,
            None => 0_i64,
        };

        last_insert = seen.insert(num, step);
        num
    })
}

// main
pub fn run() -> Result<(i64, i64)> {
    let data: Vec<i64> = utils::read_file("data/input-15.txt")?
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().expect("parse error"))
        .collect();
    let answers = utils::read_i64s("data/output-15.txt")?;

    let p1 = play_speaking_game(&data, 2020);
    let p2 = play_speaking_game(&data, 30000000);

    assert_eq!(p1, answers[0]);
    assert_eq!(p2, answers[1]);
    Ok((p1, p2))
}
