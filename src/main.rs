use std::fs;
use std::io::Result;

pub mod year_2022;

fn main() -> Result<()> {
    let input = fs::read_to_string("data/2022/day2.data")?;

    dbg!(year_2022::day2::solve_part1(&input));

    Ok(())
}
