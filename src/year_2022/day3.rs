use std::{
    collections::{HashMap, HashSet},
    fmt::format,
    vec,
};

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

pub fn solve_part2(input: &str) -> i32 {
    let mut groups = HashMap::new();
    let elves = input.split("\n");

    let mut id = 0;
    for (i, elf) in elves.enumerate() {
        if i != 0 && i % 3 == 0 {
            id += 1;
        }
        groups.entry(id).or_insert(Vec::new()).push(elf);
    }

    groups
        .into_iter()
        .filter_map(|(_, elves)| {
            let mut masks = HashMap::new();
            for (i, elf) in elves.iter().enumerate() {
                elf.chars().for_each(|c| {
                    let mask = masks.entry(c).or_insert(0);
                    // set bit for index of elf
                    *mask |= 1 << i;
                });
            }

            let badge_priority = masks
                .into_iter()
                .filter_map(|(badge, mask)| {
                    if mask == 0b111 {
                        Some(item_to_priority(badge))
                    } else {
                        None
                    }
                })
                .collect::<Vec<i32>>()
                .into_iter()
                .next()?;

            Some(badge_priority)
        })
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

    #[test]
    fn can_solve_part2() {
        let sample_input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

        assert_eq!(solve_part2(sample_input), 70);
    }
}
