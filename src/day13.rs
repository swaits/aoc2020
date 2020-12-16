// ==== day 13

use crate::utils;
use anyhow::Result;

#[derive(Debug)]
struct BusRoute {
    index: usize,
    time: usize,
}

fn parse_bus_routes(input: &str) -> Vec<BusRoute> {
    input
        .trim()
        .split(',')
        .enumerate()
        .filter_map(|(i, s)| {
            if let Ok(t) = s.parse::<usize>() {
                Some(BusRoute { index: i, time: t })
            } else {
                None
            }
        })
        .collect()
}

fn mins_to_next_departure(dep_time: usize, bus: &BusRoute) -> usize {
    let scheduled = bus.time * (dep_time / bus.time);
    if scheduled == dep_time {
        0
    } else {
        scheduled + bus.time - dep_time
    }
}

// after failing at brute force, looked online for hints and learned about
// the Chinese Remainder Theorem.
//
// code from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
#[allow(clippy::many_single_char_names)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// after failing at brute force, looked online for hints and learned about
// the Chinese Remainder Theorem.
//
// code from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

// after failing at brute force, looked online for hints and learned about
// the Chinese Remainder Theorem.
//
// code from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

// main
pub fn run() -> Result<(usize, i64)> {
    let data = utils::read_file("data/input-13.txt")?;
    let answers = utils::read_i64s("data/output-13.txt")?;

    // parse initial start time and bus routes
    let parsed: Vec<&str> = data.splitn(2, '\n').collect();
    assert_eq!(parsed.len(), 2);
    let departure_time = parsed[0].parse::<usize>().unwrap();
    let routes: Vec<BusRoute> = parse_bus_routes(parsed[1]);

    // find which of the bus routes departs most soon after departure_time
    let mut next_departures: Vec<(usize, usize)> = routes
        .iter()
        .map(|bus| (mins_to_next_departure(departure_time, bus), bus.time))
        .collect();
    next_departures.sort_unstable();
    let best = next_departures.first().unwrap();
    let p1 = best.0 * best.1;

    // use CRT to compute the congruence across all these modulii
    //
    // after failing at brute force, looked online for hints and learned about
    // the Chinese Remainder Theorem.
    //
    // code from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
    let modulii: Vec<i64> = routes.iter().map(|bus| bus.time as i64).collect();
    let residues: Vec<i64> = routes
        .iter()
        .map(|bus| {
            if bus.index == 0 {
                0
            } else {
                ((100 * bus.time - bus.index) % bus.time) as i64
            }
        })
        .collect();
    let p2 = chinese_remainder(&residues, &modulii).unwrap();

    assert_eq!(p1, answers[0] as usize);
    assert_eq!(p2, answers[1]);
    Ok((p1, p2))
}
