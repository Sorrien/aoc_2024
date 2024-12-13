use std::{env, fs, time::Instant};

use aoc_2024::{day1, day10, day11, day12, day13, day2, day3, day4, day5, day6, day7, day8, day9};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let day = &args[1];
    let part = &args[2];

    let day = day.parse::<u8>().unwrap();
    let part = part.parse::<u8>().unwrap();

    let measure = if let Some(measure) = args.get(3) {
        measure.parse::<bool>().unwrap()
    } else {
        false
    };

    let measure_count = if let Some(count) = args.get(4) {
        count.parse::<u64>().unwrap()
    } else {
        100
    } as usize;

    let file_name = if let Some(file_arg) = args.get(5) {
        format!("inputs/{}.txt", file_arg)
    } else {
        format!("inputs/day{}.txt", day)
    };
    let input = fs::read_to_string(file_name).expect("failed to load input file");

    let result = run_solution(&input, day, part);

    if measure {
        let mut timings: Vec<u128> = vec![];
        for _ in 0..measure_count {
            let start = Instant::now();
            run_solution(&input, day, part);
            timings.push((Instant::now() - start).as_nanos());
        }
        println!(
            "avg time in nanos: {}",
            timings.iter().sum::<u128>() as f64 / timings.iter().count() as f64
        );
    }

    println!("{}", result);
}

pub fn run_solution(input: &String, day: u8, part: u8) -> String {
    let input = input.clone();
    let result = match day {
        1 => match part {
            1 => day1::part1::solution(input).to_string(),
            2 => day1::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        2 => match part {
            1 => day2::part1::solution(input).to_string(),
            2 => day2::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        3 => match part {
            1 => day3::part1::solution(input).to_string(),
            2 => day3::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        4 => match part {
            1 => day4::part1::solution(input).to_string(),
            2 => day4::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        5 => match part {
            1 => day5::part1::solution(input).to_string(),
            2 => day5::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        6 => match part {
            1 => day6::part1::solution(input).to_string(),
            2 => day6::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        7 => match part {
            1 => day7::part1::solution(input).to_string(),
            2 => day7::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        8 => match part {
            1 => day8::part1::solution(input).to_string(),
            2 => day8::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        9 => match part {
            1 => day9::part1::solution(input).to_string(),
            2 => day9::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        10 => match part {
            1 => day10::part1::solution(input).to_string(),
            2 => day10::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        11 => match part {
            1 => day11::part1::solution(input).to_string(),
            2 => day11::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        12 => match part {
            1 => day12::part1::solution(input).to_string(),
            2 => day12::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        13 => match part {
            1 => day13::part1::solution(input).to_string(),
            2 => day13::part2::solution(input).to_string(),
            _ => panic!("Part not recognized"),
        },
        _ => panic!("Haven't done that yet I guess"),
    };
    result
}
