use std::cmp::max;
use std::vec;

const INPUT: &str = include_str!("day2_input.txt");

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Cubes {
    color: Color,
    count: u32,
}

#[derive(Debug)]
struct Game {
    sets: Vec<Vec<Cubes>>,
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games = vec![];

    for l in input.lines() {
        let l = l.split(": ").nth(1).unwrap();

        let sets: Vec<_> = l
            .split("; ")
            .map(|s| {
                s.split(", ")
                    .map(|s| {
                        let v: Vec<_> = s.split(' ').collect();
                        let count: u32 = v[0].parse().unwrap();

                        let color = match v[1] {
                            "red" => Color::Red,
                            "green" => Color::Green,
                            "blue" => Color::Blue,
                            _ => panic!("Unexpected color {}", v[1]),
                        };

                        Cubes { color, count }
                    })
                    .collect::<Vec<Cubes>>()
            })
            .collect();

        games.push(Game { sets });
    }

    games
}

fn solve_part1(input: &str) -> u32 {
    let (bag_red, bag_green, bag_blue) = (12, 13, 14);
    let games = parse_input(input);

    let mut result = 0;
    'games: for (idx, game) in games.iter().enumerate() {
        for s in game.sets.iter() {
            let mut reds = 0;
            let mut greens = 0;
            let mut blues = 0;

            for c in s.iter() {
                match c.color {
                    Color::Blue => {
                        blues += c.count;
                    }
                    Color::Green => {
                        greens += c.count;
                    }
                    Color::Red => {
                        reds += c.count;
                    }
                }
            }

            if reds > bag_red || greens > bag_green || blues > bag_blue {
                continue 'games;
            }
        }

        result += (idx + 1) as u32;
    }

    result
}

fn solve_part2(input: &str) -> u32 {
    let games = parse_input(input);

    games
        .iter()
        .map(|game| {
            let mut max_red = 0;
            let mut max_geen = 0;
            let mut max_blue = 0;

            for set in &game.sets {
                for cube in set {
                    match cube.color {
                        Color::Blue => {
                            max_blue = max(max_blue, cube.count);
                        }
                        Color::Green => {
                            max_geen = max(max_geen, cube.count);
                        }
                        Color::Red => {
                            max_red = max(max_red, cube.count);
                        }
                    }
                }
            }

            max_red * max_geen * max_blue
        })
        .sum()
}

fn main() {
    println!("Part 1: {}", solve_part1(INPUT));
    println!("Part 2: {}", solve_part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day2() {
        const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(solve_part1(TEST_INPUT), 8);
        assert_eq!(solve_part1(INPUT), 2476);

        assert_eq!(solve_part2(TEST_INPUT), 2286);
        assert_eq!(solve_part2(INPUT), 54911);
    }
}
