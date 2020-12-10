// ==== day 7

use crate::utils;
use anyhow::Result;
use std::collections::HashMap;

// light red bags contain 1 bright white bag, 2 muted yellow bags.
// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
// bright white bags contain 1 shiny gold bag.
// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
// faded blue bags contain no other bags.
// dotted black bags contain no other bags.

struct PurseQuantity<'a>(usize, &'a str);

// parse data into a HashMap<&str, Vec<&str>>
//
// for example: "light red bags contain 1 bright white bag, 2 muted yellow bags."
// becomes: { "light red": ["bright white", "muted yellow", "muted yellow"]}
//
fn parse(s: &str) -> HashMap<&str, Vec<PurseQuantity>> {
    s.lines()
        .map(|l| {
            let parsed: Vec<&str> = l.splitn(2, " bags contain ").collect();
            let container = parsed[0];
            let contents: Vec<PurseQuantity> = match parsed[1].trim() {
                "no other bags." => Vec::new(),
                _ => parsed[1]
                    .trim_end_matches('.')
                    .split(',')
                    .map(|c| {
                        let trimmed = c.trim_end_matches("bags").trim_end_matches("bag").trim();
                        let count: usize = trimmed[0..1].parse().unwrap();
                        let purse = &trimmed[2..];
                        PurseQuantity(count, purse)
                    })
                    .collect(),
            };
            (container, contents)
        })
        .collect()
}

fn has_contents(data: &HashMap<&str, Vec<PurseQuantity>>, container: &str, contents: &str) -> bool {
    let these_contents: &Vec<&str> = &data.get(&container).unwrap().iter().map(|c| c.1).collect();
    these_contents.contains(&contents)
        || these_contents
            .iter()
            .any(|m| has_contents(data, m, contents))
}

fn count_contents(data: &HashMap<&str, Vec<PurseQuantity>>, container: &str) -> usize {
    let these_contents: &Vec<PurseQuantity> = &data.get(&container).unwrap();
    these_contents
        .iter()
        .map(|c| c.0 + c.0 * count_contents(data, c.1))
        .sum()
}

pub fn run() -> Result<(usize, usize)> {
    let data = utils::read_file("data/input-07.txt")?;
    let answers = utils::read_i64s("data/output-07.txt")?;

    let parsed = parse(&data);

    let p1 = parsed
        .keys()
        .filter(|k| has_contents(&parsed, k, "shiny gold"))
        .count();

    let p2 = count_contents(&parsed, "shiny gold");

    assert_eq!(p1, answers[0] as usize);
    assert_eq!(p2, answers[1] as usize);
    Ok((p1, p2))
}
