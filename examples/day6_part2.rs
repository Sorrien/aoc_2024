mod common;
use aoc_2024::day6::part2::solution_parallel;
use common::*;
use std::fs;

fn main() {
    run_solution(true, || {
        let input_string =
            fs::read_to_string("inputs/day6.txt").expect("failed to load input file");

        let sum = solution_parallel(input_string);
        sum.to_string()
    });
}
