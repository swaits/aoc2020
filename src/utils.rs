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

pub fn parse_usize_in_range(s: &str, min: usize, max: usize) -> Option<usize> {
    let x: usize = s.parse().expect("int parse error");
    if min <= x && x <= max {
        Some(x)
    } else {
        None
    }
}

// Lexicographic combinations.
//
// This algorithm generates all t-combinations of n numbers {0, 1, ..., n-1},
// given n >= t >= 0. Additional variables c[t] and c[t+1] are used as
// sentinels.
//
// Source: Donald E. Knuth, The Art of Computer Programming, VOLUME 4A,
//         Combinatorial Algorithms, Part 1, Section 7.2.1.3, Algorithm L,
//         Page 358, First printing, January 2011.
pub struct CombinationsState {
    t: usize,
    c: Vec<usize>,
    done: bool,
}

pub fn combinations(t: usize, n: usize) -> CombinationsState {
    let mut c = CombinationsState {
        t,
        c: vec![0; t + 2],
        done: false,
    };

    // L1. Initialize
    for i in 0..t {
        c.c[i] = i;
    }
    c.c[t] = n;
    c.c[t + 1] = 0;
    c
}

impl Iterator for CombinationsState {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        // L2. Visit.
        let ret = if self.done {
            None
        } else {
            Some((&self.c[0..self.c.len() - 2]).to_vec())
        };

        // L3. Find j.
        let mut j = 0;
        while (self.c[j] + 1) == self.c[j + 1] {
            self.c[j] = j;
            j += 1;
        }

        // L4. Done?
        if j >= self.t {
            self.done = true;
        };

        // L5. Increase c[j].
        self.c[j] += 1;

        // return Some(visited) or None from L2 above
        ret
    }
}
