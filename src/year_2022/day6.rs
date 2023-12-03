use std::collections::HashSet;

const FRAME_SIZE: usize = 4;

pub fn solve_part1(input: &str) -> usize {
    let mut frame = HashSet::<char>::new();

    for i in 0..input.len() - FRAME_SIZE {
        for c in input[i..(i + FRAME_SIZE)].chars() {
            frame.insert(c);
        }

        if frame.len() == FRAME_SIZE {
            return i + FRAME_SIZE;
        }

        frame.clear();
    }

    return usize::MAX;
}

pub fn solve_part2(input: &str) -> usize {
    todo!()
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
}
