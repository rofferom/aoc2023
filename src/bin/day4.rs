use std::collections::HashSet;

const INPUT: &str = include_str!("day4_input.txt");

#[derive(Debug)]
struct Card {
    own: HashSet<i32>,
    winning: HashSet<i32>,
}

fn solve_part1(input: &str) -> u32 {
    let split_values = |s: &str| -> HashSet<i32> {
        s.split(' ')
            .filter_map(|s| {
                let s = s.trim();
                if s.is_empty() {
                    None
                } else {
                    Some(s)
                }
            })
            .map(|s| s.parse::<i32>().unwrap())
            .collect()
    };

    let cards: Vec<_> = input
        .split('\n')
        .map(|l| {
            let parts: Vec<_> = l.split(": ").collect();
            let parts: Vec<_> = parts[1].split(" | ").collect();

            let own = split_values(parts[0]);
            let winning = split_values(parts[1]);

            Card { own, winning }
        })
        .collect();

    cards
        .iter()
        .map(|c| {
            let score = c.winning.intersection(&c.own).count() as u32;
            if score > 0 {
                2u32.pow(score - 1)
            } else {
                score
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    0
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day4() {
        const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(solve_part1(TEST_INPUT), 13);
        assert_eq!(solve_part1(INPUT), 28750);
    }
}
