use std::collections::{HashMap, HashSet};

pub fn solve_part1(input: &str) -> i32 {
    let mut map = HashMap::new();
    input
        .split("\n")
        .filter_map(|backpack| {
            let mut duplicates = HashSet::new();
            let (first, second) = backpack.split_at(backpack.len() / 2);
            first.chars().for_each(|c| {
                map.entry(c).or_insert(true);
            });

            second.chars().for_each(|c| match map.get(&c) {
                Some(_) => {
                    duplicates.insert(c);
                }
                None => {}
            });

            map.clear();

            duplicates.into_iter().next()
        })
        .map(item_to_priority)
        .sum()
}

fn item_to_priority(item: char) -> i32 {
    if item.is_uppercase() {
        item as i32 - 64 + 26
    } else if item.is_lowercase() {
        item as i32 - 96
    } else {
        0
    }
}

pub fn solve_part2(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_map_item_to_priority() {
        assert_eq!(item_to_priority('a'), 1);
        assert_eq!(item_to_priority('z'), 26);
        assert_eq!(item_to_priority('A'), 27);
        assert_eq!(item_to_priority('Z'), 52);
    }

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        assert_eq!(solve_part1(sample_input), 157);
    }
}
