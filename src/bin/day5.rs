const INPUT: &str = include_str!("day5_input.txt");

#[derive(Debug)]
struct CategoryMapEntry {
    destination: usize,
    source: usize,
    range: usize,
}

#[derive(Debug)]
struct CategoryMap {
    entries: Vec<CategoryMapEntry>,
}

impl CategoryMap {
    fn new() -> Self {
        Self { entries: vec![] }
    }

    fn add_entry(&mut self, entry: CategoryMapEntry) {
        self.entries.push(entry)
    }

    fn map(&self, v: usize) -> usize {
        for e in &self.entries {
            let range = e.source..=e.source + e.range;
            if range.contains(&v) {
                return e.destination + v - e.source;
            }
        }

        v
    }
}

fn solve_part1(input: &str) -> usize {
    let mut input_line_reader = input.split('\n');

    // Parse seeds
    let seeds: Vec<usize> = input_line_reader
        .next()
        .unwrap()
        // Remove "seeds: " prefix
        .split(": ")
        .nth(1)
        .unwrap()
        // Parse int list
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();

    input_line_reader.next();

    // Parse stages
    let mut stage_maps: Vec<CategoryMap> = vec![];
    'outer: loop {
        // Ignore CategoryMap title
        input_line_reader.next();

        stage_maps.push(CategoryMap::new());
        let current_stage = stage_maps.last_mut().unwrap();

        loop {
            let line = match input_line_reader.next() {
                Some(line) => {
                    if line.is_empty() {
                        break;
                    }

                    line
                }
                None => break 'outer,
            };

            let line: Vec<usize> = line.split(' ').map(|n| n.parse().unwrap()).collect();
            current_stage.add_entry(CategoryMapEntry {
                destination: line[0],
                source: line[1],
                range: line[2],
            });
        }
    }

    // Process seeds
    seeds
        .iter()
        .map(|&seed| {
            let mut v = seed;

            for stage in &stage_maps {
                v = stage.map(v);
            }

            v
        })
        .min()
        .unwrap()
}

fn solve_part2(input: &str) -> usize {
    let mut input_line_reader = input.split('\n');

    // Parse seeds
    #[derive(Debug)]
    struct SeedRange {
        start: usize,
        size: usize,
    }

    let seeds: Vec<usize> = input_line_reader
        .next()
        .unwrap()
        // Remove "seeds: " prefix
        .split(": ")
        .nth(1)
        .unwrap()
        // Parse int list
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut seeds: Vec<_> = seeds
        .chunks(2)
        .map(|values| SeedRange {
            start: values[0],
            size: values[1],
        })
        .collect();
    seeds.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    input_line_reader.next();

    // Parse stages
    let mut stage_maps: Vec<CategoryMap> = vec![];
    'outer: loop {
        // Ignore CategoryMap title
        input_line_reader.next();

        stage_maps.push(CategoryMap::new());
        let current_stage = stage_maps.last_mut().unwrap();

        loop {
            let line = match input_line_reader.next() {
                Some(line) => {
                    if line.is_empty() {
                        break;
                    }

                    line
                }
                None => break 'outer,
            };

            let line: Vec<usize> = line.split(' ').map(|n| n.parse().unwrap()).collect();
            current_stage.add_entry(CategoryMapEntry {
                destination: line[0],
                source: line[1],
                range: line[2],
            });
        }
    }

    // Process seeds
    let mut min_location = usize::MAX;
    let mut last_seed = 0;

    for mut seed in seeds {
        // Some seed ranges are overlaping
        if last_seed > seed.start {
            seed.start = last_seed;
            last_seed = seed.start + seed.size - 1;
        }

        for seed in seed.start..seed.start + seed.size {
            let mut value = seed;

            for stage in &stage_maps {
                value = stage.map(value);
            }

            if value < min_location {
                min_location = value;
            }
        }
    }

    min_location
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day5() {
        const TEST_INPUT: &str = "seeds: 79 14 55 13

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
56 93 4";

        assert_eq!(solve_part1(TEST_INPUT), 35);
        assert_eq!(solve_part1(INPUT), 322500873);

        assert_eq!(solve_part2(TEST_INPUT), 46);
        assert_eq!(solve_part2(INPUT), 108956227);
    }
}
