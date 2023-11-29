use std::fs;
use std::io::Result;

pub mod year_2022;

fn main() -> Result<()> {
    let input = fs::read_to_string("data/2022/day1.data")?;

    dbg!(year_2022::day1::solve_part2(&input));

    Ok(())
}
