use anyhow::Result;

mod day01;
mod day02;
mod day03;
mod utils;

// TODO
// ====
// * move from i64 to something else that works with the data+problems, int, i32, usize?
// * clean up error handling (move to `?` inside closures)

// ==== main()

fn main() -> Result<()> {
    println!("Day 01: {:?}", day01::run()?);
    println!("Day 02: {:?}", day02::run()?);
    println!("Day 03: {:?}", day03::run()?);
    Ok(())
}
