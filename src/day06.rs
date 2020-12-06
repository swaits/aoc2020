// ==== day 6

use crate::utils;
use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn count_uniq_alpha_chars(s: &str) -> usize {
    let seen: HashSet<usize> = s
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c as usize)
        .collect();
    seen.len()
}

fn count_common_answers(s: &str) -> usize {
    let mut seen: HashMap<char, usize> = HashMap::new();
    for l in s.lines() {
        for c in l.chars().filter(|c| c.is_alphabetic()) {
            *seen.entry(c).or_insert(0) += 1;
        }
    }
    let total = s.lines().count();
    seen.values().filter(|v| *v == &total).count()
}

pub fn run() -> Result<(usize, usize)> {
    let data = utils::read_file("data/input-06.txt")?;
    let answers = utils::read_i64s("data/output-06.txt")?;

    // parse into an array of Passport{}
    let p1 = data.split("\n\n").map(|s| count_uniq_alpha_chars(s)).sum();

    // count passports with all req'd and validated fields
    let p2 = data.split("\n\n").map(|s| count_common_answers(s)).sum();

    assert_eq!(p1, answers[0] as usize);
    assert_eq!(p2, answers[1] as usize);

    Ok((p1, p2))
}
