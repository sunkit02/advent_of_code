pub fn solve_part1(input: &str) -> i32 {
    parse_input(input)
}

pub fn solve_part2(input: &str) -> i32 {
    parse_input2(input)
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

fn parse_input2(input: &str) -> i32 {
    input.split("\n").map(parse_line).flatten().sum()
}

fn parse_line(line: &str) -> Option<i32> {
    let mut num_str = String::new();
    // find front
    for i in 0..line.len() {
        let sub_str = &line[i..];
        if let Some(c) = sub_str.chars().next() {
            if c.is_numeric() {
                num_str.push(c);
                break;
            }
        }

        if sub_str.starts_with("one") {
            num_str.push('1');
            break;
        } else if sub_str.starts_with("two") {
            num_str.push('2');
            break;
        } else if sub_str.starts_with("three") {
            num_str.push('3');
            break;
        } else if sub_str.starts_with("four") {
            num_str.push('4');
            break;
        } else if sub_str.starts_with("five") {
            num_str.push('5');
            break;
        } else if sub_str.starts_with("six") {
            num_str.push('6');
            break;
        } else if sub_str.starts_with("seven") {
            num_str.push('7');
            break;
        } else if sub_str.starts_with("eight") {
            num_str.push('8');
            break;
        } else if sub_str.starts_with("nine") {
            num_str.push('9');
            break;
        }
    }

    // find back
    for i in (0..line.len()).rev() {
        let sub_str = &line[..=i];
        if let Some(c) = sub_str.chars().next_back() {
            if c.is_numeric() {
                num_str.push(c);
                break;
            }
        }

        if sub_str.ends_with("one") {
            num_str.push('1');
            break;
        } else if sub_str.ends_with("two") {
            num_str.push('2');
            break;
        } else if sub_str.ends_with("three") {
            num_str.push('3');
            break;
        } else if sub_str.ends_with("four") {
            num_str.push('4');
            break;
        } else if sub_str.ends_with("five") {
            num_str.push('5');
            break;
        } else if sub_str.ends_with("six") {
            num_str.push('6');
            break;
        } else if sub_str.ends_with("seven") {
            num_str.push('7');
            break;
        } else if sub_str.ends_with("eight") {
            num_str.push('8');
            break;
        } else if sub_str.ends_with("nine") {
            num_str.push('9');
            break;
        }
    }

    return Some(num_str.parse::<i32>().ok()?);
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
