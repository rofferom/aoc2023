use std::ops::RangeInclusive;
use std::{collections::HashSet, hash::Hash};

const INPUT: &str = include_str!("day3_input.txt");

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum ItemType {
    Symbol(char),
    Part(u32),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Item {
    x: RangeInclusive<i32>,
    y: i32,
    type_: ItemType,
}

impl Item {
    fn matches_pos(&self, x: i32, y: i32) -> bool {
        self.y == y && self.x.contains(&x)
    }

    fn part(x: i32, y: i32, c: char) -> Self {
        Self {
            x: x..=x,
            y,
            type_: ItemType::Part(c.to_digit(10).unwrap()),
        }
    }

    fn symbol(x: i32, y: i32, s: char) -> Self {
        Self {
            x: x..=x,
            y,
            type_: ItemType::Symbol(s),
        }
    }
}

struct Schematic {
    items: Vec<Item>,
    columns: i32,
    rows: i32,
}

impl Schematic {
    fn parts_candidates(&self) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|&i| matches!(i.type_, ItemType::Part(_)))
            .collect()
    }
}

fn parse_line(line: &str, row_idx: usize, items: &mut Vec<Item>) {
    let mut item: Option<&mut Item> = None;

    for (column_idx, c) in line.chars().enumerate() {
        let column_idx = column_idx as i32;
        let row_idx = row_idx as i32;
        let is_digit = c.is_ascii_digit();

        match item {
            Some(Item {
                type_: ItemType::Symbol(_),
                ..
            }) => {
                if is_digit {
                    // Symbol => Part
                    items.push(Item::part(column_idx, row_idx, c));
                    item = items.last_mut();
                } else if c != '.' {
                    // Symbol => Symbol
                    items.push(Item::symbol(column_idx, row_idx, c));
                    item = None;
                } else {
                    // Symbol => Nothing
                    item = None;
                }
            }
            Some(Item {
                x,
                type_: ItemType::Part(number),
                ..
            }) => {
                if is_digit {
                    // Part continues
                    *number = *number * 10 + c.to_digit(10).unwrap();
                    *x = *x.start()..=column_idx;
                } else if c != '.' {
                    // Part => Symbol
                    items.push(Item::symbol(column_idx, row_idx, c));
                    item = None;
                } else {
                    // Part => Nothing
                    item = None;
                }
            }
            None => {
                if is_digit {
                    // Nothing => Part
                    items.push(Item::part(column_idx, row_idx, c));
                    item = items.last_mut();
                } else if c != '.' {
                    // Nothing => Symbol
                    items.push(Item::symbol(column_idx, row_idx, c));
                    item = None;
                }
            }
        }
    }
}

fn parse_schematic(input: &str) -> Schematic {
    let mut items = vec![];
    let mut columns = None;
    let mut rows = 0;

    input.split('\n').enumerate().for_each(|(row_idx, line)| {
        rows += 1;

        if columns.is_none() {
            columns = Some(line.len());
        }

        parse_line(line, row_idx, &mut items);
    });

    Schematic {
        items,
        columns: columns.unwrap() as _,
        rows,
    }
}

fn solve_part1(input: &str) -> u32 {
    let schematic = parse_schematic(input);
    let parts_candidates = schematic.parts_candidates();

    let mut valid_parts = HashSet::new();

    for symbol in schematic
        .items
        .iter()
        .filter(|i| matches!(i.type_, ItemType::Symbol(_)))
    {
        let directions = &[
            // X axis
            (-1, 0),
            (1, 0),
            // Y axis
            (0, -1),
            (0, 1),
            // Diagonals
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        for (offset_x, offset_y) in directions {
            let mut x = *symbol.x.start();
            let mut y = symbol.y;

            x += offset_x;
            y += offset_y;

            if x < 0 || x >= schematic.columns || y < 0 || y >= schematic.rows {
                continue;
            }

            if let Some(&part) = parts_candidates
                .iter()
                .find(|&&part| part.matches_pos(x, y))
            {
                valid_parts.insert(part);
            }
        }
    }

    valid_parts
        .iter()
        .map(|&part| {
            let ItemType::Part(part) = part.type_ else {
                panic!("Should be a part")
            };

            part
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    let schematic = parse_schematic(input);
    let parts_candidates = schematic.parts_candidates();

    let mut ratio_sum = 0;

    for symbol in schematic
        .items
        .iter()
        .filter(|i| matches!(i.type_, ItemType::Symbol('*')))
    {
        let directions = &[
            // X axis
            (-1, 0),
            (1, 0),
            // Y axis
            (0, -1),
            (0, 1),
            // Diagonals
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];

        let mut matching_parts = HashSet::new();

        for (offset_x, offset_y) in directions {
            let mut x = *symbol.x.start();
            let mut y = symbol.y;

            x += offset_x;
            y += offset_y;

            if x < 0 || x >= schematic.columns || y < 0 || y >= schematic.rows {
                continue;
            }

            if let Some(&part) = parts_candidates
                .iter()
                .find(|&&part| part.matches_pos(x, y))
            {
                matching_parts.insert(part);
            }
        }

        if matching_parts.len() != 2 {
            continue;
        }

        ratio_sum += matching_parts
            .iter()
            .map(|&part| {
                let ItemType::Part(part) = part.type_ else {
                    panic!("Should be a part")
                };

                part
            })
            .product::<u32>();
    }

    ratio_sum
}

fn main() {
    // > 352547
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day3() {
        const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(solve_part1(TEST_INPUT), 4361);
        assert_eq!(solve_part1(INPUT), 543867);

        assert_eq!(solve_part2(TEST_INPUT), 467835);
        assert_eq!(solve_part2(INPUT), 79613331);
    }
}
