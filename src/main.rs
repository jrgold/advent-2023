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
    let day_number = 9;

    let day: &dyn Day = match day_number {
        1 => &day_01::Day01{},
        2 => &day_02::Day02{},
        3 => &day_03::Day03{},
        4 => &day_04::Day04{},
        5 => &day_05::Day05{},
        6 => &day_06::Day06{},
        7 => &day_07::Day07{},
        8 => &day_08::Day08{},
        9 => &day_09::Day09{},
        _ => unreachable!(),
    };

    let input_filename = format!("input/day_{:02}.txt", day_number);
    // let input_filename = "input/day_09_sample.txt";

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

mod day_05 {
    use std::collections::BTreeMap;
    use std::fmt::Display;
    use crate::day::{Day, Solution};

    pub struct Day05;

    #[derive(Debug, Clone, Copy)]
    struct Conversion {
        from_start: i64,
        to_start: i64,
        length: i64,
    }

    struct Input {
        seeds: Vec<i64>,
        mappings: Vec<BTreeMap<i64, Conversion>>,
    }

    impl Day for Day05 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let mut paragraphs = input.split("\n\n");

            let seeds: Vec<i64> = paragraphs.next().unwrap()[7..].split(' ').map(|s| s.parse().unwrap()).collect();

            // let mappingRegex = Regex::new("^([^-]*)-to-([^-]*) map:$").unwrap();
            let mappings = paragraphs.map(|para|
                para.lines()
                    .skip(1)
                    .map(|line| {
                        let mut split = line.split(' ');
                        let conv = Conversion {
                            to_start: split.next().unwrap().parse().unwrap(),
                            from_start: split.next().unwrap().parse().unwrap(),
                            length: split.next().unwrap().parse().unwrap(),
                        };
                        (conv.from_start, conv)
                    }).collect()
            ).collect();

            Box::new(Input {
                seeds,
                mappings,
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let mut numbers = self.seeds.clone();

            for mapping in &self.mappings {
                numbers.iter_mut()
                    .for_each(|x| {
                        let conv = mapping.values()
                            .filter(|conv| *x >= conv.from_start && (*x - (conv.length - 1)) <= conv.from_start)
                            .nth(0)
                            .unwrap_or(&Conversion { from_start: 0, to_start: 0, length: 0 });

                        *x = *x - conv.from_start + conv.to_start;
                    })
            }

            let answer: i64 = numbers.into_iter().min().unwrap();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let seed_ranges = self.seeds.chunks(2).map(|ss| (ss[0], ss[0] + ss[1] - 1)).collect();

            let mut vec_0: Vec<(i64, i64)> = seed_ranges;
            let mut vec_1: Vec<(i64, i64)> = vec![];

            let input_ranges = &mut vec_0;
            let output_ranges = &mut vec_1;

            for mapping in &self.mappings {
                'next_input_range: for (mut start, end) in input_ranges.iter() {
                    if start > *end {
                        break 'next_input_range;
                    }
                    let possibles = mapping.range(0..=start).last().into_iter().chain(mapping.range(start..=*end)).map(|(_, c)| c);
                    'next_possibility: for possible in possibles {
                        let range_start = possible.from_start;
                        let range_end = range_start + possible.length - 1;

                        let range_diff = possible.to_start - possible.from_start;

                        if start > range_end {
                            // a   ()
                            // r ()
                            continue 'next_possibility;
                        } else if start >= range_start && *end > range_end {
                            // a (   )
                            // r ( )
                            output_ranges.push((start + range_diff, range_end + range_diff));
                            start = range_end + 1;
                        } else if start >= range_start && *end <= range_end {
                            // a (  )
                            // r (    )
                            output_ranges.push((start + range_diff, *end + range_diff));
                            continue 'next_input_range;
                        } else if start < range_start && *end <= range_end {
                            // a (   )
                            // r   ( )
                            output_ranges.push((start, range_start - 1));
                            output_ranges.push((range_start + range_diff, *end + range_diff));
                            continue 'next_input_range;
                        } else if start < range_start && *end > range_end {
                            // a (     )
                            // r   ( )
                            output_ranges.push((start, range_start - 1));
                            output_ranges.push((range_start + range_diff, range_end + range_diff));
                            start = range_end + 1;
                        }
                    }
                    if start <= *end {
                        output_ranges.push((start, *end));
                    }
                }

                std::mem::swap(input_ranges, output_ranges);
                output_ranges.truncate(0);
            }

