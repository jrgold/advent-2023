use std::time::Instant;
use crate::day::Day;

mod day {
    use std::fmt::Display;

    pub trait Day {
        fn process_input(&self, input: &str) -> Box<dyn Solution>;
    }

    pub trait Solution {
        fn part_1(&self) -> Box<dyn Display>;
        fn part_2(&self) -> Box<dyn Display>;
    }
}

fn main() {
    let day_number = 4;

    let day: &dyn Day = match day_number {
        1 => &day_01::Day01{},
        2 => &day_02::Day02{},
        3 => &day_03::Day03{},
        4 => &day_04::Day04{},
        _ => unreachable!(),
    };

    let input_filename = format!("input/day_{:02}.txt", day_number);
    // let input_filename = "input/day_04_sample.txt";

    let input = std::fs::read_to_string(&input_filename).unwrap();

    let input_processing_start = Instant::now();
    let solution = day.process_input(&input);
    let input_processing_time = input_processing_start.elapsed();
    println!("Input processing time: {}ms", input_processing_time.as_millis());

    let part_1_start = Instant::now();
    let part_1_answer = solution.part_1();
    let part_1_time = part_1_start.elapsed();
    println!("{}", part_1_answer);
    println!("Part 1 time: {}ms", part_1_time.as_millis());

    let part_2_start = Instant::now();
    let part_2_answer = solution.part_2();
    let part_2_time = part_2_start.elapsed();
    println!("{}", part_2_answer);
    println!("Part 2 time: {}ms", part_2_time.as_millis());
}

mod day_01 {
    use std::fmt::Display;
    use regex::Regex;
    use crate::day::{Day, Solution};

    pub struct Day01;
    struct Input {
        input: Vec<String>
    }

    impl Day for Day01 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let input = input.lines()
                .map(|line| line.to_owned())
                .collect();

            Box::new(Input {
                input
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer = self.input.iter()
                .map(|line| {
                    let first = line.chars().filter(|c| c.is_numeric()).nth(0).unwrap();
                    let last = line.chars().rev().filter(|c| c.is_numeric()).nth(0).unwrap();
                    (first as i32 - '0' as i32) * 10 + (last as i32 - '0' as i32)
                }).sum::<i32>();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let forward_pattern = Regex::new("^.*?([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
            let backward_pattern = Regex::new("^.*?([0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();

            let answer = self.input.iter().map(|line| {
                let first = forward_pattern.captures(line).unwrap().get(1).unwrap().as_str();
                let reversed = &line.chars().rev().collect::<String>();
                let last = backward_pattern.captures(reversed).unwrap().get(1).unwrap().as_str();
                to_num(first) * 10 + to_num(&last.chars().rev().collect::<String>())
            })
            .sum::<i32>();

            Box::new(answer)
        }
    }

    fn to_num(s: &str) -> i32 {
        match s {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => {
                s.parse().unwrap()
            },
        }
    }
}

mod day_02 {
    use std::collections::HashMap;
    use std::fmt::Display;
    use std::str::FromStr;
    use crate::day::{Day, Solution};

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    enum Colour { R, B, G }

    impl FromStr for Colour {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "red" => Ok(Self::R),
                "blue" => Ok(Self::B),
                "green" => Ok(Self::G),
                _ => Err(()),
            }
        }
    }

    pub struct Day02;
    struct Input {
        input: Vec<(i32, Vec<Vec<(Colour, i32)>>)>
    }

    impl Day for crate::day_02::Day02 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let input = input.lines()
                .map(|line| {
                    let (name, rest) = line.split_once(": ").unwrap();
                    let games = rest.split("; ")
                        .map(|g|
                            g.split(", ").map(|b| {
                                let (n, c) = b.split_once(" ").unwrap();
                                (c.parse().unwrap(), n.parse().unwrap())
                            }).collect()
                        ).collect();
                    (name.split_once(" ").unwrap().1.parse().unwrap(), games)
                }).collect();

            Box::new(Input {
                input
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let limits: HashMap::<Colour, i32> = HashMap::from([(Colour::R, 12), (Colour::G, 13), (Colour::B, 14)]);
            let answer: i32 = self.input.iter()
                .filter_map(|(i, rounds)| {
                    let illegal = rounds.iter()
                        .flat_map(|r| r.iter())
                        .any(|(col, n)| n > limits.get(col).unwrap());
                    if !illegal { Some(i) } else { None }
                })
                .sum();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let answer: i32 = self.input.iter()
                .map(|(_, rounds)| {
                    let mut mins = HashMap::new();
                    rounds.iter()
                        .flat_map(|r| r.iter())
                        .for_each(|(col, n)| {
                            mins.entry(*col)
                                .and_modify(|old: &mut i32| *old = (*old).max(*n))
                                .or_insert(*n);
                        });
                    mins.values().product::<i32>()
                }).sum();

            Box::new(answer)
        }
    }
}
mod day_03 {
    use std::collections::{HashMap, HashSet};
    use std::fmt::Display;
    use crate::day::{Day, Solution};

    pub struct Day03;

    struct Input {
        // numbers and their adjacent characters
        numbers: Vec<(i32, HashSet<((i32, i32), char)>)>
    }

