use std::fs;
use std::io::Result;

pub mod year_2022;
pub mod year_2023;

fn main() -> Result<()> {
    let input = fs::read_to_string("data/2022/day7.data")?;

    println!("Output: {:?}", year_2022::day7::solve_part2(&input));

    Ok(())
}
