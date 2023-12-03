use std::collections::HashSet;

const FRAME_SIZE_1: usize = 4;
const FRAME_SIZE_2: usize = 14;

pub fn solve_part1(input: &str) -> usize {
    let mut frame = HashSet::<char>::new();

    for i in 0..input.len() - FRAME_SIZE_1 {
        for c in input[i..(i + FRAME_SIZE_1)].chars() {
            frame.insert(c);
        }

        if frame.len() == FRAME_SIZE_1 {
            return i + FRAME_SIZE_1;
        }

        frame.clear();
    }

    return usize::MAX;
}

pub fn solve_part2(input: &str) -> usize {
    let mut frame = HashSet::<char>::new();

    for i in 0..input.len() - FRAME_SIZE_2 {
        for c in input[i..(i + FRAME_SIZE_2)].chars() {
            frame.insert(c);
        }

        if frame.len() == FRAME_SIZE_2 {
            return i + FRAME_SIZE_2;
        }

        frame.clear();
    }

    return usize::MAX;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solve_part1(sample_input), 5);

        let sample_input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(solve_part1(sample_input), 6);

        let sample_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(solve_part1(sample_input), 10);

        let sample_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solve_part1(sample_input), 11);
    }

    #[test]
    fn can_solve_part2() {
        let sample_input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solve_part2(sample_input), 19);

        let sample_input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solve_part2(sample_input), 23);

        let sample_input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(solve_part2(sample_input), 23);

        let sample_input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(solve_part2(sample_input), 29);

        let sample_input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solve_part2(sample_input), 26);
    }
}