            let answer: i64 = input_ranges.iter()
                .min_by_key(|r| r.0)
                .unwrap()
                .0;

            Box::new(answer)
        }
    }
}

mod day_06 {
    use std::fmt::Display;
    use num::BigInt;
    use crate::day::{Day, Solution};

    pub struct Day06;
    struct Input {
        input: Vec<(String, String)>
    }

    impl Day for Day06 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let mut lines = input.lines();
            let time_line = lines.next().unwrap();
            let record_line = lines.next().unwrap();

            let input = time_line.split_ascii_whitespace().skip(1)
                .zip(record_line.split_ascii_whitespace().skip(1))
                .map(|(time, record)| (time.to_owned(), record.to_owned())).collect();

            Box::new(Input {
                input
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer: i32 = self.input.iter().map(|r| {
                let time = r.0.parse().unwrap();
                let record = r.1.parse().unwrap();
                (1..time)
                .filter(|t| t * (time - t) > record)
                    .count() as i32
            }).product();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let time: String = self.input.iter().map(|r| &*r.0).collect();
            let record: String = self.input.iter().map(|r| &*r.1).collect();
            let time: BigInt = time.parse().unwrap();
            let record: BigInt = record.parse().unwrap();

            println!("({} + ({}*{} - 4 * {})^0.5) / 2", &time, &time, &time, &record);
            println!("({} - ({}*{} - 4 * {})^0.5) / 2", &time, &time, &time, &record);
            let answer = "put the above into Spotlight and count the numbers between them";
            // 31,224,779.9109468335
            // 7,723,190.08905316653
            Box::new(answer)
        }
    }
}

mod day_07 {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt::Display;
    use crate::day::{Day, Solution};

    pub struct Day07;
    struct Input {
        input: Vec<(Vec<char>, i32)>,
    }

    impl Day for Day07 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let input = input.lines()
                .map(|line| {
                    let (hand, bid) = line.split_once(' ').unwrap();
                    (
                        hand.chars().collect(),
                        bid.parse().unwrap()
                    )
                }).collect();

