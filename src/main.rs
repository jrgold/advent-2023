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
    let day_number = 17;

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
        11 => &day_11::Day11{},
        12 => &day_12::Day12{},
        13 => &day_13::Day13{},
        14 => &day_14::Day14{},
        17 => &day_17::Day17{},
        _ => unimplemented!(),
    };

    let input_filename = format!("input/day_{:02}.txt", day_number);
    // let input_filename = format!("input/day_{:02}_sample_2.txt", day_number);

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
    use std::collections::HashMap;
    use std::fmt::Display;
    use regex::Regex;
    use crate::day::{Day, Solution};

    #[derive(Debug)]
    enum Dir { L, R, }
    impl Dir {
        fn go<T> (&self, choices: (T, T)) -> T {
            match self {
                Dir::L => choices.0,
                Dir::R => choices.1,
            }
        }
        fn from(c: char) -> Self {
            match c {
                'L' => Dir::L,
                'R' => Dir::R,
                _   => unreachable!(),
            }
        }
    }

    pub struct Day08;
    struct Input {
        movements: Vec<Dir>,
        graph: HashMap<i32, (i32, i32)>,
    }

    fn to_id(name: &str) -> i32 {
        let bytes = name.as_bytes();
        (bytes[0] as i32 - 'A' as i32) * 26 * 26 +
            (bytes[1] as i32 - 'A' as i32) * 26 +
            (bytes[2] as i32 - 'A' as i32)
    }

    #[allow(unused)]
    fn to_name(id: i32) -> String {
        let mut s = String::new();
        s.push((id / 26 / 26 + 'A' as i32) as u8 as char);
        s.push(((id / 26) % 26 + 'A' as i32) as u8 as char);
        s.push((id % 26 + 'A' as i32) as u8 as char);
        s
    }

    impl Day for Day08 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let mut lines = input.lines();

            let movements = lines.next().unwrap().chars().map(Dir::from).collect();

            lines.next();

            let line_regex = Regex::new("^(...) = \\((...), (...)\\)$").unwrap();

            let graph = lines
                .map(|line| {
                    let captures = line_regex.captures(line).unwrap();
                    (
                        to_id(captures.get(1).unwrap().as_str()),
                        (
                            to_id(captures.get(2).unwrap().as_str()),
                            to_id(captures.get(3).unwrap().as_str())
                        )
                    )
                })
                .collect();

            Box::new(Input {
                movements,
                graph,
            })
        }
    }

    impl Input {
        fn period(&self, start: i32) -> (Vec<u64>, u64, Vec<u64>) {
            let mut zs: Vec<(u64, i32)> = vec![];
            let mut loc = start;
            let mut moves = 0u64;
            for dir in self.movements.iter().cycle() {
                loc = dir.go(*self.graph.get(&loc).unwrap());
                moves += 1;
                if loc % 26 == 25 {
                    if let Some((period_start, _)) = zs.iter().filter(|z| z.1 == loc).next() {
                        let period_end = moves;
                        return (
                            zs.iter().copied()
                                .take_while(|(p, _)| p < period_start)
                                .map(|(p, _)| p)
                                .collect(),
                            period_end - period_start,
                            zs.iter().copied()
                                .skip_while(|(p, _)| p < period_start)
                                .map(|(p, _)| p)
                                .collect(),
                        );
                    } else {
                        zs.push((moves, loc));
                    }
                }
            }
            unreachable!()
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let mut moves = 0;
            let mut loc = to_id("AAA");

            for dir in self.movements.iter().cycle() {
                loc = dir.go(*self.graph.get(&loc).unwrap());
                moves += 1;
                if loc == to_id("ZZZ") {
                    break;
                }
            }

            let answer: i32 = moves;

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let starters: Vec<i32> = self.graph.keys().copied().filter(|id| *id % 26 == 0).collect();

            // all paths are periodic through a single Z node, with periods equal
            // to the distance from the respective A node. This means we can just
            // lowest-common-multiple the periods. This solution is more general,
            // working for multi-end cycles and differently-sized/non-cyclical
            // lead-in sequences.

            let periods: Vec<(Vec<u64>, u64, Vec<u64>)> = starters.into_iter()
                .map(|id| self.period(id))
                .collect();

            let answer = (0..)
                .flat_map(|n| {
                    let n = n;
                    let period = periods[0].1;
                    periods[0].2.iter()
                        .map(move |x| n * period + x)
                })
                .filter(|x| {
                    periods[1..].iter().all(|ghost| {
                        ghost.0.contains(x) || ghost.2.iter().any(|y| (x - y) % ghost.1 == 0)
                    })
                })
                .next().unwrap();

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

mod day_11 {
    use std::collections::{HashMap, HashSet};
    use std::fmt::Display;
    use crate::day::{Day, Solution};

    pub struct Day11;
    struct Input {
        galaxy_coords: HashSet<(i32, i32)>,
    }

    fn expand_gaps(coords: &HashSet<i32>, extra: i32) -> HashMap<i32, i32> {
        let mut running_extra = 0;
        let mut adjusted: HashMap<i32, i32> = HashMap::new();
        for x in (*coords.iter().min().unwrap())..=*coords.iter().max().unwrap() {
            if !coords.contains(&x) {
                running_extra += extra;
            }
            adjusted.insert(x, x + running_extra);
        }
        adjusted
    }

    fn expand(coords: &HashSet<(i32, i32)>, extra: i32) -> Vec<(i32, i32)> {
        let xs: HashSet<i32> = coords.iter().map(|(x, _)| *x).collect();
        let ys: HashSet<i32> = coords.iter().map(|(_, y)| *y).collect();

        let adjusted_xs = expand_gaps(&xs, extra);
        let adjusted_ys = expand_gaps(&ys, extra);

        coords.into_iter()
            .map(|(x, y)| (
                *adjusted_xs.get(&x).unwrap() as i32,
                *adjusted_ys.get(&y).unwrap() as i32
            ))
            .collect()
    }

    impl Day for Day11 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let galaxy_coords: HashSet<(i32, i32)> = input.lines().enumerate()
                .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x as i32, y as i32), c)))
                .filter_map(|(p, c)| if c == '#' { Some(p) } else { None })
                .collect();

            Box::new(Input {
                galaxy_coords
            })
        }
    }

    fn sum_distances(coords: &[(i32 ,i32)]) -> i64 {
        let mut answer: i64 = 0;

        for i in 0..coords.len()-1 {
            for j in i+1..coords.len() {
                answer += (coords[i].0 - coords[j].0).abs() as i64 +
                    (coords[i].1 - coords[j].1).abs() as i64;
            }
        }

        answer
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let expanded = expand(&self.galaxy_coords, 1);
            let answer = sum_distances(&expanded);

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let expanded = expand(&self.galaxy_coords, 999999);
            let answer = sum_distances(&expanded);

            Box::new(answer)
        }
    }
}

