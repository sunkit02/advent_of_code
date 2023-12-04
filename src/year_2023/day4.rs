pub fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| Card::try_from(line).ok())
        .map(|card| card.total_points())
        .sum()
}

pub fn solve_part2(input: &str) -> i32 {
    todo!()
}

struct Card {
    winning_nums: Vec<i32>,
    nums: Vec<i32>,
}

impl Card {
    fn total_points(&self) -> i32 {
        let mut points = None;
        for n in &self.nums {
            if self.winning_nums.contains(n) {
                match points {
                    Some(ref mut points) => *points *= 2,
                    None => points = Some(1),
                }
            }
        }

        points.unwrap_or(0)
    }
}

impl From<(Vec<i32>, Vec<i32>)> for Card {
    fn from(value: (Vec<i32>, Vec<i32>)) -> Self {
        Self {
            winning_nums: value.0,
            nums: value.1,
        }
    }
}

impl TryFrom<&str> for Card {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (_, cards_str) = value.split_once(": ").ok_or_else(|| "Wrong card format.")?;
        let (winning_nums, nums) = cards_str
            .split_once(" | ")
            .ok_or_else(|| "No '|' delimeter found.")?;
        let winning_nums = winning_nums
            .split(' ')
            .filter_map(|num_str| num_str.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        let nums = nums
            .split(' ')
            .filter_map(|num_str| num_str.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        Ok(Self { winning_nums, nums })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(solve_part1(sample_input), 13);
    }
}
