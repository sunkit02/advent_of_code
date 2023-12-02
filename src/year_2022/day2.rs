pub fn solve_part1(input: &str) -> i32 {
    let parsed = parse_input(input);
    play(&parsed)
}

pub fn solve_part2(input: &str) -> i32 {
    let mut parsed = parse_input(input);
    todo!()
}

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn value(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn play(&self, other: &Self) -> PlayResult {
        match (self, other) {
            (Self::Rock, Self::Paper) => PlayResult::Lose(*self),
            (Self::Rock, Self::Scissors) => PlayResult::Win(*self),
            (Self::Paper, Self::Rock) => PlayResult::Win(*self),
            (Self::Paper, Self::Scissors) => PlayResult::Lose(*self),
            (Self::Scissors, Self::Rock) => PlayResult::Lose(*self),
            (Self::Scissors, Self::Paper) => PlayResult::Win(*self),
            _ => PlayResult::Draw(*self),
        }
    }
}

impl From<&str> for Hand {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!(r#"Did not expect string: "{}""#, value),
        }
    }
}

#[derive(Debug)]
enum PlayResult {
    Win(Hand),
    Draw(Hand),
    Lose(Hand),
}

impl PlayResult {
    fn value(&self) -> i32 {
        match self {
            Self::Win(hand_played) => hand_played.value() + 6,
            Self::Draw(hand_played) => hand_played.value() + 3,
            Self::Lose(hand_played) => hand_played.value() + 0,
        }
    }
}

fn parse_input(input: &str) -> Vec<(Hand, Hand)> {
    let mut line_num = 1;
    input
        .split("\n")
        .map(|hands| {
            let (left, right) = hands.split_once(" ")?;
            line_num += 1;
            Some((Hand::from(left), Hand::from(right)))
        })
        .flatten()
        .collect()
}

// Return the greatest calorie count
fn play(input: &[(Hand, Hand)]) -> i32 {
    input
        .iter()
        .map(|(other_hand, my_hand)| my_hand.play(other_hand).value())
        .sum()
}

// fn find_top_3_calories_sum(input: &mut [i32]) -> i32 {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"A Y
B X
C Z"#;

        let result = play(&parse_input(sample_input));
        assert_eq!(result, 15);
    }
}
