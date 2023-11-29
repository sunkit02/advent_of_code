pub fn solve(input: &str) -> usize {
    find_max_calories(parse_input(input))
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|load| {
            load.split("\n")
                .map(|e| e.parse::<usize>().ok())
                .flatten()
                .sum()
        })
        .collect()
}

// Return the greatest calorie count
fn find_max_calories(mut input: Vec<usize>) -> usize {
    input.sort_by(|a, b| b.cmp(a));
    return *input.get(0).expect("Should have at least one elf");
}


#[cfg(test)]
mod tests {
    use super::{parse_input, find_max_calories};

    #[test]
    fn can_count_calories() {
        let sample_input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

        let result = find_max_calories(parse_input(sample_input));

        assert_eq!(result, 24000);
    }
}
