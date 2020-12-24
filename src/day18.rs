// ==== day 18

use crate::utils;
use anyhow::{anyhow, Result};

peg::parser! {
    grammar arithmetic_parser() for str {
        rule ws() = quiet!{[' ' | '\n' | '\t']+}

        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule arith_equal_precedence() -> i64 = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "*" y:@ { x * y }
            --
            ws()* n:number() ws()* { n }
            ws()* "(" ws()* e:arith_equal_precedence() ws()* ")" ws()* { e }
        }

        pub rule arith_addition_precedence() -> i64 = precedence!{
            x:(@) "*" y:@ { x * y }
            --
            x:(@) "+" y:@ { x + y }
            --
            ws()* n:number() ws()* { n }
            ws()* "(" ws()* e:arith_addition_precedence() ws()* ")" ws()* { e }
        }
    }
}

// main
pub fn run() -> Result<(i64, i64)> {
    let data = utils::read_file("data/input-18.txt")?;
    let answers = utils::read_i64s("data/output-18.txt")?;

    let p1 = data.lines().fold(0, |sum, line| {
        sum + arithmetic_parser::arith_equal_precedence(line).unwrap()
    });
    let p2 = data.lines().fold(0, |sum, line| {
        sum + arithmetic_parser::arith_addition_precedence(line).unwrap()
    });

    assert_eq!(p1, answers[0]);
    assert_eq!(p2, answers[1]);
    Ok((p1, p2))
}
