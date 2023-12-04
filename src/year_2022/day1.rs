pub fn solve_part1(input: &str) -> usize {
    let mut parsed = parse_input(input);
    find_max_calories(&mut parsed)
}

pub fn solve_part2(input: &str) -> usize {
    let mut parsed = parse_input(input);
    find_top_3_calories_sum(&mut parsed)
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split("\n\n")
        .map(|load| {
            load.split('\n')
                .filter_map(|e| e.parse::<usize>().ok())
                .sum()
        })
        .collect()
}

// Return the greatest calorie count
fn find_max_calories(input: &mut [usize]) -> usize {
    input.sort_by(|a, b| b.cmp(a));
    return *input.first().expect("Should have at least one elf");
}

fn find_top_3_calories_sum(input: &mut [usize]) -> usize {
    input.sort_by(|a, b| b.cmp(a));
    input[..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let result = solve_part1(sample_input);

        assert_eq!(result, 24000);
    }

    #[test]
    fn can_find_top_3_calories() {
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

        let result = solve_part2(sample_input);

        assert_eq!(result, 45000);
    }
}
