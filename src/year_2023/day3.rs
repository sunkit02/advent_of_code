pub fn solve_part1(input: &str) -> i32 {
    let schema: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;
    let mut num_str: String = String::new();
    let mut is_part = false;

    for (y, line) in schema.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if !c.is_numeric() || x == line.len() - 1 {
                // Push last numeric char
                if c.is_numeric() {
                    num_str.push(*c);
                }

                if is_part {
                    sum += num_str.parse::<i32>().unwrap();
                    is_part = false;
                }
                num_str.clear();
                continue;
            }

            num_str.push(*c);

            assert!(c.is_numeric());

            if x > 0 {
                // check left
                if is_symbol(schema[y][x - 1]) {
                    is_part = true;
                    continue;
                }

                if y > 0 {
                    // check up left
                    if is_symbol(schema[y - 1][x - 1]) {
                        is_part = true;
                        continue;
                    }
                }

                if y < schema.len() - 1 {
                    // check down left
                    if is_symbol(schema[y + 1][x - 1]) {
                        is_part = true;
                        continue;
                    }
                }
            }

            if y > 0 {
                // check up
                if is_symbol(schema[y - 1][x]) {
                    is_part = true;
                    continue;
                }
            }

            if y < schema.len() - 1 {
                // check down
                if is_symbol(schema[y + 1][x]) {
                    is_part = true;
                    continue;
                }
            }

            if x < line.len() - 1 {
                // check right
                if is_symbol(schema[y][x + 1]) {
                    is_part = true;
                    continue;
                }

                if y > 0 {
                    // check up right
                    if is_symbol(schema[y - 1][x + 1]) {
                        is_part = true;
                        continue;
                    }
                }

                if y < schema.len() - 1 {
                    // check down right
                    if is_symbol(schema[y + 1][x + 1]) {
                        is_part = true;
                        continue;
                    }
                }
            }
        }
    }

    return sum;
}

fn is_symbol(c: char) -> bool {
    match c {
        '`' => true,
        '~' => true,
        '!' => true,
        '@' => true,
        '#' => true,
        '$' => true,
        '%' => true,
        '^' => true,
        '&' => true,
        '*' => true,
        '(' => true,
        ')' => true,
        '-' => true,
        '_' => true,
        '+' => true,
        '=' => true,
        '[' => true,
        ']' => true,
        '{' => true,
        '}' => true,
        '\\' => true,
        '|' => true,
        ';' => true,
        ':' => true,
        '\'' => true,
        '"' => true,
        ',' => true,
        '<' => true,
        '>' => true,
        '?' => true,
        '/' => true,
        _ => false,
    }
}
pub fn solve_part2(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(solve_part1(sample_input), 4361);
    }

    #[test]
    fn can_solve_part1_changed() {
        let sample_input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

        assert_eq!(solve_part1(sample_input), 4361);
    }
}
