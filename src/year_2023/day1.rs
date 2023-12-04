use std::collections::HashMap;

pub fn solve_part1(input: &str) -> i32 {
    parse_input(input)
}

pub fn solve_part2(input: &str) -> i32 {
    parse_input2(input)
}

fn parse_input(input: &str) -> i32 {
    input
        .split('\n')
        .filter_map(|line| {
            let num_chars = line.chars().filter(|c| c.is_numeric());
            let front = num_chars.clone().take(1).collect::<Vec<char>>().pop()?;
            let back = num_chars.rev().take(1).collect::<Vec<char>>().pop()?;

            let num_str = String::from_utf8(vec![front as u8, back as u8]).ok()?;

            num_str.parse::<i32>().ok()
        })
        .sum()
}

fn parse_input2(input: &str) -> i32 {
    input.split('\n').filter_map(parse_line).sum()
}

fn parse_line(line: &str) -> Option<i32> {
    let num_map: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut front = None;
    let mut back = None;

    // find front
    for i in 0..line.len() {
        let sub_str = &line[i..];
        if let Some(c) = sub_str.chars().next() {
            if c.is_numeric() {
                if front.is_none() {
                    let _ = front.insert(c);
                    continue;
                } else {
                    let _ = back.insert(c);
                    continue;
                }
            }
        }

        for (&k, v) in &num_map {
            if sub_str.starts_with(k) {
                if front.is_none() {
                    let _ = front.insert(*v);
                    continue;
                } else {
                    let _ = back.insert(*v);
                    continue;
                }
            }
        }
    }

    let num_str = String::from_utf8(vec![front? as u8, back? as u8]).ok()?;
    num_str.parse().ok()
}

#[cfg(test)]
mod tests {
    use crate::year_2023::day1::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        assert_eq!(parse_input(sample_input), 142);
    }

    #[test]
    fn can_solve_part2() {
        let sample_input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

        assert_eq!(parse_input2(sample_input), 281);
    }
}
