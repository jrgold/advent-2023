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
    let day_number = 1;

    let day: &dyn Day = match day_number {
        1 => &day_01::Day01{},
        _ => unreachable!(),
    };

    let input_filename = format!("input/day_{:02}.txt", day_number);
    // let input_filename = "day_01_sample.txt";

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