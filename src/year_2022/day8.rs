const ASCII_NUM_OFFSET: i32 = 48;

pub fn solve_part1(input: &str) -> i32 {
    let trees: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as i32 - ASCII_NUM_OFFSET).collect())
        .collect();

    let mut visible_count = 0;

    for (y, row) in trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            // Tree on edge
            if x == 0 || y == 0 || x == row.len() - 1 || y == trees.len() - 1 {
                visible_count += 1;
                continue;
            }

            // Check left
            if let Some(height) = trees[y][..x].iter().max() {
                if height < tree {
                    visible_count += 1;
                    continue;
                }
            }

            // Check right
            if let Some(height) = trees[y][x + 1..].iter().max() {
                if height < tree {
                    visible_count += 1;
                    continue;
                }
            }

            // Check up
            if let Some(height) = trees[..y].iter().map(|row| row[x]).max() {
                if height < *tree {
                    visible_count += 1;
                    continue;
                }
            }

            // Check down
            if let Some(height) = trees[y + 1..].iter().map(|row| row[x]).max() {
                if height < *tree {
                    visible_count += 1;
                    continue;
                }
            }
        }
    }

    visible_count
}

pub fn solve_part2(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"30373
25512
65332
33549
35390"#;

        assert_eq!(solve_part1(sample_input), 21);
    }
}
