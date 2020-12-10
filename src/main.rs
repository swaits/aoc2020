use anyhow::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod utils;

// TODO
// ====
// * clean up error handling (move to `?` inside closures)
// * templatize read_i64s()
// * build out a bit of a harness for the runner
// * add criterion benchmarking
// * figure out nicer way to organize into modules

// ==== main()

fn main() -> Result<()> {
    println!("Day 01: {:?}", day01::run()?);
    println!("Day 02: {:?}", day02::run()?);
    println!("Day 03: {:?}", day03::run()?);
    println!("Day 04: {:?}", day04::run()?);
    println!("Day 05: {:?}", day05::run()?);
    println!("Day 06: {:?}", day06::run()?);
    println!("Day 07: {:?}", day07::run()?);
    println!("Day 08: {:?}", day08::run()?);
    Ok(())
}
