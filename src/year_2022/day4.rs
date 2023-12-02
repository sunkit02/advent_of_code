use std::ops::RangeBounds;

pub fn solve_part1(input: &str) -> i32 {
    input
        .split("\n")
        .filter_map(|pair| pair.split_once(","))
        .filter_map(|pair: (&str, &str)| -> Option<((i32, i32), (i32, i32))> {
            let (first, second) = pair;

            let (f_low, f_high) = first.split_once("-")?;
            let (s_low, s_high) = second.split_once("-")?;

            Some((
                (f_low.parse().ok()?, f_high.parse().ok()?),
                (s_low.parse().ok()?, s_high.parse().ok()?),
            ))
        })
        .filter(|pair| {
            let ((a_low, a_high), (b_low, b_high)) = pair;
            if a_low <= b_low && a_high >= b_high || b_low <= a_low && b_high >= a_high {
                true
            } else {
                false
            }
        })
        .count() as i32
}

pub fn solve_part2(input: &str) -> i32 {
    input
        .split("\n")
        .filter_map(|pair| pair.split_once(","))
        .filter_map(|pair: (&str, &str)| -> Option<((i32, i32), (i32, i32))> {
            let (first, second) = pair;

            let (f_low, f_high) = first.split_once("-")?;
            let (s_low, s_high) = second.split_once("-")?;

            Some((
                (f_low.parse().ok()?, f_high.parse().ok()?),
                (s_low.parse().ok()?, s_high.parse().ok()?),
            ))
        })
        .filter(|pair| {
            let ((a_low, a_high), (b_low, b_high)) = pair;
            let a_range = *a_low..=*a_high;
            let b_range = *b_low..=*b_high;

            if a_range.contains(b_low)
                || a_range.contains(b_high)
                || b_range.contains(a_low)
                || b_range.contains(a_high)
            {
                true
            } else {
                false
            }
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        assert_eq!(solve_part1(sample_input), 2);
    }

    #[test]
    fn can_solve_part2() {
        let sample_input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

        assert_eq!(solve_part2(sample_input), 4);
    }
}
