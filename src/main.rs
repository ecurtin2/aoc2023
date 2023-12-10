// Import day1
mod day1;
mod day2;
use clap::Parser;
use std::fmt;
use std::fs;
use std::time::Instant;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum Part {
    Part1 = 1,
    Part2 = 2,
}

impl PartialEq<u32> for Part {
    fn eq(&self, other: &u32) -> bool {
        *other == (*self as u32)
    }
}
impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u32).fmt(f)
    }
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq)]
enum Day {
    Day1 = 1,
    Day2 = 2,
    Day3 = 3,
    Day4 = 4,
    Day5 = 5,
    Day6 = 6,
    Day7 = 7,
    Day8 = 8,
    Day9 = 9,
    Day10 = 10,
    Day11 = 11,
    Day12 = 12,
    Day13 = 13,
    Day14 = 14,
    Day15 = 15,
    Day16 = 16,
    Day17 = 17,
    Day18 = 18,
    Day19 = 19,
    Day20 = 20,
    Day21 = 21,
    Day22 = 22,
    Day23 = 23,
    Day24 = 24,
    Day25 = 25,
}
impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (*self as u32).fmt(f)
    }
}

impl PartialEq<u32> for Day {
    fn eq(&self, other: &u32) -> bool {
        *other == (*self as u32)
    }
}

pub fn load_file(path: String) -> Vec<String> {
    let input = fs::read_to_string(path).expect("Error reading file");
    input.split('\n').map(|s| s.to_string()).collect()
}

struct Result {
    day: Day,
    part: Part,
    result: Option<i32>,
    duration: std::time::Duration,
}

fn solve_part(day: &Day, part: &Part) -> Result {
    let start = Instant::now();
    let result: Option<i32> = match (day, part) {
        (Day::Day1, Part::Part1) => {
            Some(day1::solve_part1(load_file("data/day1.txt".to_string())) as i32)
        }
        (Day::Day1, Part::Part2) => {
            Some(day1::solve_part2(load_file("data/day1.txt".to_string())) as i32)
        }
        (Day::Day2, Part::Part1) => {
            Some(day2::solve_part1(load_file("data/day2.txt".to_string())) as i32)
        }
        (Day::Day2, Part::Part2) => {
            Some(day2::solve_part2(load_file("data/day2.txt".to_string())) as i32)
        }
        _ => None,
    };
    let duration = start.elapsed();
    Result {
        day: *day,
        part: *part,
        result,
        duration,
    }
}

// CLI stuff

#[derive(Parser, Debug)]
#[clap(version = "0.1", about = "Advent of Code 2023")]
struct Args {
    #[clap(short, long)]
    day: Option<u32>,
    #[clap(short, long)]
    part: Option<u32>,
}

fn main() {
    let args = Args::parse();
    let mut parts_to_solve = Vec::new();
    for day in Day::iter() {
        for part in Part::iter() {
            parts_to_solve.push((day, part));
        }
    }
    parts_to_solve = parts_to_solve
        .into_iter()
        .filter(|(d, _)| args.day.map_or(true, |day| *d == day))
        .filter(|(_, p)| args.part.map_or(true, |part| *p == part))
        .collect();

    // Display and Run Results
    println!(
        "|{:^5}|{:^6}|{:^10}| {:10}",
        "Day", "Part", "Result", "Duration"
    );
    let line = format!("|{:-<5}|{:-<6}|{:-<10}|{:-<12}", "", "", "", "");
    println!("{}", line);

    let mut last_day = Day::Day1;
    for (day, part) in parts_to_solve.iter() {
        let result = solve_part(day, part);
        if let Some(r) = result.result {
            if day != &last_day {
                println!("{}", line);
            }
            println!(
                "|{:^5}|{:^6}|{:^10}| {:?}",
                result.day, result.part, r, result.duration
            );
        }
        last_day = *day;
    }
}