    impl Day for Day03 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let input: HashMap<(i32, i32), char> = input.lines()
                .enumerate()
                .flat_map(|(y, line)|
                    line.chars()
                        .enumerate()
                        .map(move |(x, c)| ((x as i32, y as i32), c))
                )
                .collect();

            let x_max = input.keys().max_by_key(|p| p.0).unwrap().0;
            let y_max = input.keys().max_by_key(|p| p.1).unwrap().1;

            let mut adjacent_symbols: HashMap<(i32, i32), HashSet<((i32, i32), char)>> = HashMap::new();
            input.iter().for_each(|(&(x, y), &c)|
                    if (c < '0' || c > '9') && c != '.' {
                        vec![
                            (x - 1 ,y - 1),
                            (x - 1, y    ),
                            (x - 1, y + 1),
                            (x     ,y - 1),
                            (x    , y + 1),
                            (x + 1 ,y - 1),
                            (x + 1, y    ),
                            (x + 1, y + 1)
                        ].iter().for_each(|coord| {
                            let e = adjacent_symbols.entry(*coord).or_insert_with(|| HashSet::new());
                            e.insert(((x, y), c));
                        })
                    }
                );

            let mut numbers = vec![];

            let mut running: Option<i32> = None;
            let mut adjacents: Option<HashSet<((i32, i32), char)>> = None;

            let empty_set = HashSet::new();

            for y in 0..=y_max {
                for x in 0..=x_max {
                    let c = *input.get(&(x, y)).unwrap();
                    match c {
                        '0' ..= '9' => {
                            adjacents = Some(adjacents.unwrap_or_else(|| HashSet::new()));
                            adjacents.as_mut().unwrap().extend(adjacent_symbols.get(&(x, y)).unwrap_or_else(|| &empty_set));
                            running = Some(running.unwrap_or(0) * 10 + c as i32 - '0' as i32);
                        }
                        _ => {
                            if running.is_some() {
                                numbers.push((running.unwrap(), adjacents.clone().unwrap()));
                            }
                            running = None;
                            adjacents = None;
                        }
                    }
                }
                if running.is_some() {
                    numbers.push((running.unwrap(), adjacents.clone().unwrap()));
                }
                running = None;
                adjacents = None;
            }

            Box::new(Input {
                numbers
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer: i32 = self.numbers.iter()
                .filter(|(_, symbols)| symbols.len() > 0)
                .map(|(num, _)| num)
                .sum();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let mut gears: HashMap::<(i32, i32), Vec<i32>> = HashMap::new();

            self.numbers.iter()
                .flat_map(|(num, adjacents)|
                    adjacents.into_iter().map(move |(coord, _)| (num, coord)))
                .for_each(|(num, coord)| {
                    let nums = gears.entry(*coord).or_insert(vec![]);
                    nums.push(*num);
                });

            let answer: i32 = gears.into_iter()
                .filter(|(_, nums)| nums.len() == 2)
                .map(|(_, nums)| nums.iter().product::<i32>())
                .sum();

            Box::new(answer)
        }
    }
}

mod day_04 {
    use std::collections::{HashMap, HashSet};
    use std::fmt::Display;
    use regex::Regex;
    use crate::day::{Day, Solution};

    pub struct Day04;

    struct Input {
        // numbers and their adjacent characters
        input: Vec<(i32, HashSet<i32>, Vec<i32>)>,
    }

    impl Day for Day04 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let regex = Regex::new("^Card *([0-9]*): ([^|]*)\\|(.*)$").unwrap();

            let input = input.lines()
                .map(|line| {
                    let captures = regex.captures(line).unwrap();

                    let id = captures.get(1).unwrap().as_str().parse().unwrap();
                    let winners: Vec<i32> = captures.get(2).unwrap().as_str()
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect();
                    let ours: Vec<i32> = captures.get(3).unwrap().as_str()
                        .split(' ')
                        .filter(|s| !s.is_empty())
                        .map(|s| s.parse().unwrap())
                        .collect();

                    (
                        id,
                        HashSet::from_iter(winners),
                        ours
                    )
                }).collect();

            Box::new(Input {
                input
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer: i32 = self.input.iter()
                .map(|(_id, winners, ours)| {
                    let matches = ours.iter().filter(|n| winners.contains(n)).count();
                    if matches > 0 { 2i32.pow(matches as u32 - 1) } else { 0 }
                }).sum();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let winnings_per_card: HashMap<i32, Vec<i32>> = self.input.iter()
                .map(|(id, winners, ours)| {
                    let matches = ours.iter().filter(|n| winners.contains(n)).count();
                    let winnings = (1..=matches).into_iter()
                        .map(|i| id + i as i32)
                        .collect();
                    (*id, winnings)
                }).collect();

            let mut total_cards: HashMap<i32, i32> = self.input.iter()
                .map(|(id, _, _)| (*id, 1))
                .collect();

            for i in 1..=self.input.last().unwrap().0 {
                let n = *total_cards.get(&i).unwrap_or(&0);
                let winnings = winnings_per_card.get(&i);
                if let Some(winnings) = winnings {
                    winnings.iter().for_each(|c| {
                        *(total_cards.get_mut(c).unwrap()) += n;
                    });
                }
            }

            let answer: i32 = total_cards.values().sum();

            Box::new(answer)
        }
    }
}
