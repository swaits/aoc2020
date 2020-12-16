// ==== day 16

use crate::utils;
use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Field {
    name: String,
    ranges: Vec<std::ops::Range<i64>>,
}

impl Field {
    fn num_in_range(&self, num: i64) -> bool {
        self.ranges.iter().any(|r| r.contains(&num))
    }
}

fn parse_fields(input: &str) -> Result<Vec<Field>> {
    let re = Regex::new(
        r"(?P<field>[\w\s]+): (?P<min1>\d+)\-(?P<max1>\d+) or (?P<min2>\d+)\-(?P<max2>\d+)",
    )?;

    Ok(re
        .captures_iter(input)
        .map(|caps| Field {
            name: caps["field"].trim().to_owned(),
            ranges: vec![
                std::ops::Range {
                    start: caps["min1"].trim().parse::<i64>().unwrap(),
                    end: caps["max1"].trim().parse::<i64>().unwrap() + 1_i64,
                },
                std::ops::Range {
                    start: caps["min2"].trim().parse::<i64>().unwrap(),
                    end: caps["max2"].trim().parse::<i64>().unwrap() + 1_i64,
                },
            ],
        })
        .collect())
}

fn parse_tickets(input: &str) -> Result<(Vec<i64>, Vec<Vec<i64>>)> {
    let mut lines = input.lines().map(str::trim).filter(|l| l.contains(','));

    let my_ticket: Vec<i64> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let other_tickets: Vec<Vec<i64>> = lines
        .map(|l| l.split(',').map(|n| n.parse::<i64>().unwrap()).collect())
        .collect();

    Ok((my_ticket, other_tickets))
}

fn ticket_error_rate(fields: &[Field], ticket: &[i64]) -> i64 {
    ticket.iter().fold(0, |sum, value| {
        if fields
            .iter()
            .any(|f| f.ranges.iter().any(|r| r.contains(value)))
        {
            sum
        } else {
            sum + value
        }
    })
}

fn find_field_locations(fields: &[Field], my_ticket: &[i64], tickets: &[Vec<i64>]) -> i64 {
    // find valid tickets
    let mut valid_tickets: Vec<&Vec<i64>> = tickets
        .iter()
        .filter(|t| ticket_error_rate(&fields, t) == 0)
        .collect();
    let my_ticket = my_ticket.to_vec();
    valid_tickets.push(&my_ticket);

    // build matrix of maps which we'll use to track viable fields for
    // each column of each ticket
    let mut viable_fields: Vec<HashMap<&str, usize>> = vec![HashMap::new(); my_ticket.len()];
    valid_tickets.iter().for_each(|ticket| {
        ticket.iter().enumerate().for_each(|(j, col)| {
            fields.iter().for_each(|f| {
                if f.num_in_range(*col) {
                    *viable_fields[j].entry(&f.name).or_insert(0) += 1;
                }
            });
        });
    });

    // now revisit that matrix, and looking at one column at a time, find
    // the column that has only one field which 100% matched, then remove it
    // from the list
    let mut matches: HashMap<&str, usize> = HashMap::new();
    for _ in 0..my_ticket.len() {
        // find a matching field
        for (col_id, fields_for_column) in viable_fields.iter().enumerate() {
            let possible_fields = fields_for_column
                .iter()
                .filter(|(k, _)| !matches.contains_key(*k))
                .filter(|(_, v)| **v == valid_tickets.len())
                .collect::<HashMap<&&str, &usize>>();
            if possible_fields.len() == 1 {
                let match_name = possible_fields.keys().next().unwrap();
                let match_col = col_id;
                matches.insert(match_name, match_col);
            }
        }
    }

    // multiply our ticket's "departure" fields
    matches.iter().fold(1, |product, (k, v)| {
        if k.starts_with("departure") {
            product * my_ticket[*v]
        } else {
            product
        }
    })
}

// main
pub fn run() -> Result<(i64, i64)> {
    let data = utils::read_file("data/input-16.txt")?;
    let answers = utils::read_i64s("data/output-16.txt")?;

    let fields = parse_fields(&data)?;
    let (my_ticket, other_tickets) = parse_tickets(&data)?;

    // find invalid tickets, sum error rates
    let p1 = other_tickets
        .iter()
        .map(|t| ticket_error_rate(&fields, t))
        .sum::<i64>();

    let p2 = find_field_locations(&fields, &my_ticket, &other_tickets);

    assert_eq!(p1, answers[0]);
    assert_eq!(p2, answers[1]);
    Ok((p1, p2))
}
