use std::fs;
use std::io::Result;

pub mod year_2022;
pub mod year_2023;

fn main() -> Result<()> {
    let input = fs::read_to_string("data/2023/day1.data")?;

    dbg!(year_2023::day1::solve_part1(&input));

    Ok(())
}