mod day_12 {
    use std::collections::HashMap;
    use std::fmt::Display;
    use crate::day::{Day, Solution};
    use crate::day_12::Progress::{Either, MustBeDamaged, MustBeOperational};
    use crate::day_12::Spring::Unknown;

    #[derive(Debug, Eq, PartialEq, Clone, Copy)]
    enum Spring {
        Operational,
        Damaged,
        Unknown,
    }

    impl Spring {
        fn from(c: char) -> Spring {
            match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Unknown,
                _   => panic!("invalid spring character: {}", c),
            }
        }
    }

    pub struct Day12;
    struct Input {
        input: Vec<(Vec<Spring>, Vec<i32>)>,
    }

    impl Day for Day12 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let input = input.lines()
                .map(|line| {
                    let (springs, groups) = line.split_once(' ').unwrap();
                    (
                        springs.chars().map(|c| Spring::from(c)).collect(),
                        groups.split(',').map(|s| s.parse().unwrap()).collect()
                    )
                }).collect();

            Box::new(Input {
                input
            })
        }
    }

    #[allow(unused)]
    fn show(r: &[Spring]) -> String {
        r.iter().map(|s| match s {
            Spring::Operational => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        }).collect()
    }

    #[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
    enum Progress {
        MustBeDamaged,
        MustBeOperational,
        Either,
    }

    fn variations_2(running: &mut Vec<Spring>, cache: &mut HashMap<(usize, Progress, Vec<i32>), u64>, index: usize, next: Progress, groups: &mut [i32]) -> u64 {
        let g_clone = groups.to_owned();
        if let Some(cached) = cache.get(&(index, next, g_clone)) {
            return *cached;
        }
        let g_clone = groups.to_owned();

        // println!("{} {:?} {:?}", index, next, groups);
        if index == running.len() {
            if groups.is_empty() {
                return 1;
            } else {
                return 0;
            }
        }

        let v = match (running[index], next) {
            (Spring::Damaged, MustBeDamaged | Either) =>
                if groups[0] == 1 {
                    // end of group
                    variations_2(running, cache, index + 1, MustBeOperational, &mut groups[1..])
                } else {
                    // group still ongoing
                    groups[0] -= 1;
                    let v = variations_2(running, cache, index + 1, MustBeDamaged, groups);
                    groups[0] += 1;
                    v
                },
            (Spring::Damaged, MustBeOperational) => 0,
            (Spring::Operational, MustBeDamaged) => 0,
            (Spring::Operational, MustBeOperational | Either) =>
                variations_2(running, cache, index + 1, if groups.is_empty() { MustBeOperational } else { Either }, groups),
            (Spring::Unknown, MustBeDamaged) =>
                if groups[0] == 1 {
                    // end of group
                    variations_2(running, cache, index + 1, MustBeOperational, &mut groups[1..])
                } else {
                    // group still ongoing
                    groups[0] -= 1;
                    let v = variations_2(running, cache, index + 1, MustBeDamaged, groups);
                    groups[0] += 1;
                    v
                },
            (Spring::Unknown, MustBeOperational) =>
                variations_2(running, cache, index + 1, if groups.is_empty() { MustBeOperational } else { Either }, groups),
            (Spring::Unknown, Either) => {
                let assume_damaged = if groups[0] == 1 {
                        // end of group
                        variations_2(running, cache, index + 1, MustBeOperational, &mut groups[1..])
                    } else {
                        // group still ongoing
                        groups[0] -= 1;
                        let v = variations_2(running, cache, index + 1, MustBeDamaged, groups);
                        groups[0] += 1;
                        v
                    };
                let assume_operational = variations_2(running, cache, index + 1, if groups.is_empty() { MustBeOperational } else { Either }, groups);
                assume_damaged + assume_operational
            }
        };

        cache.insert((index, next, g_clone), v);
        v
    }

    // fn variations(running: &[Spring], index: usize, in_group: bool, groups: &[i32]) -> u64 {
    //     if index >= running.len() {
    //         if groups.is_empty() || (groups.len() == 1 && groups[0] == 0) {
    //             return 1;
    //         } else {
    //             return 0;
    //         }
    //     }
    //
    //     match running[index] {
    //         Spring::Unknown => {
    //             0
    //         },
    //         Spring::Damaged => {
    //             if in_group {
    //                 if groups[0]
    //             }
    //         },
    //         Spring::Operational => {
    //             if in_group {
    //                 if groups[0] == 0 {
    //                     // reached the end of a group when it was done
    //                     variations(running, index + 1, false, &groups[1..])
    //                 } else {
    //                     // reached the end of a group before the group was long enough
    //                     0
    //                 }
    //             } else {
    //                 variations(running, index + 1, false, groups)
    //             }
    //         }
    //     }
    // }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer: u64 = self.input.iter()
                .map(|(s, g)| {
                    let mut s = s.clone();
                    let mut cache = HashMap::new();
                    variations_2(&mut s, &mut cache, 0, Either, &mut g.clone())
                }).sum();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let answer: u64 = self.input.iter()
                .map(|(s, g)| {
                    let mut unfolded_springs = s.clone();
                    unfolded_springs.push(Unknown);
                    unfolded_springs.extend_from_slice(&s);
                    unfolded_springs.push(Unknown);
                    unfolded_springs.extend_from_slice(&s);
                    unfolded_springs.push(Unknown);
                    unfolded_springs.extend_from_slice(&s);
                    unfolded_springs.push(Unknown);
                    unfolded_springs.extend_from_slice(&s);

                    let mut unfolded_groups = g.repeat(5);

                    let mut cache = HashMap::new();

                    variations_2(&mut unfolded_springs, &mut cache, 0, Either, &mut unfolded_groups)
                }).sum();

            Box::new(answer)
        }
    }
}

