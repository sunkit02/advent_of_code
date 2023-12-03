use std::fs;
use std::io::Result;

pub mod year_2022;
pub mod year_2023;

fn main() -> Result<()> {
    let input = fs::read_to_string("data/2023/day3.data")?;

    let highest_sum = input
        .split(|c: char| !c.is_numeric())
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i32>())
        .flatten()
        .sum::<i32>();

    dbg!(highest_sum);

    println!("Output: {}", year_2023::day3::solve_part1(&input));

    Ok(())
}
