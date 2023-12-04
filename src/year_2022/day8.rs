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

pub fn solve_part2(input: &str) -> i32 {
    let trees: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as i32 - ASCII_NUM_OFFSET).collect())
        .collect();

    let mut max_scenic_score = 0;

    for (y, row) in trees.iter().enumerate() {
        for (x, tree) in row.iter().enumerate() {
            let mut left = 0;
            let mut right = 0;
            let mut up = 0;
            let mut down = 0;

            // Check left
            for height in trees[y][..x].iter().rev() {
                left += 1;

                if height >= tree {
                    break;
                }
            }

            // Check right
            if x < row.len() - 1 {
                for height in trees[y][x + 1..].iter() {
                    right += 1;

                    if height >= tree {
                        break;
                    }
                }
            }

            // Check up
            for height in trees[..y].iter().map(|line| &line[x]).rev() {
                up += 1;

                if height >= tree {
                    break;
                }
            }

            // Check down
            if y < trees.len() - 1 {
                for height in trees[y + 1..].iter().map(|line| &line[x]) {
                    down += 1;

                    if height >= tree {
                        break;
                    }
                }
            }

            max_scenic_score = (left * right * up * down).max(max_scenic_score);
        }
    }

    max_scenic_score
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

    #[test]
    fn can_solve_part2() {
        let sample_input = r#"30373
25512
65332
33549
35390"#;

        assert_eq!(solve_part2(sample_input), 8);
    }
}
