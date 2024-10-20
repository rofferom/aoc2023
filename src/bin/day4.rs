use std::collections::HashSet;

const INPUT: &str = include_str!("day4_input.txt");

#[derive(Debug)]
struct Card {
    own: HashSet<i32>,
    winning: HashSet<i32>,
    count: usize,
}

impl Card {
    fn matching(&self) -> usize {
        self.winning.intersection(&self.own).count()
    }
}

fn load_cards(input: &str) -> Vec<Card> {
    let split_values = |s: &str| -> HashSet<i32> {
        s.split(' ')
            .filter_map(|s| {
                let s = s.trim();
                if !s.is_empty() {
                    Some(s.parse().unwrap())
                } else {
                    None
                }
            })
            .collect()
    };

    input
        .split('\n')
        .map(|l| {
            let parts: Vec<_> = l.split(": ").nth(1).unwrap().split(" | ").collect();

            Card {
                own: split_values(parts[0]),
                winning: split_values(parts[1]),
                count: 1,
            }
        })
        .collect()
}

fn solve_part1(input: &str) -> u32 {
    load_cards(input)
        .iter()
        .map(|c| {
            let matching = c.matching();
            if matching > 0 {
                2u32.pow((matching - 1) as _)
            } else {
                0
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let mut cards = load_cards(input);

    for i in 0..cards.len() {
        let matching = cards[i].matching();

        let start = i + 1;
        let mut end = start + matching;
        if end > cards.len() {
            end = cards.len() - 1;
        };

        for j in start..end {
            cards[j].count += cards[i].count;
        }
    }

    cards.iter().map(|c| c.count as u32).sum()
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

        assert_eq!(solve_part2(TEST_INPUT), 30);
        assert_eq!(solve_part2(INPUT), 10212704);
    }
}
