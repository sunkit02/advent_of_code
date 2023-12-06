pub fn solve_part1(input: &str) -> usize {
    let races = parse_input1(input);
    races
        .into_iter()
        .map(|race| {
            (0..=race.time)
                .filter(|hold_time| {
                    let dist = hold_time * (race.time - hold_time);
                    dist > race.record_dist
                })
                .count()
        })
        .product()
}

pub fn solve_part2(input: &str) -> usize {
    let race = parse_input2(input);
    (0..=race.time)
        .filter(|hold_time| {
            let dist = hold_time * (race.time - hold_time);
            dist > race.record_dist
        })
        .count()
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_dist: u64,
}

fn parse_input1(input: &str) -> Vec<Race> {
    let (times, dists) = input.split_once('\n').expect("Should have two lines");

    let (_title, times_str) = times.split_once(':').expect("Wrong time format");
    let mut times = vec![];
    let mut buf = String::new();
    for c in times_str.chars() {
        if c.is_numeric() {
            buf.push(c);
        } else if !buf.is_empty() {
            times.push(buf.parse::<u64>().expect("Failed to parse number"));
            buf.clear();
        }
    }
    // Clear up buf last time
    if !buf.is_empty() {
        times.push(buf.parse::<u64>().expect("Failed to parse number"));
    }

    let (_title, dists_str) = dists.split_once(':').expect("Wrong time format");
    let mut dists = vec![];
    let mut buf = String::new();
    for c in dists_str.chars() {
        if c.is_numeric() {
            buf.push(c);
        } else if !buf.is_empty() {
            dists.push(buf.parse::<u64>().expect("Failed to parse number"));
            buf.clear();
        }
    }
    // Clear up buf last time
    if !buf.is_empty() {
        dists.push(buf.parse::<u64>().expect("Failed to parse number"));
    }

    times
        .into_iter()
        .zip(dists.into_iter())
        .map(|(time, dist)| Race {
            time,
            record_dist: dist,
        })
        .collect()
}

fn parse_input2(input: &str) -> Race {
    let (time, dist) = input.split_once('\n').expect("Should have two lines");

    let (_title, time_str) = time.split_once(':').expect("Wrong time format");
    let mut buf = String::new();
    for c in time_str.chars() {
        if c.is_numeric() {
            buf.push(c);
        }
    }
    let time = if !buf.is_empty() {
        buf.parse::<u64>().expect("Failed to parse number")
    } else {
        panic!("No time found");
    };

    let (_title, dist_str) = dist.split_once(':').expect("Wrong dist format");
    let mut buf = String::new();
    for c in dist_str.chars() {
        if c.is_numeric() {
            buf.push(c);
        }
    }
    let dist = if !buf.is_empty() {
        buf.parse::<u64>().expect("Failed to parse number")
    } else {
        panic!("No dist found");
    };

    Race {
        time,
        record_dist: dist,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(solve_part1(sample_input), 288);
    }

    #[test]
    fn can_solve_part2() {
        let sample_input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(solve_part2(sample_input), 71503);
    }
}
