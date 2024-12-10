use std::{fs, time::Instant};

mod days;

use days::*;

fn main() {
    let days: Vec<Box<dyn AdventOfCodeDay>> = vec![
        Box::new(day1::Day),
        Box::new(day2::Day),
        Box::new(day3::Day),
        Box::new(day4::Day),
        Box::new(day5::Day),
        Box::new(day6::Day),
        Box::new(day7::Day),
        Box::new(day8::Day),
    ];
    let args = std::env::args().collect::<Vec<_>>();
    let [_, day_selection] = args.as_slice() else {
        eprintln!("Please select what day you want to run like: `cargo run -- <day>`");
        return;
    };

    let Some(day) = days
        .into_iter()
        .find(|d| &d.day().to_string() == day_selection)
    else {
        eprintln!("Counldn't find day: '{day_selection}'");
        return;
    };

    day.run_day();
}

pub trait AdventOfCodeDay {
    fn day(&self) -> u8;
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;

    fn run_day(&self) {
        let input = fs::read_to_string(format!("./inputs/day{}.txt", self.day()))
            .expect("Failed to read input file");

        let timer = Instant::now();
        let part1 = self.part1(&input);
        let p1_time = timer.elapsed();
        let timer = Instant::now();
        let part2 = self.part2(&input);
        let p2_time = timer.elapsed();

        println!("Rust Day {} results:", self.day());
        println!("  Part 1: {part1} in {}ms", p1_time.as_millis());
        println!("  Part 2: {part2} in {}ms", p2_time.as_millis());
    }
}