mod day_13 {
    use std::collections::HashSet;
    use std::fmt::Display;
    use crate::day::{Day, Solution};

    pub struct Day13;

    struct Pattern {
        pattern: HashSet<(i32, i32)>,
        x_max: i32,
        y_max: i32,
    }
    struct Input {
        input: Vec<Pattern>,
    }

    impl Pattern {
        fn vertical_reflection(&self, smudges: i32) -> Option<i32> {
            for x in 0..self.x_max {
                let mut l = x;
                let mut r = l + 1;
                let mirror_size = l.min(self.x_max - l - 1) + 1;
                let mut difference = 0;
                for _ in 0..mirror_size {
                    let left_differences = self.pattern.iter()
                        .filter(|(x, _)| *x == l)
                        .filter(|(_, y)| !self.pattern.contains(&(r, *y)))
                        .count() as i32;
                    let right_differences = self.pattern.iter()
                        .filter(|(x, _)| *x == r)
                        .filter(|(_, y)| !self.pattern.contains(&(l, *y)))
                        .count() as i32;

                    difference += left_differences + right_differences;

                    l -= 1;
                    r += 1;
                }

                if difference == smudges {
                    return Some(x + 1);
                }

            }

            None
        }

        fn horizontal_reflection(&self, smudges: i32) -> Option<i32> {
            for y in 0..self.y_max {
                let mut t = y;
                let mut b = t + 1;
                let mirror_size = t.min(self.y_max - t - 1) + 1;
                let mut difference = 0;
                for _ in 0..mirror_size {
                    let top_differences = self.pattern.iter()
                        .filter(|(_, y)| *y == t)
                        .filter(|(x, _)| !self.pattern.contains(&(*x, b)))
                        .count() as i32;
                    let bottom_differences = self.pattern.iter()
                        .filter(|(_, y)| *y == b)
                        .filter(|(x, _)| !self.pattern.contains(&(*x, t)))
                        .count() as i32;

                    difference += top_differences + bottom_differences;

                    t -= 1;
                    b += 1;
                }

                if difference == smudges {
                    return Some(y + 1);
                }
            }

            None
        }
    }

