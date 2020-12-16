// ==== day 11

use crate::utils;
use anyhow::Result;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Location {
    Floor,
    Unoccupied,
    Occupied,
}

// compute the 8 immediately adjacent coordinates which contian chairs
fn adjacent_coords(
    seat_map: &[Vec<Location>],
    pos: (usize, usize),
    max: (usize, usize),
) -> Vec<(usize, usize)> {
    let min_row = match pos.0 {
        0 => 0,
        _ => pos.0 - 1,
    };
    let min_col = match pos.1 {
        0 => 0,
        _ => pos.1 - 1,
    };
    let max_row = max.0.min(pos.0 + 1);
    let max_col = max.1.min(pos.1 + 1);

    (min_row..=max_row)
        .flat_map(|i| (min_col..=max_col).map(move |j| (i, j)))
        .filter(|p| *p != pos && seat_map[p.0][p.1] != Location::Floor)
        .collect()
}

// precompute all the immediately adjacent chairs for all positions on a map
fn compute_basic_adjacency(map: &[Vec<Location>]) -> Vec<Vec<Vec<(usize, usize)>>> {
    (0..map.len())
        .map(|row| {
            (0..map[row].len())
                .map(|col| adjacent_coords(&map, (row, col), (map.len() - 1, map[row].len() - 1)))
                .collect()
        })
        .collect()
}

// search in one direction from a position until finding a chair (used to compute
// advanced adjacency
fn search_direction_for_chair(
    map: &[Vec<Location>],
    start: (usize, usize),
    step: (i32, i32),
) -> Option<(usize, usize)> {
    let mut i = start.0 as i32 + step.0;
    let mut j = start.1 as i32 + step.1;

    while (0 <= i && i < map.len() as i32) && (0 <= j && j < map[i as usize].len() as i32) {
        if map[i as usize][j as usize] != Location::Floor {
            return Some((i as usize, j as usize));
        }
        i += step.0;
        j += step.1;
    }
    None
}

// precompute all the directionally adjacent chairs for all positions on a map
fn compute_advanced_adjacency(map: &[Vec<Location>]) -> Vec<Vec<Vec<(usize, usize)>>> {
    // figure out our eight direction step values
    let dirs: Vec<(i32, i32)> = (-1..2)
        .flat_map(|i| (-1..2).map(move |j| (i, j)))
        .filter(|p| *p != (0, 0))
        .collect();

    // for every cell, search for a chair in the eight directions
    (0..map.len())
        .map(|i| {
            (0..map[i].len())
                .map(|j| {
                    dirs.iter()
                        .filter_map(move |d| {
                            let dir = *d;
                            search_direction_for_chair(map, (i, j), dir)
                        })
                        .collect()
                })
                .collect()
        })
        .collect::<Vec<Vec<Vec<(usize, usize)>>>>()
}

// run the seating algorithm, return the total number of changes
fn seating_round(
    map: &mut Vec<Vec<Location>>,
    adj: &[Vec<Vec<(usize, usize)>>],
    occupied_threshold: usize,
) -> usize {
    let mut changes: Vec<(usize, usize, Location)> = Vec::new();

    for row in 0..map.len() {
        for col in 0..map[row].len() {
            match map[row].get(col).unwrap() {
                Location::Floor => (),
                Location::Unoccupied => {
                    if !adj[row][col]
                        .iter()
                        .any(|l| map[l.0][l.1] == Location::Occupied)
                    {
                        changes.push((row, col, Location::Occupied));
                    }
                }
                Location::Occupied => {
                    if adj[row][col]
                        .iter()
                        .filter(|l| map[l.0][l.1] == Location::Occupied)
                        .count()
                        >= occupied_threshold
                    {
                        changes.push((row, col, Location::Unoccupied));
                    }
                }
            };
        }
    }

    changes.iter().for_each(|c| map[c.0][c.1] = c.2);
    changes.len()
}

// coutnt occupised seats on a given map
fn count_occupied(map: &[Vec<Location>]) -> usize {
    map.iter()
        .flat_map(|row| row.iter().filter(|cell| **cell == Location::Occupied))
        .count()
}

// main
pub fn run() -> Result<(usize, usize)> {
    let data = utils::read_file("data/input-11.txt")?;
    let answers = utils::read_i64s("data/output-11.txt")?;

    // parse into a 2D vector of Location{}
    let starting_map: Vec<Vec<Location>> = data
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'L' => Location::Unoccupied,
                    '.' => Location::Floor,
                    _ => unreachable!("bad input"),
                })
                .collect()
        })
        .collect();

    // run seating alogirthm using basic adjacency and occupied threshold of 4
    let mut map1 = starting_map.clone();
    let basic_adjacency = compute_basic_adjacency(&map1);
    while seating_round(&mut map1, &basic_adjacency, 4) > 0 {}
    let p1 = count_occupied(&map1);
    drop(map1);

    // run seating alogirthm using advanced adjacency and occupied threshold of 5
    let mut map2 = starting_map;
    let advanced_adjacency = compute_advanced_adjacency(&map2);
    while seating_round(&mut map2, &advanced_adjacency, 5) > 0 {}
    let p2 = count_occupied(&map2);
    drop(map2);

    assert_eq!(p1, answers[0] as usize);
    assert_eq!(p2, answers[1] as usize);
    Ok((p1, p2))
}
