// ==== utilities

use anyhow::Result;
use std::fs;

pub fn read_i64s(filename: &str) -> Result<Vec<i64>> {
    Ok(read_file(filename)?
        .lines()
        .map(|s| s.parse::<i64>().expect("i64 parse error"))
        .collect())
}

pub fn read_file(filename: &str) -> Result<String> {
    Ok(fs::read_to_string(filename)?)
}
