use std::collections::HashMap;

pub fn solve_part1(input: &str) -> usize {
    let mut hands = parse_input(input);
    hands.sort();

    hands
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid as usize)
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    todo!()
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand {
    hand_type: HandType,
    cards: Vec<char>,
    bid: u32,
}

impl Hand {
    fn new(cards: Vec<char>, bid: u32) -> Self {
        Self {
            hand_type: calculate_hand_type(&cards),
            cards,
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;

        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => {
                for (my, other) in self.cards.iter().zip(other.cards.iter()) {
                    match cmp_card(*my, *other) {
                        Some(ord) => match ord {
                            Ordering::Less => return Some(Ordering::Less),
                            Ordering::Greater => return Some(Ordering::Greater),
                            Ordering::Equal => continue,
                        },
                        None => return None,
                    }
                }

                None
            }
            ordering => Some(ordering),
        }
    }
}

const VALID_CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

fn cmp_card(a: char, b: char) -> Option<std::cmp::Ordering> {
    let mut a_value = None;
    let mut b_value = None;
    for (value, &card) in VALID_CARDS.iter().rev().enumerate() {
        if card == a {
            let _ = a_value.insert(value);
        }
        if card == b {
            let _ = b_value.insert(value);
        }
    }

    if let (Some(a), Some(b)) = (a_value, b_value) {
        a.partial_cmp(&b)
    } else {
        None
    }
}

fn calculate_hand_type(cards: &[char]) -> HandType {
    let cards = &cards[..5];

    let card_freqs = cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(*card).or_insert(0) += 1;
        acc
    });

    let mut freq_counts = [0; 5];
    for (_card, freq) in card_freqs {
        freq_counts[freq - 1] += 1;
    }

    if freq_counts[4] == 1 {
        HandType::FiveOfAKind
    } else if freq_counts[3] == 1 {
        HandType::FourOfAKind
    } else if freq_counts[2] == 1 && freq_counts[1] == 1 {
        HandType::FullHouse
    } else if freq_counts[2] == 1 {
        HandType::ThreeOfAKind
    } else if freq_counts[1] == 2 {
        HandType::TwoPair
    } else if freq_counts[1] == 1 {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .filter_map(|line| {
            let (hand, bid) = line.split_once(' ')?;
            let hand = hand.chars().collect();
            Some((hand, bid.parse::<u32>().ok()?))
        })
        .map(|(hand, bid)| Hand::new(hand, bid))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

        assert_eq!(solve_part1(sample_input), 6440);
    }
}
