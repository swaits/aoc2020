// ==== day 2

use crate::utils;
use anyhow::Result;
use regex::Regex;

struct PasswordSpec {
    min: usize,
    max: usize,
    required: char,
    password: String,
}

fn valid_password_1(p: &PasswordSpec) -> bool {
    let count = p.password.chars().filter(|c| c == &p.required).count();
    p.min <= count && count <= p.max
}

fn valid_password_2(p: &PasswordSpec) -> bool {
    let char_a = p.password.chars().nth(p.min - 1).unwrap();
    let char_b = p.password.chars().nth(p.max - 1).unwrap();
    (char_a == p.required) ^ (char_b == p.required)
}

pub fn run() -> Result<(usize, usize)> {
    let data = utils::read_file("data/input-02.txt")?;
    let answers = utils::read_i64s("data/output-02.txt")?;

    // parse lines like "4-8 n: dnjjrtclnzdnghnbnn" into vector of PasswordSpec{}
    let re = Regex::new(r"(\d+)-(\d+) +([a-z]): +(\S*)")?;
    let pwspecs: Vec<PasswordSpec> = re
        .captures_iter(&data)
        .map(|cap| PasswordSpec {
            min: cap[1].parse().expect("parse error"),
            max: cap[2].parse().expect("parse error"),
            required: cap[3].chars().next().expect("string index error"),
            password: (&cap[4]).to_string(),
        })
        .collect();

    let p1 = pwspecs.iter().filter(|p| valid_password_1(&p)).count();
    assert_eq!(p1, answers[0] as usize);

    let p2 = pwspecs.iter().filter(|p| valid_password_2(&p)).count();
    assert_eq!(p2, answers[1] as usize);

    Ok((p1, p2))
}
