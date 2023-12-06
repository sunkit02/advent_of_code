use std::thread;

pub fn solve_part1(input: &str) -> u64 {
    let mut sections = input.split("\n\n");

    let seeds = sections.next().expect("Seeds");
    let seed_to_soil = sections.next().expect("Seed to soil");
    let soil_to_fertilizer = sections.next().expect("Soil to fertilizer");
    let fertilizer_to_water = sections.next().expect("Fertilizer to water");
    let water_to_light = sections.next().expect("Water to light");
    let light_to_temp = sections.next().expect("Light to temp");
    let temp_to_humidity = sections.next().expect("Temperature to humidity");
    let humidity_to_location = sections.next().expect("Humidity to location");

    let (_title, seeds) = seeds.split_once(':').expect("Split seed");
    let seeds: Vec<u64> = seeds
        .split(' ')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .collect();

    let seed_to_soil: Vec<Range> = seed_to_soil
        .lines()
        .skip(1)
        .map(|line| line.split(' '))
        .map(|splitted_line| splitted_line.map(|num_str| num_str.parse::<u64>().unwrap()))
        .map(|nums| nums.collect::<Vec<u64>>())
        .filter_map(|nums| Range::try_from(&nums[..]).ok())
        .collect();

    let soil_to_fertilizer: Vec<Range> = soil_to_fertilizer
        .lines()
        .skip(1)
        .map(|line| line.split(' '))
        .map(|splitted_line| splitted_line.map(|num_str| num_str.parse::<u64>().unwrap()))
        .map(|nums| nums.collect::<Vec<u64>>())
        .filter_map(|nums| Range::try_from(&nums[..]).ok())
        .collect();

    let fertilizer_to_water: Vec<Range> = fertilizer_to_water
        .lines()
        .skip(1)
        .map(|line| line.split(' '))
        .map(|splitted_line| splitted_line.map(|num_str| num_str.parse::<u64>().unwrap()))
        .map(|nums| nums.collect::<Vec<u64>>())
        .filter_map(|nums| Range::try_from(&nums[..]).ok())
        .collect();

    let water_to_light: Vec<Range> = water_to_light
        .lines()
        .skip(1)
        .map(|line| line.split(' '))
        .map(|splitted_line| splitted_line.map(|num_str| num_str.parse::<u64>().unwrap()))
        .map(|nums| nums.collect::<Vec<u64>>())
        .filter_map(|nums| Range::try_from(&nums[..]).ok())
        .collect();

    let light_to_temp: Vec<Range> = light_to_temp
        .lines()
        .skip(1)
        .map(|line| line.split(' '))
        .map(|splitted_line| splitted_line.map(|num_str| num_str.parse::<u64>().unwrap()))
        .map(|nums| nums.collect::<Vec<u64>>())
        .filter_map(|nums| Range::try_from(&nums[..]).ok())
        .collect();

    let temp_to_humidity: Vec<Range> = temp_to_humidity
        .lines()
        .skip(1)
        .map(|line| line.split(' '))
        .map(|splitted_line| splitted_line.map(|num_str| num_str.parse::<u64>().unwrap()))
        .map(|nums| nums.collect::<Vec<u64>>())
        .filter_map(|nums| Range::try_from(&nums[..]).ok())
        .collect();

    let humidity_to_location: Vec<Range> = humidity_to_location
        .lines()
        .skip(1)
        .map(|line| line.split(' '))
        .map(|splitted_line| splitted_line.map(|num_str| num_str.parse::<u64>().unwrap()))
        .map(|nums| nums.collect::<Vec<u64>>())
        .filter_map(|nums| Range::try_from(&nums[..]).ok())
        .collect();

    let seed_to_soil = RangeConverter(seed_to_soil);
    let soil_to_fertilizer = RangeConverter(soil_to_fertilizer);
    let fertilizer_to_water = RangeConverter(fertilizer_to_water);
    let water_to_light = RangeConverter(water_to_light);
    let light_to_temp = RangeConverter(light_to_temp);
    let temp_to_humidity = RangeConverter(temp_to_humidity);
    let humidity_to_location = RangeConverter(humidity_to_location);

    let mut min_location = u64::MAX;
    for seed in seeds {
        let soil = seed_to_soil.convert(seed);
        let fertilizer = soil_to_fertilizer.convert(soil);
        let water = fertilizer_to_water.convert(fertilizer);
        let light = water_to_light.convert(water);
        let temp = light_to_temp.convert(light);
        let humidity = temp_to_humidity.convert(temp);
        let location = humidity_to_location.convert(humidity);

        if location < min_location {
            min_location = location;
        }
    }

    min_location
}

pub fn solve_part2(input: &str) -> u64 {
    let mut sections = input.split("\n\n");

    let seeds = sections.next().expect("Seeds");

    let (_title, seeds) = seeds.split_once(':').expect("Split seed");
    let seeds: Vec<u64> = seeds
        .split(' ')
        .filter_map(|s| s.trim().parse::<u64>().ok())
        .collect();

    let converters: Vec<RangeConverter> = sections
        .map(|section| {
            section
                .lines()
                .skip(1)
                .map(|line| line.split(' '))
                .map(|splitted_line| splitted_line.map(|num_str| num_str.parse::<u64>().unwrap()))
                .map(|nums| nums.collect::<Vec<u64>>())
                .filter_map(|nums| Range::try_from(&nums[..]).ok())
                .collect()
        })
        .map(|ranges| RangeConverter(ranges))
        .collect();

    println!("Finished parsing");

    let mut handles = vec![];

    for i in (0..seeds.len() - 1).step_by(2) {
        let low = seeds[i];
        let high = seeds[i] + seeds[i + 1];

        let converters = converters.clone();

        handles.push(thread::spawn(move || {
            let mut min_location = u64::MAX;

            for seed in low..high {
                let mut value = seed;
                for converter in &converters {
                    value = converter.convert(value);
                }

                if value < min_location {
                    min_location = value;
                }
            }

            min_location
        }));
    }

    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .min()
        .unwrap()
}

#[derive(Clone)]
struct RangeConverter(Vec<Range>);

impl RangeConverter {
    fn convert(&self, value: u64) -> u64 {
        for range in &self.0 {
            if value >= range.src && value < range.src + range.len {
                return range.dst + (value - range.src);
            }
        }

        value
    }
}

#[derive(Clone)]
struct Range {
    dst: u64,
    src: u64,
    len: u64,
}

impl TryFrom<&[u64]> for Range {
    type Error = &'static str;

    fn try_from(value: &[u64]) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err("Length must be 3 or greater");
        }

        Ok(Self {
            dst: value[0],
            src: value[1],
            len: value[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_solve_part1() {
        let sample_input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

        assert_eq!(solve_part1(sample_input), 35);
    }

    #[test]
    fn can_solve_part2() {
        let sample_input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

        assert_eq!(solve_part2(sample_input), 46);
    }
}
