pub fn solve_part1(input: &str) -> i32 {
    parse_input(input)
}

pub fn solve_part2(input: &str) -> i32 {
    parse_input2(input)
}

#[derive(Debug)]
struct MaxGameRecord {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl TryFrom<&str> for MaxGameRecord {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (id_str, values_str) = value
            .split_once(": ")
            .ok_or("Failed to split initially at ': '")?;

        let (_, id) = id_str
            .split_once(' ')
            .ok_or("Failed to split `id_str`")?;

        let id = id
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Failed to parse: {}", id));

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for set in values_str.split("; ") {
            for pair in set.split(", ") {
                let (num_str, color) = pair
                    .split_once(' ')
                    .ok_or("Failed to split at ' '")?;
                let num = num_str.parse::<i32>().map_err(|e| e.to_string())?;

                match color {
                    "red" => red = if num > red { num } else { red },
                    "green" => green = if num > green { num } else { green },
                    "blue" => blue = if num > blue { num } else { blue },
                    _ => {}
                }
            }
        }

        Ok(Self {
            id,
            red,
            green,
            blue,
        })
    }
}

fn parse_input(input: &str) -> i32 {
    input
        .split('\n')
        .filter_map(|line| {
            let MaxGameRecord {
                id,
                red,
                green,
                blue,
            } = MaxGameRecord::try_from(line).ok()?;

            if red <= 12 && green <= 13 && blue <= 14 {
                return Some(id);
            }
            None
        })
        .sum()
}

fn parse_input2(input: &str) -> i32 {
    input
        .split('\n')
        .filter_map(|line| {
            let MaxGameRecord {
                id: _,
                red,
                green,
                blue,
            } = MaxGameRecord::try_from(line).ok()?;

            Some(red * green * blue)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

        assert_eq!(parse_input(sample_input), 8);
    }

    #[test]
    fn can_solve_part2() {
        let sample_input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
        assert_eq!(parse_input2(sample_input), 2286);
    }
}
