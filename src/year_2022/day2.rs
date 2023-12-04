pub fn solve_part1(input: &str) -> i32 {
    let parsed = parse_input(input);
    play(&parsed)
}

pub fn solve_part2(input: &str) -> i32 {
    parse_input2(input)
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
            Self::Lose(hand_played) => hand_played.value(),
        }
    }

    fn from_target_result(other_str: &str, result_str: &str) -> Self {
        let other_hand = Hand::from(other_str);
        let mut result = match result_str {
            // using other_hand as placeholder for final value
            "X" => Self::Lose(other_hand),
            "Y" => Self::Draw(other_hand),
            "Z" => Self::Win(other_hand),
            _ => panic!(r#"Did not expect string: "{}""#, result_str),
        };

        match result {
            PlayResult::Win(ref mut hand) => match other_hand {
                Hand::Rock => *hand = Hand::Paper,
                Hand::Paper => *hand = Hand::Scissors,
                Hand::Scissors => *hand = Hand::Rock,
            },
            PlayResult::Draw(ref mut hand) => match other_hand {
                Hand::Rock => *hand = Hand::Rock,
                Hand::Paper => *hand = Hand::Paper,
                Hand::Scissors => *hand = Hand::Scissors,
            },
            PlayResult::Lose(ref mut hand) => match other_hand {
                Hand::Rock => *hand = Hand::Scissors,
                Hand::Paper => *hand = Hand::Rock,
                Hand::Scissors => *hand = Hand::Paper,
            },
        };

        result
    }
}

fn parse_input(input: &str) -> Vec<(Hand, Hand)> {
    let mut line_num = 1;
    input
        .split('\n')
        .filter_map(|hands| {
            let (left, right) = hands.split_once(' ')?;
            line_num += 1;
            Some((Hand::from(left), Hand::from(right)))
        })
        .collect()
}

// Return the greatest calorie count
fn play(input: &[(Hand, Hand)]) -> i32 {
    input
        .iter()
        .map(|(other_hand, my_hand)| my_hand.play(other_hand).value())
        .sum()
}

fn parse_input2(input: &str) -> i32 {
    input
        .split('\n')
        .filter_map(|hands| {
            let (left, right) = hands.split_once(' ')?;
            Some(PlayResult::from_target_result(left, right))
        })
        .map(|result| result.value())
        .sum()
}

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

    #[test]
    fn can_solve_part2() {
        let sample_input = r#"A Y
B X
C Z"#;

        let result = parse_input2(sample_input);
        assert_eq!(result, 12);
    }
}