            Box::new(Input {
                input
            })
        }
    }

    fn part_1_card_value(c: char) -> u8 {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _   => c as u8 - '0' as u8
        }
    }

    fn part_1_hand_strength(hand: &[u8]) -> i32 {
        let mut counts: Vec<i32> = hand.into_iter()
            .fold(HashMap::new(), |mut m, c| {
                *m.entry(*c).or_insert(0i32) += 1;
                m
            })
            .values()
            .copied()
            .collect();
        counts.sort();
        match &*counts {
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            [1, 1, 1, 1, 1] => 1,
            _ => unreachable!()
        }
    }

    fn part_2_card_value(c: char) -> u8 {
        match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _   => c as u8 - '0' as u8
        }
    }

    fn part_2_hand_strength(hand: &[u8]) -> i32 {
        let mut counts_without_jokers: Vec<i32> = hand.into_iter()
            .filter(|v| **v != 1)
            .fold(HashMap::new(), |mut m, c| {
                *m.entry(*c).or_insert(0i32) += 1;
                m
            })
            .values()
            .copied()
            .collect();
        counts_without_jokers.sort();
        match &*counts_without_jokers {
            // no jokers
            [5] => 7,
            [1, 4] => 6,
            [2, 3] => 5,
            [1, 1, 3] => 4,
            [1, 2, 2] => 3,
            [1, 1, 1, 2] => 2,
            [1, 1, 1, 1, 1] => 1,
            // 1 joker
            [4] => 7,
            [1, 3] => 6,
            [2, 2] => 5,
            [1, 1, 2] => 4,
            [1, 1, 1, 1] => 2,
            // 2 joker
            [3] => 7,
            [1, 2] => 6,
            [1, 1, 1] => 4,
            // 3 joker
            [2] => 7,
            [1, 1] => 6,
            // 4 joker
            [1] => 7,
            // 5 joker
            [] => 7,
            _ => unreachable!()
        }
    }

    fn cmp_hands(hand1: &(i32, Vec<u8>, i32), hand2: &(i32, Vec<u8>, i32)) -> Ordering {
        hand1.0.cmp(&hand2.0)
            .then_with(|| hand1.1.iter().zip(hand2.1.iter())
                .map(|(c1, c2)| c1.cmp(c2))
                .fold(Ordering::Equal, |a, b| a.then(b))
            )
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let mut hands: Vec<(i32, Vec<u8>, i32)> = self.input.iter()
                .map(|(hand, bid)| {
                    let hand: Vec<u8> = hand.iter().map(|c| part_1_card_value(*c)).collect();
                    (part_1_hand_strength(&*hand), hand, *bid)
                })
                .collect();

            hands.sort_by(|h1, h2| cmp_hands(h1, h2));

            let answer: i32 = hands.iter().zip(1..).map(|(h, r)| h.2 * r).sum();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let mut hands: Vec<(i32, Vec<u8>, i32)> = self.input.iter()
                .map(|(hand, bid)| {
                    let hand: Vec<u8> = hand.iter().map(|c| part_2_card_value(*c)).collect();
                    (part_2_hand_strength(&*hand), hand, *bid)
                })
                .collect();

            hands.sort_by(|h1, h2| cmp_hands(h1, h2));

            let answer: i32 = hands.iter().zip(1..).map(|(h, r)| h.2 * r).sum();

            Box::new(answer)
        }
    }
}

mod day_08 {
    use std::cmp::Ordering;
    use std::collections::HashMap;
    use std::fmt::Display;
    use crate::day::{Day, Solution};

    pub struct Day08;
    struct Input {
        input: Vec<String>,
    }

    impl Day for Day08 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let input = input.lines()
                .map(|s| s.to_owned())
                .collect();

            Box::new(Input {
                input
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer: i32 = 0;

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let answer: i32 = 0;

            Box::new(answer)
        }
    }
}


mod day_09 {
    use std::fmt::Display;
    use crate::day::{Day, Solution};

    pub struct Day09;
    struct Input {
        diff_seqss: Vec<Vec<Vec<i32>>>,
    }

    impl Day for Day09 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let input: Vec<Vec<i32>> = input.lines()
                .map(|s| s.split(' ').map(|s| s.parse().unwrap()).collect())
                .collect();

            let diff_seqss: Vec<Vec<Vec<i32>>> = input.iter()
                .map(|sequence|
                     std::iter::successors(
                        Some((*sequence).clone()),
                        |seq| {
                            let diff_seq: Vec<i32> = std::iter::zip(seq.iter(), seq.iter().skip(1))
                                .map(|(a, b)| b - a)
                                .collect();
                            if diff_seq.iter().all(|n| *n == 0) {
                                None
                            } else {
                                Some(diff_seq)
                            }
                        }
                    ).collect()
                ).collect();

            Box::new(Input {
                diff_seqss,
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer: i32 = self.diff_seqss.iter()
                .map(|diff_seqs| {
                    let last = diff_seqs.iter()
                        .rev()
                        .fold(0, |acc, seq| *seq.last().unwrap() + acc);
                    last
                })
                .sum();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let answer: i32 = self.diff_seqss.iter()
            .map(|diff_seqs| {
                let last = diff_seqs.iter()
                    .rev()
                    .fold(0, |acc, seq| *seq.first().unwrap() - acc);
                last
            })
            .sum();

            Box::new(answer)
        }
    }
}
