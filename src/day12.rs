// ==== day 12

use crate::utils;
use anyhow::Result;
use std::mem;

fn navigate(data: &str) -> usize {
    let mut heading = 90;
    let (mut pos_n, mut pos_e) = (0, 0);
    data.lines().for_each(|l| {
        let dir = &l[0..1];
        let dist = &l[1..].parse::<i64>().expect("int parse error");

        match dir {
            "N" => pos_n += dist,
            "S" => pos_n -= dist,
            "E" => pos_e += dist,
            "W" => pos_e -= dist,
            "L" => heading = (heading + 360 - dist) % 360,
            "R" => heading = (heading + dist) % 360,
            "F" => match heading {
                0 => pos_n += dist,
                90 => pos_e += dist,
                180 => pos_n -= dist,
                270 => pos_e -= dist,
                360 => pos_n += dist,
                _ => panic!("non cardinal heading!"),
            },
            _ => panic!("bad direction!"),
        }
    });

    (pos_n.abs() + pos_e.abs()) as usize
}

fn navigate2(data: &str) -> usize {
    let (mut wpt_n, mut wpt_e) = (1, 10);
    let (mut ship_n, mut ship_e) = (0, 0);
    data.lines().for_each(|l| {
        let dir = &l[0..1];
        let dist = &l[1..].parse::<i64>().expect("int parse error");

        match dir {
            "N" => wpt_n += dist,
            "S" => wpt_n -= dist,
            "E" => wpt_e += dist,
            "W" => wpt_e -= dist,
            "L" => match dist {
                270 => {
                    wpt_e = -wpt_e;
                    mem::swap(&mut wpt_n, &mut wpt_e);
                }
                180 => {
                    wpt_n = -wpt_n;
                    wpt_e = -wpt_e;
                }
                90 => {
                    wpt_n = -wpt_n;
                    mem::swap(&mut wpt_n, &mut wpt_e);
                }
                _ => panic!("bad direction!"),
            },
            "R" => match dist {
                90 => {
                    wpt_e = -wpt_e;
                    mem::swap(&mut wpt_n, &mut wpt_e);
                }
                180 => {
                    wpt_n = -wpt_n;
                    wpt_e = -wpt_e;
                }
                270 => {
                    wpt_n = -wpt_n;
                    mem::swap(&mut wpt_n, &mut wpt_e);
                }
                _ => panic!("bad direction!"),
            },
            "F" => {
                ship_n += dist * wpt_n;
                ship_e += dist * wpt_e;
            }
            _ => panic!("bad direction!"),
        }
    });

    (ship_n.abs() + ship_e.abs()) as usize
}

// main
pub fn run() -> Result<(usize, usize)> {
    let data = utils::read_file("data/input-12.txt")?;
    let answers = utils::read_i64s("data/output-12.txt")?;

    let p1 = navigate(&data);
    let p2 = navigate2(&data);

    assert_eq!(p1, answers[0] as usize);
    assert_eq!(p2, answers[1] as usize);
    Ok((p1, p2))
}