    impl Day for Day13 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let input = input.split("\n\n").map(|pattern| {
                let pattern: HashSet<(i32, i32)> = pattern.lines()
                    .enumerate()
                    .flat_map(|(y, line)|
                        line.chars()
                            .enumerate()
                            .filter(|(_, c)| *c == '#')
                            .map(move |(x, _)| (x as i32, y as i32))
                    ).collect();
                let x_max = pattern.iter().map(|p| p.0).max().unwrap();
                let y_max = pattern.iter().map(|p| p.1).max().unwrap();
                Pattern {
                    pattern,
                    x_max,
                    y_max,
                }
            }).collect();

            Box::new(Input {
                input
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer: i32 = self.input.iter().map(|p| {
                100 * p.horizontal_reflection(0).unwrap_or(0) + p.vertical_reflection(0).unwrap_or(0)
            }).sum();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let answer: i32 = self.input.iter().map(|p| {
                100 * p.horizontal_reflection(1).unwrap_or(0) + p.vertical_reflection(1).unwrap_or(0)
            }).sum();

            Box::new(answer)
        }
    }
}

mod day_14 {
    use std::collections::HashMap;
    use std::fmt::Display;
    use crate::day::{Day, Solution};
    use crate::day_14::Rock::Mobile;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    enum Rock { Fixed, Mobile, }

