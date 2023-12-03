use std::collections::{HashMap, HashSet};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(usize, usize);

pub fn solve_part2(input: &str) -> i32 {
    let schema: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // gear pt -> set of the pts of nums adjacent to it
    let mut gears: HashMap<Point, HashSet<Point>> = HashMap::new();
    let mut num_strs: HashMap<Point, String> = HashMap::new();

    let mut num_pt: Option<Point> = None;

    for (y, line) in schema.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c.is_numeric() {
                num_strs
                    .entry(num_pt.get_or_insert(Point(x, y)).clone())
                    .or_insert(String::new())
                    .push(*c);

                if x > 0 {
                    // check left
                    if is_gear(schema[y][x - 1]) {
                        if let Some(pt) = num_pt {
                            gears
                                .entry(Point(x - 1, y))
                                .or_insert(HashSet::new())
                                .insert(pt);
                        }
                    }

                    if y > 0 {
                        // check up left
                        if is_gear(schema[y - 1][x - 1]) {
                            if let Some(pt) = num_pt {
                                gears
                                    .entry(Point(x - 1, y - 1))
                                    .or_insert(HashSet::new())
                                    .insert(pt);
                            }
                        }
                    }

                    if y < schema.len() - 1 {
                        // check down left
                        if is_gear(schema[y + 1][x - 1]) {
                            if let Some(pt) = num_pt {
                                gears
                                    .entry(Point(x - 1, y + 1))
                                    .or_insert(HashSet::new())
                                    .insert(pt);
                            }
                        }
                    }
                }

                if y > 0 {
                    // check up
                    if is_gear(schema[y - 1][x]) {
                        if let Some(pt) = num_pt {
                            gears
                                .entry(Point(x, y - 1))
                                .or_insert(HashSet::new())
                                .insert(pt);
                        }
                    }
                }

                if y < schema.len() - 1 {
                    // check down
                    if is_gear(schema[y + 1][x]) {
                        if let Some(pt) = num_pt {
                            gears
                                .entry(Point(x, y + 1))
                                .or_insert(HashSet::new())
                                .insert(pt);
                        }
                    }
                }

                if x < line.len() - 1 {
                    // check right
                    if is_gear(schema[y][x + 1]) {
                        if let Some(pt) = num_pt {
                            gears
                                .entry(Point(x + 1, y))
                                .or_insert(HashSet::new())
                                .insert(pt);
                        }
                    }

                    if y > 0 {
                        // check up right
                        if is_gear(schema[y - 1][x + 1]) {
                            if let Some(pt) = num_pt {
                                gears
                                    .entry(Point(x + 1, y - 1))
                                    .or_insert(HashSet::new())
                                    .insert(pt);
                            }
                        }
                    }

                    if y < schema.len() - 1 {
                        // check down right
                        if is_gear(schema[y + 1][x + 1]) {
                            if let Some(pt) = num_pt {
                                gears
                                    .entry(Point(x + 1, y + 1))
                                    .or_insert(HashSet::new())
                                    .insert(pt);
                            }
                        }
                    }
                }
            } else {
                num_pt.take();
            }
        }
    }

    let mut sum = 0;

    for (_, part_nums) in gears {
        if part_nums.len() == 2 {
            sum += part_nums
                .into_iter()
                .filter_map(|pt| num_strs.get(&pt)?.parse::<i32>().ok())
                .product::<i32>();
        }
    }

    return sum;
}

fn is_gear(c: char) -> bool {
    return c == '*';
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
    fn can_solve_part2() {
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

        assert_eq!(solve_part2(sample_input), 467835);
    }
}
