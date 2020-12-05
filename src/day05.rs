// ==== day 5

use crate::utils;
use anyhow::Result;

fn seat_code_to_usize(code: &str) -> usize {
    assert_eq!(code.len(), 10);
    let bincode = str::replace(code, "B", "1");
    let bincode = str::replace(&bincode, "F", "0");
    let bincode = str::replace(&bincode, "R", "1");
    let bincode = str::replace(&bincode, "L", "0");
    usize::from_str_radix(&bincode, 2).unwrap()
}

fn usize_to_seat_id(x: usize) -> usize {
    ((x >> 3) * 8) + (x & 0b111)
}

pub fn run() -> Result<(usize, usize)> {
    let data = utils::read_file("data/input-05.txt")?;
    let answers = utils::read_i64s("data/output-05.txt")?;

    // compute all seats (as integers)
    let mut seat_ints: Vec<usize> = data.lines().map(|code| seat_code_to_usize(code)).collect();
    seat_ints.sort_unstable();

    // find the seat with the highest seat code
    let p1 = usize_to_seat_id(*seat_ints.last().unwrap());

    // find the empty seat
    let mut empty = 0;
    for i in 0..seat_ints.len() - 1 {
        if seat_ints[i] + 2 == seat_ints[i + 1] {
            empty = seat_ints[i] + 1;
            break;
        }
    }
    let p2 = usize_to_seat_id(empty);

    assert_eq!(p1, answers[0] as usize);
    assert_eq!(p2, answers[1] as usize);

    Ok((p1, p2))
}