    impl TryFrom<char> for Rock {
        type Error = ();
        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                'O' => Ok(Rock::Mobile),
                '#' => Ok(Rock::Fixed),
                _   => Err(())
            }
        }
    }

    pub struct Day14;
    #[derive(Clone)]
    struct Input {
        platform: HashMap<(i32, i32), Rock>,
        x_max: i32,
        y_max: i32,
    }

    impl Input {
        fn load(&self) -> i32 {
            self.platform.iter()
                .filter(|(_, r)| **r == Mobile)
                .map(|(p, _)| self.y_max - p.1 + 1)
                .sum()
        }

        fn north(&mut self) {
            let mut mobiles: Vec<(i32, i32)> = self.platform.iter()
                .filter(|(_, r)| **r == Mobile)
                .map(|(p, _)| p)
                .copied()
                .collect();
            mobiles.sort_by_key(|p| p.1);

            mobiles.iter().for_each(|p| {
                self.platform.remove(p);
                let mut dest = *p;
                for y in (0..p.1).rev() {
                    if self.platform.contains_key(&(p.0, y)) {
                        break;
                    }
                    dest.1 = y;
                }
                self.platform.insert(dest, Mobile);
            })
        }

        fn south(&mut self) {
            let mut mobiles: Vec<(i32, i32)> = self.platform.iter()
                .filter(|(_, r)| **r == Mobile)
                .map(|(p, _)| p)
                .copied()
                .collect();
            mobiles.sort_by_key(|p| p.1);
            mobiles.reverse();

            mobiles.iter().for_each(|p| {
                self.platform.remove(p);
                let mut dest = *p;
                for y in p.1+1..=self.y_max {
                    if self.platform.contains_key(&(p.0, y)) {
                        break;
                    }
                    dest.1 = y;
                }
                self.platform.insert(dest, Mobile);
            })
        }

        fn west(&mut self) {
            let mut mobiles: Vec<(i32, i32)> = self.platform.iter()
                .filter(|(_, r)| **r == Mobile)
                .map(|(p, _)| p)
                .copied()
                .collect();
            mobiles.sort_by_key(|p| p.0);

            mobiles.iter().for_each(|p| {
                self.platform.remove(p);
                let mut dest = *p;
                for x in (0..p.0).rev() {
                    if self.platform.contains_key(&(x, p.1)) {
                        break;
                    }
                    dest.0 = x;
                }
                self.platform.insert(dest, Mobile);
            })
        }

        fn east(&mut self) {
            let mut mobiles: Vec<(i32, i32)> = self.platform.iter()
                .filter(|(_, r)| **r == Mobile)
                .map(|(p, _)| p)
                .copied()
                .collect();
            mobiles.sort_by_key(|p| p.0);
            mobiles.reverse();

            mobiles.iter().for_each(|p| {
                self.platform.remove(p);
                let mut dest = *p;
                for x in p.0+1..=self.x_max {
                    if self.platform.contains_key(&(x, p.1)) {
                        break;
                    }
                    dest.0 = x;
                }
                self.platform.insert(dest, Mobile);
            })
        }

        fn spin(&mut self) {
            self.north();
            self.west();
            self.south();
            self.east();
        }
    }

    impl Day for Day14 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let platform: HashMap<(i32, i32), Rock> = input.lines()
                .enumerate()
                .flat_map(|(y, line)|
                    line.chars()
                        .enumerate()
                        .filter_map(move |(x, c)| c.try_into().map(|r: Rock| ((x as i32, y as i32), r)).ok())
                ).collect();
            let x_max = platform.keys().map(|p| p.0).max().unwrap();
            let y_max = platform.keys().map(|p| p.1).max().unwrap();
            Box::new(Input {
                platform,
                x_max,
                y_max,
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let mut platform = self.clone();
            platform.north();
            let answer = platform.load();

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let mut platform = self.clone();

            let mut cycles: Vec<Input> = vec![];
            cycles.push(platform.clone());
            let mut cycle_start = 0;
            let mut cycle_end = 0;

            'cycle_found: for cycle in 1..=1000 {
                platform.spin();

                for i in 0..cycle {
                    if platform.platform == cycles[i].platform {
                        cycle_start = i;
                        cycle_end = cycle;
                        break 'cycle_found;
                    }
                }

                cycles.push(platform.clone())
            }

            let cycles_left = (1_000_000_000 - cycle_end) % (cycle_end - cycle_start);
            for _ in 0..cycles_left {
                platform.spin();
            }

            let answer = platform.load();

            Box::new(answer)
        }
    }
}

mod day_17 {
    use std::cmp::Reverse;
    use std::collections::{HashMap, HashSet};
    use std::fmt::Display;
    use std::hash::Hash;
    use priority_queue::PriorityQueue;
    use crate::day::{Day, Solution};
    use crate::day_17::Dir::{N, S, E, W};

    pub struct Day17;
    #[derive(Clone)]
    struct Input {
        blocks: HashMap<(i32, i32), u8>,
        x_max: i32,
        y_max: i32,
    }

