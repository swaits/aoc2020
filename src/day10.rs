// ==== day 10

use crate::utils;
use anyhow::Result;

pub fn count_diff1_diff3(diffs: &Vec<i64>) -> usize {
    let diff_1 = diffs.iter().filter(|n| **n == 1).count();
    let diff_3 = diffs.iter().filter(|n| **n == 3).count();
    diff_1 * diff_3
}

// the number of paths from a contiguous sequence [i+0, i+1, i+N-1] of size N
// follows the pattern 0, 1, 1, 2, 4, where the total paths for a runlength N
// is sum of the paths in N-3, N-2, and N-1.
fn paths_by_runlength(runlength: usize) -> usize {
    let mut p: Vec<usize> = vec![0, 1, 1]; // vec of most recent 4 in sequence
    const L: usize = 3; // length of p Vec
    for i in L..runlength + 1 {
        p[i % L] = p[(i - 3) % L] + p[(i - 2) % L] + p[(i - 1) % L];
    }
    p[runlength % L]
}

fn count_paths(diffs: &Vec<i64>) -> usize {
    // count the runs of 1s
    let mut last_diff: i64 = 0;
    let mut runlength: usize = 1;
    let mut runs_of_ones: Vec<usize> = Vec::new();
    diffs.iter().for_each(|d| {
        if *d == last_diff {
            runlength += 1;
        } else {
            if last_diff == 1 {
                runs_of_ones.push(runlength + 1);
            }
            runlength = 1;
            last_diff = *d;
        }
    });

    // figure out how many path options we have for each run and multiply
    runs_of_ones
        .iter()
        .map(|r| paths_by_runlength(*r))
        .product()
}

pub fn run() -> Result<(usize, usize)> {
    let mut data = utils::read_i64s("data/input-10.txt")?;
    let answers = utils::read_i64s("data/output-10.txt")?;

    // add the adapters on either end of the sequence and sort
    data.push(0); // add the (0) adapter at my seat
    data.sort_unstable(); // paths are always sorted
    data.push(data.last().unwrap() + 3); // add the (max in the list + 3) adapter in my device

    // create a vec of the diffs from data[i] to data[i+1]
    let diffs: Vec<i64> = data
        .iter()
        .enumerate()
        .take(data.len() - 1)
        .map(|(i, _)| (&data[i] - &data[i + 1]).abs())
        .collect();

    let p1 = count_diff1_diff3(&diffs);
    let p2 = count_paths(&diffs);

    assert_eq!(p1, answers[0] as usize);
    assert_eq!(p2, answers[1] as usize);
    Ok((p1, p2))
}
