pub fn solve_part1(input: &str) -> i32 {
    parse_input(input)
}

pub fn solve_part2(input: &str) -> i32 {
    let parsed = parse_input(input);
    todo!()
}

fn parse_input(input: &str) -> i32 {
    input
        .split("\n")
        .map(|line| {
            let num_chars = line.chars().filter(|c| c.is_numeric());
            let front = num_chars.clone().take(1).collect::<Vec<char>>().pop()?;
            let back = num_chars.rev().take(1).collect::<Vec<char>>().pop()?;

            let num_str = String::from_utf8(vec![front as u8, back as u8]).ok()?;

            return Some(num_str.parse::<i32>().ok()?);
        })
        .flatten()
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day1::parse_input;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        assert_eq!(parse_input(sample_input), 142);
    }
}
