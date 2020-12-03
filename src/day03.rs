// ==== day 3

use crate::utils;
use anyhow::Result;

fn count_trees(data: &str, right: usize, down: usize) -> Result<usize> {
    Ok(data
        .lines()
        .step_by(down)
        .enumerate()
        .filter(|(i, s)| {
            s.chars()
                .nth((i * right) % s.len())
                .expect("string index error")
                == '#'
        })
        .count())
}

pub fn run() -> Result<(usize, usize)> {
    let data = utils::read_file("data/input-03.txt")?;
    let answers = utils::read_i64s("data/output-03.txt")?;

    let p1 = count_trees(&data, 3, 1)?;
    assert_eq!(p1, answers[0] as usize);

    let p2 = count_trees(&data, 1, 1)?
        * p1
        * count_trees(&data, 5, 1)?
        * count_trees(&data, 7, 1)?
        * count_trees(&data, 1, 2)?;
    assert_eq!(p2, answers[1] as usize);

    Ok((p1, p2))
}