    impl Day for Day17 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let blocks: HashMap<(i32, i32), u8> = input.lines()
                .enumerate()
                .flat_map(|(y, line)|
                    line.chars()
                        .enumerate()
                        .map(move |(x, c)| ((x as i32, y as i32), c as u8 - '0' as u8))
                ).collect();
            let x_max = blocks.keys().map(|(x, _)| *x).max().unwrap();
            let y_max = blocks.keys().map(|(_, y)| *y).max().unwrap();
            Box::new(Input {
                blocks,
                x_max,
                y_max,
            })
        }
    }

    #[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
    enum Dir { N, S, E, W, }
    impl Dir {
        fn movement(&self) -> (i32, i32) {
            match self {
                N => (0, -1),
                S => (0, 1),
                E => (1, 0),
                W => (-1, 0),
            }
        }

        fn regular_crucible_movements(&self, straight_distance: i32) -> Vec<Dir> {
            [N, S, E, W].into_iter()
                .filter(|&d| match (self, d) {
                    (N, S) => false,
                    (S, N) => false,
                    (W, E) => false,
                    (E, W) => false,
                    _      => true,
                })
                .filter(|&d| d != *self || straight_distance < 3)
                .collect()
        }

        fn ultra_crucible_movements(&self, straight_distance: i32) -> Vec<Dir> {
            if straight_distance < 4 {
                return Vec::from(&[*self]);
            }

            [N, S, E, W].into_iter()
                .filter(|&d| match (self, d) {
                    (N, S) => false,
                    (S, N) => false,
                    (W, E) => false,
                    (E, W) => false,
                    _      => true,
                })
                .filter(|&d| d != *self || straight_distance < 10)
                .collect()
        }
    }

    #[derive(Debug, Hash, PartialEq, Eq)]
    struct Position {
        pos: (i32, i32),
        dir: Dir,
        straight_distance: i32,
    }

    fn least_path<F, G>(input: &Input, allowable_movements: F, can_stop: G, start: (i32, i32), end: (i32, i32), starting_direction: Dir) -> i32
        where F: Fn(&Dir, i32) -> Vec<Dir>,
              G: Fn(&Position) -> bool
    {
        let mut to_visit: PriorityQueue<Position, Reverse<i32>> = PriorityQueue::new();
        let mut visited: HashSet<Position> = HashSet::new();

        to_visit.push(Position { pos: start, dir: starting_direction, straight_distance: 0 }, Reverse(0));

        loop {
            let (position, distance) = to_visit.pop().unwrap();

            if position.pos == end && can_stop(&position) {
                return distance.0;
            }

            let unvisited_moves: Vec<Position> = allowable_movements(&position.dir, position.straight_distance).into_iter()
                .map(|d| {
                    let movement = d.movement();
                    Position {
                        pos: (position.pos.0 + movement.0, position.pos.1 + movement.1),
                        dir: d,
                        straight_distance: if d == position.dir { position.straight_distance + 1 } else { 1 },
                    }
                })
                .filter(|new_pos| new_pos.pos.0 >= 0 && new_pos.pos.0 <= input.x_max && new_pos.pos.1 >= 0 && new_pos.pos.1 <= input.y_max)
                .filter(|new_pos| !visited.contains(new_pos))
                .collect();

            for unvisited in unvisited_moves {
                let tentative_distance = distance.0 + *input.blocks.get(&unvisited.pos).unwrap() as i32;
                match to_visit.get(&unvisited) {
                    Some((_, existing_tentative_distance)) => if tentative_distance < existing_tentative_distance.0 {
                        to_visit.change_priority(&unvisited, Reverse(tentative_distance));
                    },
                    None => {
                        to_visit.push(unvisited, Reverse(tentative_distance));
                    },
                }
            }

            visited.insert(position);
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer = least_path(
                &self,
                &Dir::regular_crucible_movements,
                |_| true,
                (0, 0),
                (self.x_max, self.y_max),
                S
            );

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let answer_east = least_path(
                &self,
                &Dir::ultra_crucible_movements,
                |p| p.straight_distance >= 4,
                (0, 0),
                (self.x_max, self.y_max),
                E
            );

            let answer_south = least_path(
                &self,
                &Dir::ultra_crucible_movements,
                |p| p.straight_distance >= 4,
                (0, 0),
                (self.x_max, self.y_max),
                S
            );

            Box::new(std::cmp::min(answer_east, answer_south))
        }
    }
}
