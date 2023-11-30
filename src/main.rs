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

    let input_processing_start = Instant::now();
    let solution = day.process_input("1\n2\n");
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
    use crate::day::{Day, Solution};

    pub struct Day01;
    struct Input {
        input: Vec<i32>
    }

    impl Day for Day01 {
        fn process_input(&self, input: &str) -> Box<dyn Solution> {
            let input = vec![];

            Box::new(Input {
                input
            })
        }
    }

    impl Solution for Input {
        fn part_1(&self) -> Box<dyn Display> {
            let answer = "";

            Box::new(answer)
        }

        fn part_2(&self) -> Box<dyn Display> {
            let answer = "";

            Box::new(answer)
        }
    }
}