const INPUT: &str = include_str!("day6_input.txt");

#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn run(&self) -> usize {
        let mut winning = 0;

        for d in 1..self.time {
            let remain = self.time - d;
            let distance = remain * d;

            if distance > self.distance {
                winning += 1;
            }
        }

        winning
    }
}

fn parse_input1(input: &str) -> Vec<Race> {
    let lines: Vec<Vec<usize>> = input
        .split('\n')
        .map(|l| l.split(':').nth(1).map(|l| l.trim()).unwrap())
        .map(|l| {
            l.split(' ')
                .filter_map(|s| {
                    if s.is_empty() {
                        return None;
                    }

                    Some(s.parse().unwrap())
                })
                .collect()
        })
        .collect();
    assert_eq!(lines.len(), 2);

    let mut races = vec![];

    for i in 0..lines[0].len() {
        races.push(Race {
            time: lines[0][i],
            distance: lines[1][i],
        });
    }

    races
}

fn solve_part1(input: &str) -> usize {
    parse_input1(input).iter().map(|r| r.run()).product()
}

fn parse_input2(input: &str) -> Race {
    let lines: Vec<_> = input
        .split('\n')
        .map(|l| l.split(':').nth(1).map(|l| l.trim()).unwrap())
        .map(|l| l.chars().filter(|&c| c != ' ').collect())
        .map(|l: String| l.parse().unwrap())
        .collect();
    assert_eq!(lines.len(), 2);

    Race {
        time: lines[0],
        distance: lines[1],
    }
}

fn solve_part2(input: &str) -> usize {
    parse_input2(input).run()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day6() {
        const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

        assert_eq!(solve_part1(TEST_INPUT), 288);
        assert_eq!(solve_part1(INPUT), 131376);

        assert_eq!(solve_part2(TEST_INPUT), 71503);
        assert_eq!(solve_part2(INPUT), 34123437);
    }
}
