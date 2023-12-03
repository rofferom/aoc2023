const INPUT: &str = include_str!("day1_input.txt");

fn parse_input(input: &str) -> Vec<String> {
    input.split('\n').map(|s| s.to_string()).collect()
}

fn solve_part1(input: &str) -> u32 {
    let values = parse_input(input);

    values
        .iter()
        .map(|s| {
            let digits: Vec<_> = s.chars().filter_map(|v| v.to_digit(10)).collect();
            digits.first().unwrap() * 10 + digits.last().unwrap()
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    const DIGIT_MAP: &[(&str, u32)] = &[
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    let values = parse_input(input);
    let mut sum = 0;

    for v in values {
        let mut digits = vec![];

        for i in 0..v.len() {
            if let Some(digit) = v.chars().nth(i).unwrap().to_digit(10) {
                digits.push(digit)
            } else {
                for (digit_str, digit) in DIGIT_MAP {
                    if v.len() - i < digit_str.len() {
                        continue;
                    }

                    let substr: String = v.chars().skip(i).take(digit_str.len()).collect();
                    if digit_str == &substr {
                        digits.push(*digit)
                    }
                }
            }
        }

        sum += digits.first().unwrap() * 10 + digits.last().unwrap();
    }

    sum
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        const TEST_INPUT1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(solve_part1(TEST_INPUT1), 142);
        assert_eq!(solve_part1(INPUT), 55130);

        const TEST_INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(solve_part2(TEST_INPUT2), 281);
        assert_eq!(solve_part2(INPUT), 54985);
    }
}
