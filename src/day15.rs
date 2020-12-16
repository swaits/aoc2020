// ==== day 15

use crate::utils;
use anyhow::Result;
use std::collections::HashMap;

fn play_speaking_game(input: &[i64], n: usize) -> i64 {
    let mut seen: HashMap<i64, usize> = HashMap::new();
    let mut last_insert: Option<usize> = None;

    input.iter().enumerate().for_each(|(turn, num)| {
        last_insert = seen.insert(*num, turn);
    });

    (seen.len()..n).fold(*input.last().unwrap(), |_, turn| {
        let num = match last_insert {
            Some(prev_turn) => (turn - 1 - prev_turn),
            None => 0,
        } as i64;

        last_insert = seen.insert(num, turn);
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
    let p2 = play_speaking_game(&data, 30_000_000);

    assert_eq!(p1, answers[0]);
    assert_eq!(p2, answers[1]);
    Ok((p1, p2))
}
